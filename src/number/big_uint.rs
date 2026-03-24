// Copyright 2023, 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a fixed-size big unsigned integer implementation
//! with customizable bit-widths and associated methods.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::mem::size_of;
use std::ptr::copy_nonoverlapping;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, RangeBounds };
use std::marker::{ Send, Sync };

use crate::number::{ SmallUInt, LongerUnion, SharedValues, SharedArrays, NumberErr };


/// A trait alias for the collection of traits required by the internal
/// storage type `T` in `BigUInt<T, N>`.
///
/// # Note
/// These trait requirements are subject to change in future versions to
/// accommodate updates to the internal implementation.
pub trait TraitsBigUInt<T>: SmallUInt + Copy + Clone + Display + Debug + ToString
                        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
                        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
                        + Rem<Output=T> + RemAssign
                        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
                        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
                        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
                        + PartialEq + PartialOrd + Send + Sync + 'static
{}

impl<T> TraitsBigUInt<T> for T
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd + Send + Sync + 'static
{}

// unsafe impl<T> Send for T where T: Send {}

// pub trait TraitsBigUInt<T>: SmallUInt + Copy + Clone + Display + Debug + ToString
//         + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
//         + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
//         + Rem<Output=T> + RemAssign
//         + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
//         + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
//         + BitXor<Output=T> + BitXorAssign + Not<Output=T>
//         + PartialEq + PartialOrd = {};

macro_rules! define_biguint_alias {
    ($name:ident, $inner:ty, $len:expr, $bits:expr) => {
        #[doc = concat!(
            stringify!($bits), "-bit unsigned integer implemented as `BigUInt<", 
            stringify!($inner), ", ", stringify!($len), ">` (comprising ", 
            stringify!($len), " x `", stringify!($inner), "`)."
        )]
        #[allow(non_camel_case_types)]
        pub type $name = BigUInt<$inner, $len>;
    };
}

// --- u128 Variants ---
define_biguint_alias!(U256_with_u128, u128, 2, 256);
define_biguint_alias!(U384_with_u128, u128, 3, 384);
define_biguint_alias!(U512_with_u128, u128, 4, 512);
define_biguint_alias!(U768_with_u128, u128, 6, 768);
define_biguint_alias!(U1024_with_u128, u128, 8, 1024);
define_biguint_alias!(U2048_with_u128, u128, 16, 2048);
define_biguint_alias!(U3072_with_u128, u128, 24, 3072);
define_biguint_alias!(U4096_with_u128, u128, 32, 4096);
define_biguint_alias!(U5120_with_u128, u128, 40, 5120);
define_biguint_alias!(U6144_with_u128, u128, 48, 6144);
define_biguint_alias!(U7168_with_u128, u128, 56, 7168);
define_biguint_alias!(U8192_with_u128, u128, 64, 8192);
define_biguint_alias!(U16384_with_u128, u128, 128, 16384);

// --- u64 Variants ---
define_biguint_alias!(U256_with_u64, u64, 4, 256);
define_biguint_alias!(U384_with_u64, u64, 6, 384);
define_biguint_alias!(U512_with_u64, u64, 8, 512);
define_biguint_alias!(U768_with_u64, u64, 12, 768);
define_biguint_alias!(U1024_with_u64, u64, 16, 1024);
define_biguint_alias!(U2048_with_u64, u64, 32, 2048);
define_biguint_alias!(U3072_with_u64, u64, 48, 3072);
define_biguint_alias!(U4096_with_u64, u64, 64, 4096);
define_biguint_alias!(U5120_with_u64, u64, 80, 5120);
define_biguint_alias!(U6144_with_u64, u64, 96, 6144);
define_biguint_alias!(U7168_with_u64, u64, 112, 7168);
define_biguint_alias!(U8192_with_u64, u64, 128, 8192);
define_biguint_alias!(U16384_with_u64, u64, 256, 16384);

// --- u32 Variants ---
define_biguint_alias!(U256_with_u32, u32, 8, 256);
define_biguint_alias!(U384_with_u32, u32, 12, 384);
define_biguint_alias!(U512_with_u32, u32, 16, 512);
define_biguint_alias!(U768_with_u32, u32, 24, 768);
define_biguint_alias!(U1024_with_u32, u32, 32, 1024);
define_biguint_alias!(U2048_with_u32, u32, 64, 2048);
define_biguint_alias!(U3072_with_u32, u32, 96, 3072);
define_biguint_alias!(U4096_with_u32, u32, 128, 4096);
define_biguint_alias!(U5120_with_u32, u32, 160, 5120);
define_biguint_alias!(U6144_with_u32, u32, 192, 6144);
define_biguint_alias!(U7168_with_u32, u32, 224, 7168);
define_biguint_alias!(U8192_with_u32, u32, 256, 8192);
define_biguint_alias!(U16384_with_u32, u32, 512, 16384);

// --- u16 Variants ---
define_biguint_alias!(U256_with_u16, u16, 16, 256);
define_biguint_alias!(U384_with_u16, u16, 24, 384);
define_biguint_alias!(U512_with_u16, u16, 32, 512);
define_biguint_alias!(U768_with_u16, u16, 48, 768);
define_biguint_alias!(U1024_with_u16, u16, 64, 1024);
define_biguint_alias!(U2048_with_u16, u16, 128, 2048);
define_biguint_alias!(U3072_with_u16, u16, 192, 3072);
define_biguint_alias!(U4096_with_u16, u16, 256, 4096);
define_biguint_alias!(U5120_with_u16, u16, 320, 5120);
define_biguint_alias!(U6144_with_u16, u16, 384, 6144);
define_biguint_alias!(U7168_with_u16, u16, 448, 7168);
define_biguint_alias!(U8192_with_u16, u16, 512, 8192);
define_biguint_alias!(U16384_with_u16, u16, 1024, 16384);

// --- u8 Variants ---
define_biguint_alias!(U256_with_u8, u8, 32, 256);
define_biguint_alias!(U384_with_u8, u8, 48, 384);
define_biguint_alias!(U512_with_u8, u8, 64, 512);
define_biguint_alias!(U768_with_u8, u8, 96, 768);
define_biguint_alias!(U1024_with_u8, u8, 128, 1024);
define_biguint_alias!(U2048_with_u8, u8, 256, 2048);
define_biguint_alias!(U3072_with_u8, u8, 384, 3072);
define_biguint_alias!(U4096_with_u8, u8, 512, 4096);
define_biguint_alias!(U5120_with_u8, u8, 640, 5120);
define_biguint_alias!(U6144_with_u8, u8, 768, 6144);
define_biguint_alias!(U7168_with_u8, u8, 896, 7168);
define_biguint_alias!(U8192_with_u8, u8, 1024, 8192);
define_biguint_alias!(U16384_with_u8, u8, 2048, 16384);


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

macro_rules! biguint_overflowing_calc
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        let current_overflow = $func(&mut res, $rhs);
        return (res, current_overflow);
    }
    // biguint_overflowing_calc!(self, Self::overflowing_add_assign_uint, rhs);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // let current_overflow = res.overflowing_add_assign_uint(rhs);
    // (res, current_overflow)
}

macro_rules! biguint_overflowing_calc_div
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (quotient, _) = $func($me, $rhs);
        return (quotient, false);
    }
    // biguint_overflowing_calc_div!(self, Self::divide_fully_uint, rhs);
    //
    // let (quotient, _) = self.divide_fully_uint(rhs);
    // let overflow = quotient.is_overflow();
    // (quotient, overflow)
}

macro_rules! biguint_overflowing_calc_rem
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let (_, remainder) = $func($me, $rhs);
        return (remainder, false);
    }
    // biguint_overflowing_calc_rem!(self, Self::divide_fully_uint, rhs);
    //
    // let (_, remainder) = self.divide_fully_uint(rhs);
    // (remainder, false)
}

macro_rules! biguint_overflowing_calc_assign
{
    ($me:expr, $func:expr, $rhs:expr) => {
        let flags = $me.get_all_flags();
        $me.reset_all_flags();
        $func($me, $rhs);
        let current_overflow = $me.is_overflow();
        $me.set_flag_bit(flags);
        return current_overflow;
    }
    // biguint_overflowing_calc_assign!(self, Self::wrapping_add_assign_uint, rhs);
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

    ($me:expr, $func:expr, $rhs:expr, $modulus:expr) => {
        let mut res = Self::from_array($me.get_number().clone());
        $func(&mut res, $rhs, $modulus);
        return res;
    }
    // biguint_calc_assign_to_calc!(self, Self::modular_add_assign_uint, rhs, modulus);
    //
    // let mut res = Self::from_array(self.get_number().clone());
    // res.modular_add_assign_uint(rhs, modulus);
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

macro_rules! biguint_calc_to_calc_assign
{
    ($me:expr, $func:expr) => {
        let res = $func($me);
        $me.set_number(res.get_number());
        $me.set_all_flags(res.get_all_flags());
    };
    // biguint_calc_to_calc_assign!(self, Self::ilog2);
    //
    // let res = self.ilog2();
    // self.set_number(res.get_number());
    // self.set_all_flags(res.get_all_flags());

    ($me:expr, $func:expr, $rhs:expr) => {
        let res = $func($me, $rhs);
        $me.set_number(res.get_number());
        $me.set_all_flags(res.get_all_flags());
    };
    // biguint_calc_to_calc_assign!(self, Self::wrapping_div_uint, rhs);
    //
    // let res = self.wrapping_div_uint(rhs);
    // self.set_number(res.get_number());
    // self.set_all_flags(res.get_all_flags());

    ($me:expr, $func:expr, $rhs:expr, $modulus:expr) => {
        let res = $func($me, $rhs, $modulus);
        $me.set_number(res.get_number());
        $me.set_all_flags(res.get_all_flags());
    };
    // biguint_calc_to_calc_assign!(self, Self::modular_gcd, rhs, modulus);
    //
    // let res = self.modular_gcd(&rhs, &modulus);
    // self.set_number(res.get_number());
    // self.set_all_flags(res.get_all_flags());
}
pub(super) use biguint_calc_to_calc_assign;

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


//////////////////////////////////////////
/// # A Generic Fixed-Size Big Unsigned Integer
///
/// `BigUInt<T, const N: usize>` is a generic structure providing high-precision 
/// arithmetic with a fixed memory footprint.
///
/// # Generic Parameters
/// * `T`: The underlying primitive unsigned integer type
///   (`u8`, `u16`, `u32`, `u64`, `u128`, or `usize`).
/// * `N`: The number of elements of type `T`.
///   The total bit-width is calculated as `size_of::<T>() * N * 8`.
/// 
/// # Internal Representation
/// * **Endianness**: Data is stored in **little-endian** order.
/// * **Storage**: It consists of an internal array `[T; N]`
///   and a `u8` flag field.
/// * **Flags**: Tracks states such as `OVERFLOW`, `UNDERFLOW`, `INFINITY`,
///   `DIVIDED_BY_ZERO`, `UNDEFINED`, `LEFT_CARRY`, and `RIGHT_CARRY`.
/// 
/// # Panics & Safety
/// * **Size Constraint**: If the total size (`size_of::<T>() * N`) is
///   16 bytes (128 bits) or less, certain methods may panic or exhibit
///   undefined behavior.
/// 
/// # Quick Start
/// You can define custom bit-widths using type aliases.
/// Although `u1024` is not a built-in Rust type,
/// you can easily define it as follows:
/// 
/// ## Example 1
/// ```rust
/// use cryptocol::number::*;
///
/// // Define a 1024-bit integer using 8 x u128
/// type u1024 = BigUInt<u128, 8>;
/// ```
/// Once defined, `u1024` can be used similarly to primitive types
/// like `u8` or `u64`.
/// 
/// ## Example 2: Alternative Implementations
/// ```rust
/// use cryptocol::number::*;
/// 
/// // Define a 1024-bit integer using 16 x u64
/// type u1024_Alt = BigUInt<u64, 16>;
/// ```
/// While `BigUInt<u64, 16>` and `BigUInt<u128, 8>` represent the same
/// bit-width, they consist of different base components.
/// You can even use `BigUInt<u8, 128>`.
/// The choice of base type is flexible and depends on your requirements.
/// 
/// ## Performance Guide
/// * The same size `BigUInt` can be made in several ways.
///   For example, a 1024-bit unsigned integer can be implemented with
///   either `BigUInt<u128, 8>`, `BigUInt<u64, 16>`, `BigUInt<u32, 32>`,
///   `BigUInt<u16, 64>`, or `BigUInt<u8, 128>`. They are all 1024-bit
///   unsigned integers, but their performance will be different from 
///   one another depending on the target CPU architecture.
/// * The choice of base type `T` significantly impacts performance
///   depending on the operation and the target CPU architecture:
///
/// | Operation      | Recommended Base Type (`T`) | Best Performance Context |
/// |----------------|-----------------------------|--------------------------|
/// | Addition       | `u128`                      | Debug & Release          |
/// | Subtraction    | `u128`                      | Debug & Release          |
/// | Multiplication | `u16`                       | Release mode             |
/// | Division       | `u64`                       | Release mode             |
/// 
/// *Note: On 64-bit architectures, `u128` generally performs best
/// in Debug mode across all operations.*
///
/// To find the optimal configuration for your specific hardware, refer to the 
/// performance test code section below and run it on your machine.
/// 
/// ## Convenience Modules and Constructors
/// For a more intuitive experience, it is recommended to import
/// `std::str::FromStr` and the entire `cryptocol::number` module.
/// 
/// ## Example 3: Basic Usage
/// ```rust
/// use std::str::FromStr;
/// use cryptocol::number::*;
/// 
/// type U1024 = BigUInt::<u128, 8>;
/// 
/// // Create from an array
/// let a_biguint = U1024::from([1_u128; 8]);
/// println!("a_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nUndefined: {}\nDivided_by_Zero: {}\nLeft_Carry: {}\nRight_Carry: {}", a_biguint.get_number(), a_biguint.is_overflow(), a_biguint.is_underflow(), a_biguint.is_infinity(), a_biguint.is_undefined(), a_biguint.is_divided_by_zero(), a_biguint.is_left_carry(), a_biguint.is_right_carry());
/// assert_eq!(*a_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(a_biguint.is_overflow(), false);
/// assert_eq!(a_biguint.is_underflow(), false);
/// assert_eq!(a_biguint.is_infinity(), false);
/// assert_eq!(a_biguint.is_undefined(), false);
/// assert_eq!(a_biguint.is_divided_by_zero(), false);
/// assert_eq!(a_biguint.is_left_carry(), false);
/// assert_eq!(a_biguint.is_right_carry(), false);
/// 
/// println!("a_biguint = {}", a_biguint);
/// let txt = format!("{}", a_biguint);
/// assert_eq!(txt, "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273");
/// 
/// println!("a_biguint = {:#x}", a_biguint);
/// let txt = format!("{:#x}", a_biguint);
/// assert_eq!(txt, "0x100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// // String formatting support
/// let b_biguint = U1024::from_string("528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273").unwrap();
/// println!("b_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nUndefined: {}\nDivided_by_Zero: {}\nLeft_Carry: {}\nRight_Carry: {}", b_biguint.get_number(), b_biguint.is_overflow(), b_biguint.is_underflow(), b_biguint.is_infinity(), b_biguint.is_undefined(), b_biguint.is_divided_by_zero(), b_biguint.is_left_carry(), b_biguint.is_right_carry());
/// assert_eq!(*b_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(b_biguint.is_overflow(), false);
/// assert_eq!(b_biguint.is_underflow(), false);
/// assert_eq!(b_biguint.is_infinity(), false);
/// assert_eq!(b_biguint.is_undefined(), false);
/// assert_eq!(b_biguint.is_divided_by_zero(), false);
/// assert_eq!(b_biguint.is_left_carry(), false);
/// assert_eq!(b_biguint.is_right_carry(), false);
/// 
/// println!("b_biguint = {}", b_biguint);
/// assert_eq!(b_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902273");
/// 
/// // String formatting and Radix support
/// println!("b_biguint = {:X}", b_biguint);
/// assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// let c_biguint = U1024::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// 
/// println!("c_biguint_biguint_biguint = {}", c_biguint);
/// assert_eq!(c_biguint.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
/// 
/// // Basic Arithmetic (Note: Operators consume the operands)
/// let mut d_biguint = b_biguint.clone() + c_biguint.clone();
/// println!("b_biguint + c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");
/// 
/// d_biguint = b_biguint.clone() - c_biguint.clone();
/// println!("b_biguint - c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");
/// 
/// d_biguint = c_biguint.clone() - b_biguint.clone();
/// println!("c_biguint_biguint - b_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");
/// 
/// d_biguint = c_biguint.clone() * b_biguint.clone();
/// println!("c_biguint_biguint * b_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");
/// 
/// d_biguint = b_biguint.clone() / c_biguint.clone();
/// println!("b_biguint / c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");
/// 
/// d_biguint = b_biguint.clone() % c_biguint.clone();
/// println!("b_biguint % c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");
/// 
/// d_biguint = b_biguint.clone() + 5_u128;
/// println!("b_biguint + 5 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");
/// 
/// d_biguint = b_biguint.clone() - 1_u128;
/// println!("b_biguint - 1 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");
/// 
/// d_biguint = b_biguint.clone() * 42_u128;
/// println!("b_biguint * 42 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");
/// 
/// d_biguint = b_biguint.clone() / 5_u128;
/// println!("b_biguint / 5 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");
/// 
/// let e_uint = b_biguint.clone() % 5_u128;
/// println!("b_uint % 5 = {}", e_uint);
/// assert_eq!(e_uint, 3);
/// ```
/// 
/// # Operators vs. Methods
/// **Caution**: Standard operators (e.g., `+`, `-`, `*`, `/`, `%`, `+=`, `-=`,
/// `*=`, `/=`, `%=`, `&`, `|`, `^`, `&=`, `|=`, `^=`, and `!`) use
/// **move semantics** and consume the operands.
/// To reuse variables, you must either `clone()` them or use the provided
/// **wrapping** or **checked** methods (e.g., `.wrapping_add(&rhs)`). Using
/// methods rather than operators is generally more performant as they
/// operate on references.
/// 
/// See the following example. Example 4 is a better version of Example 3 in
/// the viewpoint of performance though Example 4 looks less easy to read
/// than Example 3.
///  
/// ## Predefined Datatypes for Convenience
/// Common types like `U256`, `U512`, `U1024`, `UU32`, `UU64`, and `UU128` can
/// be generated using the `define_utypes_with!()` macro.
/// This macro supports `u8` through `u128` as base types.
/// `usize` is not supported due to its platform-dependent size.
/// So, Example 3 can be rewritten as Example 4.
/// 
/// ## Example 4: Using the Macro
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// 
/// define_utypes_with!(u128);
/// 
/// let a_biguint = U1024::from([1; 8]);
/// let b_biguint = U1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let c_biguint = UU128::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
/// 
/// println!("a_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", a_biguint.get_number(), a_biguint.is_overflow(), a_biguint.is_underflow(), a_biguint.is_infinity(), a_biguint.is_divided_by_zero());
/// assert_eq!(*a_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(a_biguint.is_overflow(), false);
/// assert_eq!(a_biguint.is_underflow(), false);
/// assert_eq!(a_biguint.is_infinity(), false);
/// assert_eq!(a_biguint.is_divided_by_zero(), false);
/// 
/// println!("a_biguint = {}", a_biguint.to_string_with_radix(16).unwrap());
/// assert_eq!(a_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("b_biguint = {:?}\nOverflow: {}\nUnderflow: {}\nInfiniity: {}\nDivided_by_Zero: {}", b_biguint.get_number(), b_biguint.is_overflow(), b_biguint.is_underflow(), b_biguint.is_infinity(), b_biguint.is_divided_by_zero());
/// assert_eq!(*b_biguint.get_number(), [1, 1, 1, 1, 1, 1, 1, 1]);
/// assert_eq!(b_biguint.is_overflow(), false);
/// assert_eq!(b_biguint.is_underflow(), false);
/// assert_eq!(b_biguint.is_infinity(), false);
/// assert_eq!(b_biguint.is_divided_by_zero(), false);
/// 
/// println!("b_biguint = {}", b_biguint.to_string_with_radix(16).unwrap());
/// assert_eq!(b_biguint.to_string_with_radix(16).unwrap(), "100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001");
/// 
/// println!("c_biguint_biguint = {}", c_biguint);
/// assert_eq!(c_biguint.to_string(), "1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000");
/// 
/// let mut d_biguint = c_biguint.wrapping_add(&b_biguint);
/// println!("b_biguint + c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173408080836782200210173329257227380279796317564269871499347888467433181081526414901390421871123571433486060157564694902273");
/// 
/// d_biguint = b_biguint.wrapping_sub(&c_biguint);
/// println!("b_biguint - c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173405611700999731074415082343649158057573873119825204832680999578544069970415081568056866315567793655708060157566694902273");
/// 
/// d_biguint = c_biguint.wrapping_sub(&b_biguint);
/// println!("c_biguint - b_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "179769313486231590772930519078902473361269403363094992027077741372816159198980563288580055091344426332604977474759407049726638194120401741388541284402205712176239488954006474494558295411072688507752083221010590686494501524284889008354087905708146237584806440714171216671890379622911922649127296172057529234943");
/// 
/// d_biguint = c_biguint.wrapping_mul(&b_biguint);
/// println!("c_biguint * b_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "59830717854030867758075123183163555064720825939616846267926369121354707541167863856429897315021801292311343603281484761713479005341939688693125073345149826515313989515871501605159397439048630578377892313876159164289859563003628270426845234033215692532247483706885131175507859004610238546564083383732338767360");
/// 
/// d_biguint = b_biguint.wrapping_div(&c_biguint);
/// println!("b_biguint / c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "427918573686029304066254243786715892164567464161173266402914429285403265969001177679575353202952599315891695262671719654199608368852942773933951103642477");
/// 
/// d_biguint = b_biguint.wrapping_rem(&c_biguint);
/// println!("b_biguint % c_biguint = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "974831854472745921484474959642423157588012401465652792186214606232572248263942179693215574222740495163800042694902273");
/// 
/// d_biguint = b_biguint.wrapping_add_uint(5_u128);
/// println!("b_biguint + 5 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902278");
/// 
/// d_biguint = b_biguint.wrapping_sub_uint(1_u128);
/// println!("b_biguint - 1 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "528294531135665246352339784916516606520399844128422231063109688515136405111986307932151574694014881104306146237268412201528404470859010781743924190173406846268890965642294205800438269168685095342047538166014444022988625525970748234723644093345682544597060157565694902272");
/// 
/// d_biguint = b_biguint.wrapping_mul_uint(42_u128);
/// println!("b_biguint * 42 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "22188370307697940346798270966493697473856793453393733704650606917635729014703424933150366137148625006380858141965273312464192987776078452833244815987283087543293420556976356643618407305084774004365996602972606648965522272090771425858393051920518666873076526617759185895466");
/// 
/// d_biguint = b_biguint.wrapping_div_uint(5_u128);
/// println!("b_biguint / 5 = {}", d_biguint);
/// assert_eq!(d_biguint.to_string(), "105658906227133049270467956983303321304079968825684446212621937703027281022397261586430314938802976220861229247453682440305680894171802156348784838034681369253778193128458841160087653833737019068409507633202888804597725105194149646944728818669136508919412031513138980454");
/// 
/// let e_uint = b_biguint.wrapping_rem_uint(5_u128);
/// println!("b_biguint % 5 = {}", e_uint);
/// assert_eq!(e_uint, 3);
/// ```
/// 
/// However, if you want to use any datatypes that are not predefined
/// such as `u136`, `U144`, `U192`, `U320`, `U384`, etc. or `UU17`, `UU18`,
/// `UU24`, `UU40`, `UU48`, etc., you can define them for yourself as Example 5.
/// For non-standard widths (e.g., 136-bit, 192-bit), define them manually:
///  
/// ## Example 5: Custom Widths
/// ```
/// use cryptocol::number::*;
/// type U136 = BigUInt::<u8, 17>;
/// type U144 = BigUInt::<u16, 9>;
/// type U192 = BigUInt::<u32, 6>;
/// type U320 = BigUInt::<u64, 5>;
/// type U348 = BigUInt::<u128, 3>;
/// type UU17 = BigUInt::<u8, 17>;
/// type UU18 = BigUInt::<u16, 9>;
/// type UU24 = BigUInt::<u32, 6>;
/// type UU40 = BigUInt::<u64, 5>;
/// type UU48 = BigUInt::<u128, 3>;
/// ```
/// 
/// ## Performance Test Results (Reference)
/// The following table summarizes performance tests conducted on
/// a **64-bit machine**. Results may vary on 32-bit architectures.
/// | Operation      | Best base (Release)              | Best base (Debug)                |
/// |----------------|----------------------------------|----------------------------------|
/// | Addition       | `u128` (mostly) / `u64` (rarely) | `u128` (always)                  |
/// | Subtraction    | `u128` (always)                  | `u128` (always)                  |
/// | Multiplication | `u16`  (always)                  | `u128` (mostly) / `u64` (rarely) |
/// | Division       | `u64` (always)                   | `u128` (mostly) / `u64` (rarely) |
/// 
/// The following is the code used for the Performance Test.
/// performance!() is a macro. And, Rust Playground may not run this code
/// correctly because of using a user-defined macro. You are recommended to
/// build and run the performance test code on your own computer by yourself
/// rather than on Rust Playground.
/// 
/// ## Performance Test Code
/// Use the following macro and main function to benchmark `BigUInt` on your
/// local environment. It is recommended to run this locally rather than
/// in the Rust Playground for accurate results.
/// ```rust
/// macro_rules! performance
/// {
///     ($t:ty, $b:expr, $ti:expr, $f:expr) => {
///         let mut sum = <$t>::zero();
///         let a = <$t>::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
///         let now: SystemTime;
///         match $f
///         {
///             0 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_add_assign(&a); }
///             },
///             1 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_sub_assign(&a); }
///             },
///             2 => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_mul_assign(&a); }
///             },
///             _ => {
///                 now = SystemTime::now();
///                 for _ in 0..100_0000
///                     { sum.wrapping_div_assign(&a); }
///             },
///         }
///         match now.elapsed()
///         {
///             Ok(elapsed) => {
///                 $ti = elapsed.as_micros();
///                 println!("{}-based: {} usec", $b, $ti);
///             },
///             Err(e) => { println!("{}", e); },
///         }
///     }
/// }
/// 
/// fn main()
/// {
///     use std::time::SystemTime;
///     use cryptocol::number::*;
/// 
///     let mut ti = [0_u128; 5];   // How many microseconds
///     let dt = ["u128", "u64", "u32", "u16", "u8"];
///     let op = ["addition", "subtraction", "multplication", "division" ];
/// 
///     for operator in 0..4
///     {
///         println!("=== {} ===", op[operator]);
///         performance!(U1024_with_u128, dt[0], ti[0], operator);
///         performance!(U1024_with_u64, dt[1], ti[1], operator);
///         performance!(U1024_with_u32, dt[2], ti[2], operator);
///         performance!(U1024_with_u16, dt[3], ti[3], operator);
///         performance!(U1024_with_u8, dt[4], ti[4], operator);
///     
///         let mut fastest = 0;
///         for i in 1..5
///         {
///             if ti[fastest] > ti[i]
///                 { fastest = i; }
///         }
///         println!("The fastest one is {}.\n", dt[fastest]);
///         
///         #[cfg(debug_assertions)]
///         {
///             assert_eq!(fastest, 0); // It means u128 shows the best performance most of the time.
///         }
///         #[cfg(not(debug_assertions))]
///         {
///             if operator < 2
///                 { assert_eq!(fastest, 0); } // It means u128 shows the best performance.
///             else
///                 { assert_eq!(fastest, 1); } // It means u64 shows the best performance most of the time.
///         }
///     }
/// }
/// ```
/// `U1024_with_u128`, `U1024_with_u64`, `U1024_with_u32`, `U1024_with_u16`,
/// and `U1024_with_u128` are all predefined datatypes too.
/// 
/// So, if ypu calculate addition and subtraction mainly, you'd better use
/// `u128`-based datatype such as `BigUInt<u128, N>`. Or use predefined
/// datatype as follows.
/// 
/// ## Example 6
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u128);
/// let a_biguint = U1024::new();
/// ```
/// If you calculate multiplication and division mainly, you'd better use
/// `u16`-based datatype for multiplication and `u64`-based datatype for
/// division such as `BigUInt<u16, N>` and `BigUInt<u64, N>`, respectively.
/// Or use predefined datatype as follows.
/// 
/// ## Example 7
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u16);
/// let a_biguint = U1024::new();
/// ```
/// 
/// # Endianness Warning
/// This implementation is **experimental on Big-Endian CPUs**. 
/// Use in production environments on Big-Endian architectures at your own risk.
#[derive(Debug, Clone)]
pub struct BigUInt<T, const N: usize>
where T: TraitsBigUInt<T>
{
    // method_widening_mul_assign_uint: fn(&mut Self, T) -> Self,
    // method_wrapping_mul_assign_uint: fn(&mut Self, T),
    // method_widening_mul_assign: fn(&mut Self, &Self) -> Self,
    // method_wrapping_mul_assign: fn(&mut Self, &Self),
    number: [T; N],
    flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    /***** CONSTANTS FOR FLAGS *****/

    /// Indicates whether an overflow occurred during a previous operation.
    pub(super) const OVERFLOW: u8   = 0b0000_0001;

    /// Indicates whether an underflow occurred during a previous operation.
    pub(super) const UNDERFLOW: u8  = 0b0000_0010;
    
    /// Indicates that the value reached infinity, typically due to 
    /// operations like division by zero.
    pub(super) const INFINITY: u8   = 0b0000_0100;

    /// Indicates whether a division-by-zero error occurred.
    pub(super) const DIVIDED_BY_ZERO: u8    = 0b0000_1000;

    /// Indicates an undefined result. For example, this flag is set 
    /// when performing an indeterminate operation such as `0 / 0`.
    pub(super) const UNDEFINED: u8  = 0b0001_0000;

    /// Indicates whether a bit was shifted out (carried) during 
    /// a previous left-shift operation.
    pub(super) const LEFT_CARRY: u8 = 0b0010_0000;

    /// Indicates whether a bit was shifted out (carried) during 
    /// a previous right-shift operation.
    pub(super) const RIGHT_CARRY: u8    = 0b0100_0000;


    #[allow(non_upper_case_globals)]
    const method_widening_mul_assign_uint: fn(&mut Self, T) -> Self
            =   if N > 16
                    { Self::widening_mul_assign_uint_1 }
                else
                    { Self::widening_mul_assign_uint_2 };

    #[allow(non_upper_case_globals)]
    const method_wrapping_mul_assign_uint: fn(&mut Self, T)
            =   if N > 16
                    { Self::wrapping_mul_assign_uint_1 }
                else
                    { Self::wrapping_mul_assign_uint_2 };

    #[allow(non_upper_case_globals)]
    const method_widening_mul_assign: fn(&mut Self, &Self) -> Self
            =   if N > 16
                    { Self::widening_mul_assign_1 }
                else
                    { Self::widening_mul_assign_2 };

    #[allow(non_upper_case_globals)]
    const method_wrapping_mul_assign: fn(&mut Self, &Self)
            =   if N > 16
                    { Self::wrapping_mul_assign_1 }
                else
                    { Self::wrapping_mul_assign_2 };

    /***** CONSTRUCTORS *****/

    // pub const fn new() -> Self
    /// Constructs a new `BigUInt<T, N>` initialized to zero.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance where all internal elements 
    /// and flags are set to `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let obj = U256::new();
    /// println!("obj = {}", obj);
    /// assert_eq!(obj.to_string(), "0");
    /// assert_eq!(obj.is_overflow(), false);
    /// assert_eq!(obj.is_underflow(), false);
    /// assert_eq!(obj.is_infinity(), false);
    /// assert_eq!(obj.is_divided_by_zero(), false);
    /// assert_eq!(obj.is_undefined(), false);
    /// assert_eq!(obj.is_left_carry(), false);
    /// assert_eq!(obj.is_right_carry(), false);
    /// ```
    pub const fn new() -> Self
    {
        Self
        {
            number: [T::MIN; N],
            flag: 0,
        }
    }

    // pub const fn zero() -> Self
    /// Constructs a new `BigUInt<T, N>` with a value of zero.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing zero.
    /// 
    /// # Implementation Details
    /// This is a convenience wrapper that calls [`Self::new()`]. 
    /// Both methods are functionally identical.
    /// 
    /// # Usage Note
    /// Using `zero()` instead of `new()` is recommended when you want 
    /// to explicitly indicate that the variable is being initialized 
    /// to the numeric value of zero, improving code readability.
    ///
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let zero = U256::zero();
    /// println!("zero = {}", zero);
    /// assert_eq!(zero.to_string(), "0");
    /// assert_eq!(zero.is_overflow(), false);
    /// assert_eq!(zero.is_underflow(), false);
    /// assert_eq!(zero.is_infinity(), false);
    /// assert_eq!(zero.is_divided_by_zero(), false);
    /// assert_eq!(zero.is_undefined(), false);
    /// assert_eq!(zero.is_left_carry(), false);
    /// assert_eq!(zero.is_right_carry(), false);
    /// ```
    #[inline]
    pub const fn zero() -> Self
    {
        Self::new()   // unsafe { zeroed::<Self>() }
    }

    // pub const fn one() -> Self
    /// Constructs a new `BigUInt<T, N>` with a value of one.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing one.
    /// 
    /// # Usage Note
    /// Using `one()` is a more readable and expressive alternative to 
    /// creating a zeroed instance via `new()` and subsequently calling 
    /// `set_uint(1)`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let one = U256::one();
    /// println!("one = {}", one);
    /// assert_eq!(one.to_string(), "1");
    /// assert_eq!(one.is_overflow(), false);
    /// assert_eq!(one.is_underflow(), false);
    /// assert_eq!(one.is_infinity(), false);
    /// assert_eq!(one.is_divided_by_zero(), false);
    /// assert_eq!(one.is_undefined(), false);
    /// assert_eq!(one.is_left_carry(), false);
    /// assert_eq!(one.is_right_carry(), false);
    /// ```
    pub const fn one() -> Self
    {
        let mut me = Self::zero();
        me.set_num_(0, T::ONE);
        me
    }

    // pub const fn max() -> Self
    /// Constructs a new `BigUInt<T, N>` with the maximum possible value.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance where every bit is set to `1`.
    /// 
    /// # Implementation Details
    /// This represents the largest value that can be held by the current 
    /// bit-width (`size_of::<T>() * N * 8`).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let maximum = U256::max();
    /// assert_eq!(maximum.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(maximum.is_overflow(), false);
    /// assert_eq!(maximum.is_underflow(), false);
    /// assert_eq!(maximum.is_infinity(), false);
    /// assert_eq!(maximum.is_divided_by_zero(), false);
    /// assert_eq!(maximum.is_undefined(), false);
    /// assert_eq!(maximum.is_left_carry(), false);
    /// assert_eq!(maximum.is_right_carry(), false);
    /// assert_eq!(maximum.wrapping_add_uint(1_u16), U256::zero());
    /// ```
    pub const fn max() -> Self
    {
        Self
        {
            number: [T::MAX; N],
            flag: 0,
        }
    }

    // pub fn submax(size_in_bits: usize) -> Self
    /// Constructs a new `BigUInt<T, N>` where the first `size_in_bits`
    /// are set to `1`.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing the maximum value attainable 
    /// within the specified bit length.
    /// 
    /// # Implementation Details
    /// This method sets all bits from the Least Significant Bit (LSB) up to 
    /// `size_in_bits` to `1`, while all remaining bits up to the Most 
    /// Significant Bit (MSB) are initialized to `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::submax(128_u32);
    /// println!("half maximum = \t{}", half);
    /// println!("half maximum = \t{}", half.to_string_with_radix_and_stride(16, 4).unwrap());
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(half.is_overflow(), false);
    /// assert_eq!(half.is_underflow(), false);
    /// assert_eq!(half.is_infinity(), false);
    /// assert_eq!(half.is_divided_by_zero(), false);
    /// assert_eq!(half.is_undefined(), false);
    /// assert_eq!(half.is_left_carry(), false);
    /// assert_eq!(half.is_right_carry(), false);
    /// ```
    pub fn submax(size_in_bits: u32) -> Self
    {
        let mut res = Self::max();
        res.set_submax(size_in_bits);
        res
    }

    // pub fn halfmax() -> Self
    /// Constructs a new `BigUInt<T, N>` representing
    /// a half-length maximum value.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance
    /// where the lower half of the bits are set to `1`.
    /// 
    /// # Implementation Details
    /// This method sets all bits in the lower half (starting from
    /// the Least Significant Bit) to `1`, while the remaining upper half bits
    /// up to the Most Significant Bit are initialized to `0`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let half = U256::halfmax();
    /// println!("half maximum = {0} = {0:#x}", half);
    /// assert_eq!(half.to_string(), "340282366920938463463374607431768211455");
    /// assert_eq!(half.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(half.is_overflow(), false);
    /// assert_eq!(half.is_underflow(), false);
    /// assert_eq!(half.is_infinity(), false);
    /// assert_eq!(half.is_divided_by_zero(), false);
    /// assert_eq!(half.is_undefined(), false);
    /// assert_eq!(half.is_left_carry(), false);
    /// assert_eq!(half.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn halfmax() -> Self
    {
        Self::submax(Self::size_in_bits() >> 1)
    }

    // pub fn from_uint<U>(val: U) -> Self
    /// Constructs a new `BigUInt<T, N>` from a primitive unsigned integer.
    /// Supported types include `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing the same value as `val`.
    /// 
    /// # Panics & Safety
    /// **Warning**: If the total internal storage size (`size_of::<T>() * N`)
    /// is 128 bits (16 bytes) or less, this method may panic or exhibit
    /// undefined behavior.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with_u16;
    /// define_utypes_with_u16!();
    /// 
    /// let a_from_u8 = U512::from_uint(123_u8);
    /// let b_from_u16 = U512::from_uint(12345_u16);
    /// let c_from_u32 = U512::from_uint(1234567890_u32);
    /// let d_from_u64 = U512::from_uint(12345678901234567890_u64);
    /// let e_from_u128 = U512::from_uint(123456789012345678901234567890123456789_u128);
    /// let f_from_usize = U512::from_uint(123_usize);
    /// 
    /// println!("a_from_u8 = {}", a_from_u8);
    /// println!("b_from_u16 = {}", b_from_u16);
    /// println!("c_from_u32 = {}", c_from_u32);
    /// println!("d_from_u64 = {}", d_from_u64);
    /// println!("e_from_u128 = {}", e_from_u128);
    /// println!("f_from_usize = {}", f_from_usize);
    /// 
    /// assert_eq!(a_from_u8.into_u8(), 123_u8);
    /// assert_eq!(a_from_u8.is_overflow(), false);
    /// assert_eq!(a_from_u8.is_underflow(), false);
    /// assert_eq!(a_from_u8.is_infinity(), false);
    /// assert_eq!(a_from_u8.is_divided_by_zero(), false);
    /// assert_eq!(a_from_u8.is_undefined(), false);
    /// assert_eq!(a_from_u8.is_left_carry(), false);
    /// assert_eq!(a_from_u8.is_right_carry(), false);
    /// 
    /// assert_eq!(b_from_u16.into_u16(), 12345_u16);
    /// assert_eq!(b_from_u16.is_overflow(), false);
    /// assert_eq!(b_from_u16.is_underflow(), false);
    /// assert_eq!(b_from_u16.is_infinity(), false);
    /// assert_eq!(b_from_u16.is_divided_by_zero(), false);
    /// assert_eq!(b_from_u16.is_undefined(), false);
    /// assert_eq!(b_from_u16.is_left_carry(), false);
    /// assert_eq!(b_from_u16.is_right_carry(), false);
    /// 
    /// assert_eq!(c_from_u32.into_u32(), 1234567890_u32);
    /// assert_eq!(c_from_u32.is_underflow(), false);
    /// assert_eq!(c_from_u32.is_infinity(), false);
    /// assert_eq!(c_from_u32.is_divided_by_zero(), false);
    /// assert_eq!(c_from_u32.is_undefined(), false);
    /// assert_eq!(c_from_u32.is_left_carry(), false);
    /// assert_eq!(c_from_u32.is_right_carry(), false);
    /// 
    /// assert_eq!(d_from_u64.into_u64(), 12345678901234567890_u64);
    /// assert_eq!(d_from_u64.is_overflow(), false);
    /// assert_eq!(d_from_u64.is_underflow(), false);
    /// assert_eq!(d_from_u64.is_infinity(), false);
    /// assert_eq!(d_from_u64.is_divided_by_zero(), false);
    /// assert_eq!(d_from_u64.is_undefined(), false);
    /// assert_eq!(d_from_u64.is_left_carry(), false);
    /// assert_eq!(d_from_u64.is_right_carry(), false);
    /// 
    /// assert_eq!(e_from_u128.into_u128(), 123456789012345678901234567890123456789_u128);
    /// assert_eq!(e_from_u128.is_overflow(), false);
    /// assert_eq!(e_from_u128.is_underflow(), false);
    /// assert_eq!(e_from_u128.is_infinity(), false);
    /// assert_eq!(e_from_u128.is_divided_by_zero(), false);
    /// assert_eq!(e_from_u128.is_undefined(), false);
    /// assert_eq!(e_from_u128.is_left_carry(), false);
    /// assert_eq!(e_from_u128.is_right_carry(), false);
    /// 
    /// assert_eq!(f_from_usize.into_usize(), 123_usize);
    /// assert_eq!(f_from_usize.is_overflow(), false);
    /// assert_eq!(f_from_usize.is_underflow(), false);
    /// assert_eq!(f_from_usize.is_infinity(), false);
    /// assert_eq!(f_from_usize.is_divided_by_zero(), false);
    /// assert_eq!(f_from_usize.is_undefined(), false);
    /// assert_eq!(f_from_usize.is_left_carry(), false);
    /// assert_eq!(f_from_usize.is_right_carry(), false);
    /// ```
    pub fn from_uint<U>(val: U) -> Self
    where U: TraitsBigUInt<U>
    {
        let size_t = T::size_in_bytes();
        let size_u = U::size_in_bytes();
        let mut me = Self::zero();
        let mut share = SharedValues::<T, U>::from_src(val);
//        unsafe { copy_nonoverlapping(val.as_ptr() as *const u8, me.number.as_mut_ptr() as *mut u8, size_u); }
        if size_t >= size_u
        {
            unsafe { me.set_num_(0, share.des); }
        }
        else    // if size_t < size_u
        {
            let size_t_bits = size_t * 8;
            for i in 0..size_u/size_t
            {
                unsafe { me.set_num_(i as usize, share.des); }
                unsafe { share.src >>= U::u32_as_smalluint(size_t_bits); }
            }
        }
        me
    }

    // pub const fn from_array(val: [T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>` from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing the same value
    /// as the input array `val`.
    /// 
    /// # Alternatives
    /// This is a direct equivalent to the [`From<[T; N]>`] trait implementation. 
    /// You can also use [`BigUInt::from(val)`](#impl-From%3C%5BT%3B+N%3D%3E%5D-for-BigUInt%3CT%2C+N%3E).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let big_num = U256::from_array([10_u8; 32]);
    /// println!("big_num = {:X}", big_num);
    /// assert_eq!(big_num.to_string_with_radix(16).unwrap(), "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A");
    /// assert_eq!(big_num.is_overflow(), false);
    /// assert_eq!(big_num.is_underflow(), false);
    /// assert_eq!(big_num.is_infinity(), false);
    /// assert_eq!(big_num.is_divided_by_zero(), false);
    /// assert_eq!(big_num.is_undefined(), false);
    /// assert_eq!(big_num.is_left_carry(), false);
    /// assert_eq!(big_num.is_right_carry(), false);
    /// ```
    pub const fn from_array(val: [T; N]) -> Self
    {
        Self { number: val, flag: 0 }
    }

    // pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    /// Constructs a new `BigUInt<T, N>` from a `BigUInt<U, M>` with different
    /// generic parameters.
    /// 
    /// # Returns
    /// A `BigUInt<T, N>` instance representing the same value as the provided
    /// `biguint`.
    /// 
    /// # Implementation Details
    /// This method performs a deep copy of both the high-precision integer data
    /// and the current operational flags from the source instance.
    /// 
    /// # Example 1 for the same length
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::*;
    /// 
    /// let mut a_u512_with_u8 = U512_with_u8::from_str("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789").unwrap();
    /// a_u512_with_u8.set_overflow();
    /// a_u512_with_u8.set_underflow();
    /// a_u512_with_u8.set_infinity();
    /// a_u512_with_u8.set_divided_by_zero();
    /// a_u512_with_u8.set_undefined();
    /// a_u512_with_u8.set_left_carry();
    /// a_u512_with_u8.set_right_carry();
    /// assert_eq!(a_u512_with_u8.is_overflow(), true);
    /// assert_eq!(a_u512_with_u8.is_underflow(), true);
    /// assert_eq!(a_u512_with_u8.is_infinity(), true);
    /// assert_eq!(a_u512_with_u8.is_divided_by_zero(), true);
    /// assert_eq!(a_u512_with_u8.is_undefined(), true);
    /// assert_eq!(a_u512_with_u8.is_left_carry(), true);
    /// assert_eq!(a_u512_with_u8.is_right_carry(), true);
    /// 
    /// let b_u512_with_u8 = U512_with_u8::from_biguint(&a_u512_with_u8);
    /// println!("a_u512_with_u8 = {}", a_u512_with_u8);
    /// println!("b_u512_with_u8 = {}", b_u512_with_u8);
    /// assert_eq!(a_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u512_with_u8.to_string(), "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
    /// assert_eq!(b_u512_with_u8.is_overflow(), false);
    /// assert_eq!(b_u512_with_u8.is_underflow(), false);
    /// assert_eq!(b_u512_with_u8.is_infinity(), false);
    /// assert_eq!(b_u512_with_u8.is_divided_by_zero(), false);
    /// assert_eq!(b_u512_with_u8.is_undefined(), false);
    /// assert_eq!(b_u512_with_u8.is_left_carry(), false);
    /// assert_eq!(b_u512_with_u8.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.from_biguint)
    #[inline]
    pub fn from_biguint<U, const M: usize>(biguint: &BigUInt<U, M>) -> Self
    where U: TraitsBigUInt<U>
    {
        Self::from_array(unsafe {SharedArrays::<T, N, U, M>::from_src(biguint.get_number()).des})
    }

    // pub fn from_be(be: Self) -> Self
    /// Converts a big unsigned integer from big-endian byte order to the 
    /// target architecture's endianness.
    /// 
    /// # Implementation Details
    /// * On **big-endian** architectures, this is a no-op and returns the value
    ///   unchanged.
    /// * On **little-endian** architectures, the byte order is reversed
    ///   (swapped) to match the internal little-endian representation.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let be = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                             0x1122, 0x3344, 0x5566, 0x7788,
    ///                             0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                             0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le = U256::from_be(be.clone());
    /// println!("be = {:#x}", be);
    /// println!("le = {:#x}", le);
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");        
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(be.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");        
    /// }
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_be(be: Self) -> Self
    {
        #[cfg(target_endian = "little")]    return be.swap_bytes();
        #[cfg(target_endian = "big")]       return be.clone();
    }

    // pub fn from_be_bytes(be_bytes: [T; N]) -> Self
    /// Creates a native-endian `BigUInt<T, N>` from an array in big-endian
    /// byte order.
    /// 
    /// # Implementation Details
    /// * On **big-endian** architectures, the array is used as-is (no-op).
    /// * On **little-endian** architectures, the byte order is reversed 
    ///   to align with the internal little-endian storage format.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let be_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_be_bytes(be_array.clone());
    /// print!("be_array = ");
    /// for elem in be_array
    ///     { print!("{:#8x} ", elem); }
    /// println!();
    /// println!("le = {:#x}", le);
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "78563412_EFCDAB90_44332211_88776655_BBAA0099_FFEEDDCC_4C3D2E1F_89706A5B");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    pub fn from_be_bytes(be_bytes: [T; N]) -> Self
    {
        #[cfg(target_endian = "little")]
        {
            let mut me = Self::from_array(be_bytes);
            me.swap_bytes_assign();
            me
        } 
        #[cfg(target_endian = "big")]       return Self::from_array(be_bytes);
    }

    // pub fn from_le(le: Self) -> Self
    /// Converts a big unsigned integer from little-endian byte order to the
    /// target architecture's endianness.
    /// 
    /// # Implementation Details
    /// * On **little-endian** architectures, this is a no-op
    ///   and returns the value unchanged.
    /// * On **big-endian** architectures, the byte order is reversed (swapped)
    ///   to match the target's native representation.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let le1 = U256::from_array([0x1234, 0x5678, 0x90ab, 0xcdef,
    ///                 0x1122, 0x3344, 0x5566, 0x7788,
    ///                 0x9900, 0xaabb, 0xccdd, 0xeeff,
    ///                 0x1f2e, 0x3d4c, 0x5b6a, 0x7089]);
    /// let le2 = U256::from_le(le1.clone());
    /// println!("le1 = {:#x}", le1);
    /// println!("le2 = {:#x}", le2);
    /// #[cfg(target_endian = "little")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "70895B6A3D4C1F2EEEFFCCDDAABB99007788556633441122CDEF90AB56781234");
    /// }
    /// #[cfg(target_endian = "big")]
    /// {
    ///     assert_eq!(le1.to_string_with_radix(16).unwrap(), "1234567890ABCDEF11223344556677889900AABBCCDDEEFF1F2E3D4C5B6A7089");
    ///     assert_eq!(le2.to_string_with_radix(16).unwrap(), "34127856AB90EFCD22114433665588770099BBAADDCCFFEE2E1F4C3D6A5B8970");
    /// }
    /// assert_eq!(le2.is_overflow(), false);
    /// assert_eq!(le2.is_underflow(), false);
    /// assert_eq!(le2.is_infinity(), false);
    /// assert_eq!(le2.is_divided_by_zero(), false);
    /// assert_eq!(le2.is_undefined(), false);
    /// assert_eq!(le2.is_left_carry(), false);
    /// assert_eq!(le2.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_le(le: Self) -> Self
    {
        #[cfg(target_endian = "little")]    return le;
        #[cfg(target_endian = "big")]       return le.swap_bytes();
    }
    
    // pub fn from_le_bytes(le_bytes: [T; N]) -> Self
    /// Creates a native-endian `BigUInt<T, N>` from an array in little-endian
    /// byte order.
    /// 
    /// # Implementation Details
    /// * On **little-endian** architectures, the array is used as-is (no-op).
    /// * On **big-endian** architectures, the byte order is reversed (swapped)
    ///   to match the target's native representation.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let le_array = [0x12345678, 0x90abcdef, 0x11223344, 0x55667788,
    ///                 0x9900aabb, 0xccddeeff, 0x1f2e3d4c, 0x5b6a7089];
    /// let le = U256::from_le_bytes(le_array.clone());
    /// print!("le_array = ");
    /// for elem in le_array
    ///     { print!("{:#8x} ", elem); }
    /// println!();
    /// println!("le = {:#x}", le);
    /// #[cfg(target_endian = "little")]    assert_eq!(le.to_string_with_radix_and_stride(16, 8).unwrap(), "5B6A7089_1F2E3D4C_CCDDEEFF_9900AABB_55667788_11223344_90ABCDEF_12345678");
    /// #[cfg(target_endian = "big")]       assert_eq!(le.to_string_with_radix(16).unwrap(), "12345678_90ABCDEF_11223344_55667788_9900AABB_CCDDEEFF_1F2E3D4C_5B6A7089");
    /// assert_eq!(le.is_overflow(), false);
    /// assert_eq!(le.is_underflow(), false);
    /// assert_eq!(le.is_infinity(), false);
    /// assert_eq!(le.is_divided_by_zero(), false);
    /// assert_eq!(le.is_undefined(), false);
    /// assert_eq!(le.is_left_carry(), false);
    /// assert_eq!(le.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn from_le_bytes(le_bytes: [T; N]) -> Self
    {
        #[cfg(target_endian = "little")]    return Self::from_array(le_bytes);
        #[cfg(target_endian = "big")]
        {
            let mut me = Self::from_array(le_bytes);
            me.swap_bytes_assign();
            me
        } 
    }

    //  pub fn from_string(txt: &str) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a decimal string.
    /// 
    /// # Returns
    /// Returns `Ok(BigUInt<T, N>)` if the string is successfully parsed. 
    /// Otherwise, it returns a `NumberErr` indicating the cause of failure.
    /// 
    /// # Digit Separators
    /// You can use the underscore (`_`) as a separator to improve readability.
    /// For example, `"1_000_000"` and `"100_0000"` are
    /// both treated as `"1000000"`.
    /// 
    /// # Errors
    /// This method evaluates errors in the following order of priority:
    /// 
    /// | Priority | Condition                                            | Error Type                   |
    /// |:---------|:-----------------------------------------------------|:-----------------------------|
    /// | 1st      | Contains non-alphanumeric characters (excluding `_`) | `NumberErr::NotAlphaNumeric` |
    /// | 2nd      | Contains non-decimal digits (e.g., letters)          | `NumberErr::NotFitToRadix`   |
    /// | 3rd      | The value exceeds the maximum capacity of the type   | `NumberErr::TooBigNumber`    |
    /// 
    /// If multiple error conditions are met,
    /// only the one with the highest priority is returned.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U256::from_string("1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890");
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///         println!("a_correct = {}", n);
    ///         assert_eq!(n.to_string(), "1234567890123456789012345678901234567890123456789012345678901234567890");
    ///         assert_eq!(n.is_overflow(), false);
    ///         assert_eq!(n.is_underflow(), false);
    ///         assert_eq!(n.is_infinity(), false);
    ///         assert_eq!(n.is_divided_by_zero(), false);
    ///         assert_eq!(n.is_undefined(), false);
    ///         assert_eq!(n.is_left_carry(), false);
    ///         assert_eq!(n.is_right_carry(), false);
    ///     },
    ///     Err(e) => { println!("Failed: {}", e); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.from_string)
    #[inline]
    pub fn from_string(txt: &str) -> Result<Self, NumberErr>
    {
        Self::from_str_radix(txt, 10)
    }

    //  pub fn from_str_radix(txt: &str, radix: u32) -> Result<Self, NumberErr>
    /// Constructs a new `BigUInt<T, N>` from a string with the given `radix`.
    /// 
    /// # Returns
    /// Returns `Ok(BigUInt<T, N>)` if the string is successfully parsed. 
    /// Otherwise, it returns a `NumberErr` indicating the cause of failure.
    /// 
    /// # Valid Radix Range
    /// The supported radix range is **2 to 62** (inclusive). 
    /// * **Radix 2–36**: Case-insensitive. Letters `A`–`Z` (or `a`–`z`)
    ///   represent values 10–35.
    /// * **Radix 37–62**: Case-sensitive. `A`–`Z` represent 10–35,
    ///   and `a`–`z` represent 36–61.
    /// 
    /// # Digit Separators
    /// You can use the underscore (`_`) as a separator to improve readability.
    /// For example, `"1_0000"` is treated the same as `"10000"`.
    /// 
    /// # Errors
    /// This method evaluates errors in the following order of priority:
    /// 
    /// | Priority | Condition                                                  | Error Type                        |
    /// |:---------|:-----------------------------------------------------------|:----------------------------------|
    /// | 1st      | `radix` is less than `2` or greater than `62`              | `NumberErr::OutOfValidRadixRange` |
    /// | 2nd      | `txt` contains non-alphanumeric characters (excluding `_`) | `NumberErr::NotAlphaNumeric`      |
    /// | 3rd      | `txt` contains characters that exceed the given `radix`    | `NumberErr::NotFitToRadix`        |
    /// | 4th      | The value exceeds the maximum capacity of the type         | `NumberErr::TooBigNumber`         |
    /// 
    /// If multiple error conditions are met,
    /// only the one with the highest priority is returned.
    /// 
    /// # Example 1 for correct case
    /// ```
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_correct = U512::from_str_radix("1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0", 16);
    /// match a_correct
    /// {
    ///     Ok(n) => {
    ///         println!("a_correct = {}", n);
    ///         assert_eq!(n.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0");
    ///         assert_eq!(n.is_overflow(), false);
    ///         assert_eq!(n.is_underflow(), false);
    ///         assert_eq!(n.is_infinity(), false);
    ///         assert_eq!(n.is_divided_by_zero(), false);
    ///         assert_eq!(n.is_undefined(), false);
    ///         assert_eq!(n.is_left_carry(), false);
    ///         assert_eq!(n.is_right_carry(), false);
    ///     },
    ///     Err(e) => { println!("Failed: {}", e); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.from_str_radix)
    pub fn from_str_radix(txt: &str, radix: u32) -> Result<Self, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        let mut bignum = Self::zero();
        for c in txt.chars()
        {
            if c == '_'
                { continue; }
            if !c.is_alphanumeric()
                { return Err(NumberErr::NotAlphaNumeric); }
            if radix <= 10
            {
                if c as u32 >= '0' as u32 + radix
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if radix <= 10 + 26
            {
                if (c as u32 >= 'A' as u32 + radix - 10) 
                        && (c as u32 <= 'Z' as u32)
                    || (c as u32 >= 'a' as u32 + radix - 10)
                    { return Err(NumberErr::NotFitToRadix); }
            }
            else if c as u32 >= 'a' as u32 + radix - (10 + 26)  // radix <= 10 + 26 + 26
                { return Err(NumberErr::NotFitToRadix); }

            let num: u32 = if radix <= 10    { c as u32 - '0' as u32 }
                        else if radix <= 10 + 26
                        {
                            if c <= '9'
                                { c as u32 - '0' as u32 }
                            else if c <= 'Z'
                                { c as u32 - 'A' as u32 + 10 }
                            else
                                { c as u32 - 'a' as u32 + 10 }
                        }
                        else    // if radix <= 10 + 26 + 26
                        {
                            if c <= '9'
                                { c as u32 - '0' as u32 }
                            else if c <= 'Z'
                                { c as u32 - 'A' as u32 + 10 }
                            else
                                { c as u32 - 'a' as u32 + 10 + 26 }
                        };
            bignum.wrapping_mul_assign_uint(T::u32_as_smalluint(radix));
            bignum.wrapping_add_assign_uint(T::u32_as_smalluint(num));
        }
        if bignum.is_overflow()
            { Err(NumberErr::TooBigNumber) }
        else
            { Ok(bignum) }
    }

    // pub fn generate_check_bits(bit_pos: u32) -> Option<Self>
    /// Constructs a new `BigUInt<T, N>` with only the bit at the specified 
    /// position set to `1`.
    /// 
    /// # Returns
    /// * Returns `Some(BigUInt<T, N>)` if `bit_pos` is within the valid range 
    ///   (less than the total bit-width).
    /// * Returns `None` if `bit_pos` is greater than or equal to the total 
    ///   bit-width (`size_of::<T>() * N * 8`).
    /// 
    /// # Bit Position
    /// The `bit_pos` argument uses zero-based indexing, starting from the 
    /// Least Significant Bit (LSB) regardless of the architecture's endianness. 
    /// For example, if `bit_pos` is `0`, only the LSB is set to `1`, and all 
    /// other bits are set to `0`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits(0).unwrap();
    /// println!("a_0 = {:#b}", a_0);
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// assert_eq!(a_0.is_overflow(), false);
    /// assert_eq!(a_0.is_underflow(), false);
    /// assert_eq!(a_0.is_infinity(), false);
    /// assert_eq!(a_0.is_divided_by_zero(), false);
    /// assert_eq!(a_0.is_undefined(), false);
    /// assert_eq!(a_0.is_left_carry(), false);
    /// assert_eq!(a_0.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.generate_check_bits)
    pub fn generate_check_bits(bit_pos: u32) -> Option<Self>
    {
        if bit_pos < Self::size_in_bits()
            { Some(Self::generate_check_bits_(bit_pos)) }
        else
            { None }
    }

    // pub fn generate_check_bits_(bit_pos: u32) -> Self
    /// Constructs a new `BigUInt<T, N>` with only the bit at the specified 
    /// position set to `1`.
    /// 
    /// # Returns
    /// Returns a `BigUInt<T, N>` instance where only the bit at `bit_pos` is `1`.
    /// 
    /// # Panics
    /// * **Out of Bounds**: Panics if `bit_pos` is greater than or equal to 
    ///   the total bit-width (`size_of::<T>() * N * 8`).
    /// * **Safety Warning**: If the internal storage size (`size_of::<T>() * N`) 
    ///   is 128 bits or less, this method may exhibit undefined behavior or 
    ///   panic depending on the context.
    /// 
    /// # Bit Position
    /// The `bit_pos` uses zero-based indexing, starting from the Least 
    /// Significant Bit (LSB) regardless of the architecture's endianness. 
    /// For example, if `bit_pos` is `0`, only the LSB is set to `1`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with_u32;
    /// define_utypes_with_u32!();
    /// 
    /// let a_0 = U256::generate_check_bits_(0);
    /// println!("a_0 = {:#b}", a_0);
    /// assert_eq!(a_0.to_string_with_radix_and_stride(2, 10).unwrap(), "1");
    /// assert_eq!(a_0.is_overflow(), false);
    /// assert_eq!(a_0.is_underflow(), false);
    /// assert_eq!(a_0.is_infinity(), false);
    /// assert_eq!(a_0.is_divided_by_zero(), false);
    /// assert_eq!(a_0.is_undefined(), false);
    /// assert_eq!(a_0.is_left_carry(), false);
    /// assert_eq!(a_0.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.generate_check_bits_)
    pub fn generate_check_bits_(bit_pos: u32) -> Self
    {
        let mut check_bits = Self::zero();
        check_bits.turn_check_bits(bit_pos);
        check_bits
    }


    
    /***** METHODS TO GET SIZE BOTH IN BYTES AND BITS *****/

    // pub const fn size_in_bytes() -> u32
    /// Returns the total size of the `BigUInt` data in bytes.
    /// 
    /// # Returns
    /// The number of bytes used for the internal high-precision integer storage.
    /// 
    /// # Implementation Details
    /// This value represents the raw data size (`size_of::<T>() * N`) and 
    /// does not include any additional bytes used for internal flags or 
    /// metadata.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// println!("U256 is {}-byte integer.", U256::size_in_bytes());
    /// assert_eq!(U256::size_in_bytes(), 32);
    /// ```
    #[inline]
    pub const fn size_in_bytes() -> u32
    {
        T::BITS / 8 * N as u32
    }

    // pub const fn size_in_bits() -> u32
    /// Returns the total storage capacity of the `BigUInt` in bits.
    /// 
    /// # Returns
    /// The number of bits available for high-precision integer data.
    /// 
    /// # Implementation Details
    /// This value represents the full bit-width (`size_of::<T>() * N * 8`) and 
    /// does not include any internal flags or metadata used for state 
    /// tracking.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// println!("U256 is {}-bit integer.", U256::size_in_bits());
    /// assert_eq!(U256::size_in_bits(), 256);
    /// ```
    #[inline]
    pub const fn size_in_bits() -> u32
    {
        T::BITS * N as u32
    }

    // pub fn length_in_bytes(&self) -> u32
    /// Returns the size of the `BigUInt` instance in bytes.
    ///
    /// # Returns
    /// The number of bytes allocated for storing the high-precision
    /// integer value.
    ///
    /// # Implementation Details
    /// This value reflects the static byte-width of the `BigUInt` and 
    /// does not include space used for internal flags or metadata.
    ///
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a_biguint is {}-byte integer.", a_biguint.length_in_bytes());
    /// assert_eq!(a_biguint.length_in_bytes(), 32);
    /// ```
    #[inline]
    pub fn length_in_bytes(&self) -> u32
    {
        Self::size_in_bytes()
    }

    // pub fn length_in_bits(&self) -> u32
    /// Returns the size of the `BigUInt` instance in bits.
    ///
    /// # Returns
    /// The total bit-width allocated for storing the high-precision
    /// integer value.
    ///
    /// # Implementation Details
    /// This value reflects the static bit-width of the `BigUInt` and 
    /// does not include space used for internal flags or metadata.
    ///
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("A16F", 16).unwrap();
    /// println!("a_biguint is {}-bit integer.", a_biguint.length_in_bits());
    /// assert_eq!(a_biguint.length_in_bits(), 256);
    /// ```
    #[inline]
    pub fn length_in_bits(&self) -> u32
    {
        Self::size_in_bits()
    }



    /***** METHODS TO GET, SET, AND CHECK *****/

    // pub fn turn_check_bits(&mut self, bit_pos: u32)
    /// Resets the `BigUInt` instance to zero and sets only the specified bit
    /// to 1.
    ///
    /// # Arguments
    /// * `bit_pos`: The zero-based index of the bit to be set, counted from 
    ///   the Least Significant Bit (LSB) regardless of the architecture's
    ///   endianness.
    ///
    /// # Panics
    /// * Panics if `bit_pos` is greater than or equal to the total bit-width
    ///   of the `BigUInt`.
    ///
    /// # Implementation Details
    /// If the internal storage size is 128 bits or less, certain operations
    /// may exhibit undefined behavior or result in a panic depending on the
    /// environment.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("256487951236974125896345564889974258").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.turn_check_bits(102);
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint, U256::from_str_radix("1000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000", 2).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.turn_check_bits)
    pub fn turn_check_bits(&mut self, bit_pos: u32)
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = bit_pos / size_t_bits;
        let piece_num = bit_pos % size_t_bits;
        let mut val = T::one();
        val <<= T::u32_as_smalluint(piece_num);
        self.set_zero();
        self.set_num_(chunk_num as usize, val);
    }

    // pub fn is_bit_set(&self, bit_pos: u32) -> Option<bool>
    /// Checks if the bit at the specified position is set to 1.
    ///
    /// # Arguments
    /// * `bit_pos`: The zero-based index of the bit to check, counted from the
    ///   Least Significant Bit (LSB) regardless of the architecture's
    ///   endianness.
    ///
    /// # Returns
    /// * `Some(true)` if the bit is 1.
    /// * `Some(false)` if the bit is 0.
    /// * `None` if `bit_pos` is out of the valid range for this `BigUInt`.
    ///
    /// # Implementation Details
    /// For better performance in cases where the `bit_pos` is guaranteed to
    /// be within range, consider using the unchecked variant `is_bit_set_()`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    /// let res = a_biguint.is_bit_set(151);
    /// match res
    /// {
    ///     Some(r) => {
    ///         println!("The {}th bit is set: {}", 151, r);
    ///         assert_eq!(a_biguint.is_bit_set_(151), true);
    ///         assert_eq!(a_biguint.is_overflow(), false);
    ///         assert_eq!(a_biguint.is_underflow(), false);
    ///         assert_eq!(a_biguint.is_infinity(), false);
    ///         assert_eq!(a_biguint.is_undefined(), false);
    ///         assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{}_U256 does not have the {}th bit.", a_biguint, 151);
    ///     }
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_bit_set)
    pub fn is_bit_set(&self, bit_pos: u32) -> Option<bool>
    {
        if (bit_pos / T::size_in_bits()) >= N as u32
            { None }
        else
            { Some(self.is_bit_set_(bit_pos)) }
    }

    // pub fn is_bit_set_(&self, bit_pos: u32) -> bool
    /// Checks if the bit at the specified position is set to 1 without range
    /// checking.
    ///
    /// # Arguments
    /// * `bit_pos`: The zero-based index of the bit to check, counted from the
    ///   Least Significant Bit (LSB) regardless of the architecture's
    ///   endianness.
    ///
    /// # Returns
    /// * `true` if the bit is 1.
    /// * `false` if the bit is 0.
    ///
    /// # Panics
    /// * Panics if `bit_pos` is greater than or equal to the total bit-width
    ///   of the `BigUInt`.
    ///
    /// # Implementation Details
    /// This is a performance-oriented unchecked version of `is_bit_set()`. 
    /// Use this method only when `bit_pos` is guaranteed to be within the 
    /// valid range.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// println!("a_biguint = {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap());
    /// println!("The {}th bit is set: {}", 151, a_biguint.is_bit_set_(151));
    /// assert_eq!(a_biguint.is_bit_set_(151), true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_bit_set_)
    pub fn is_bit_set_(&self, bit_pos: u32) -> bool
    {
        let size_t_bits = T::size_in_bits();
        let chunk_num = bit_pos / size_t_bits;
        let piece_num = bit_pos % size_t_bits;
        self.get_num_(chunk_num as usize).is_bit_set_(piece_num)
    }

    // pub fn get_upper_portion(&self, portion: u32) -> Self
    /// Extracts the most significant bits of the non-zero part of the 
    /// `BigUInt`.
    ///
    /// # Arguments
    /// * `portion`: The number of high-order bits to extract from the non-zero 
    ///   portion of the number.
    ///
    /// # Returns
    /// A new `BigUInt` containing the requested upper portion. If `portion` 
    /// exceeds the length of the non-zero part, a clone of `self` is returned.
    ///
    /// # Implementation Details
    /// The "non-zero part" is defined as the range from the most significant 
    /// non-zero bit down to the Least Significant Bit (LSB). For example, the 
    /// non-zero part of `00101100` is `101100`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912345").unwrap();
    /// let b = a_biguint.get_upper_portion(10);
    /// println!("The 10-bit upper portion of {}_U256 is {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101101001");
    /// ```
    pub fn get_upper_portion(&self, portion: u32) -> Self
    {
        let leading = self.leading_zeros();
        let size = self.length_in_bits();
        let available = size - leading;
        if portion >= available
            { self.clone() }
        else
            { self.shift_right(available - portion) }
    }

    // pub fn get_lower_portion(&self, portion: u32) -> Self
    /// Extracts the least significant bits of the non-zero part of the 
    /// `BigUInt`.
    ///
    /// # Arguments
    /// * `portion`: The number of low-order bits to extract from the non-zero 
    ///   portion of the number.
    ///
    /// # Returns
    /// A new `BigUInt` containing the requested lower portion. If `portion` 
    /// is `0`, it returns `BigUInt::zero()`.
    ///
    /// # Implementation Details
    /// The "non-zero part" is defined as the range from the most significant 
    /// non-zero bit down to the Least Significant Bit (LSB). For example, the 
    /// non-zero part of `00101100` is `101100`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("12345678912345678912345678912345678912345678912345678912345678912345678912340").unwrap();
    /// let b = a_biguint.get_lower_portion(10);
    /// println!("The 10-bit lower portion of {}_U256 is {}_U256", a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), b.to_string_with_radix_and_stride(2, 10).unwrap());
    /// assert_eq!(b.to_string_with_radix_and_stride(2, 10).unwrap(), "1101010100");
    /// ```
    pub fn get_lower_portion(&self, portion: u32) -> Self
    {
        let leading = self.leading_zeros();
        let size = self.length_in_bits();
        let available = size - leading;
        let mut ret = self.clone();
        if portion == 0
        {
            return Self::zero();
        }
        else if portion < available
        {
            let size_t_bits = T::size_in_bits();
            let chunk_num = (portion - 1) / size_t_bits;
            let piece_num = portion % size_t_bits;
            if piece_num != 0
            {
                let mut thing = ret.get_num_(chunk_num as usize);
                thing <<= T::u32_as_smalluint(T::size_in_bits() - piece_num);
                thing >>= T::u32_as_smalluint(T::size_in_bits() - piece_num);
                ret.set_num_(chunk_num as usize, thing);
            }
            for i in (chunk_num + 1)..(N as u32 - leading / size_t_bits)
                { ret.set_num_(i as usize, T::zero()); }
        }
        ret
    }

    // pub fn get_num(&self, i: usize) -> Option<T>
    /// Returns the i-th internal storage element of type `T`.
    ///
    /// # Arguments
    /// * `i`: The zero-based index of the storage element. The element at index
    ///   0 contains the Least Significant Bit (LSB), while index `N-1` contains
    ///   the Most Significant Bit (MSB), regardless of the architecture's
    ///   endianness.
    ///
    /// # Returns
    /// * `Some(T)` if `i` is within the valid range (less than `N`).
    /// * `None` if `i` is greater than or equal to `N`.
    ///
    /// # Implementation Details
    /// For better performance in cases where the index `i` is guaranteed to 
    /// be within range, consider using the unchecked variant `get_num_()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// match a_biguint.get_num(3)
    /// {
    ///     Some(num) => {
    ///         println!("a_biguint.get_num(3).unwrap() = {}", num);
    ///         assert_eq!(num, 30);
    ///     },
    ///     None => {
    ///         println!("There is no third element.");
    ///     },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.get_num)
    pub fn get_num(&self, i: usize) -> Option<T>
    {
        if i < N
        {
            #[cfg(target_endian = "little")]    { Some(self.get_number()[i]) }
            #[cfg(target_endian = "big")]       { Some(self.get_number()[N-1-i]) }
        }
        else
        {
            None
        }
    }

    // pub fn get_num_(&self, i: usize) -> T
    /// Returns the i-th internal storage element of type `T` without range 
    /// checking.
    ///
    /// # Arguments
    /// * `i`: The zero-based index of the storage element. The element at index
    ///   0 contains the Least Significant Bit (LSB), while index `N-1` contains
    ///   the Most Significant Bit (MSB), regardless of the architecture's
    ///   endianness.
    ///
    /// # Returns
    /// The storage element of type `T` at the specified index.
    ///
    /// # Panics
    /// Panics if the index `i` is greater than or equal to `N`.
    ///
    /// # Implementation Details
    /// This is a performance-oriented unchecked version of `get_num()`. 
    /// Use this method only when the index `i` is guaranteed to be within the 
    /// valid range.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from([0_u32, 10, 20, 30, 40, 50, 60, 70]);
    /// let b = a_biguint.get_num_(3);
    /// println!("a_biguint.get_num_(3) = {}", b);
    /// assert_eq!(b, 30);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.get_num_)
    #[inline]
    pub fn get_num_(&self, i: usize) -> T
    {
        #[cfg(target_endian = "little")]    { self.number[i] }
        #[cfg(target_endian = "big")]       { self.number[N-1-i] }
    }

    // pub fn set_num(&mut self, i: usize, val: T) -> bool
    /// Sets the i-th internal storage element to the specified value.
    ///
    /// # Arguments
    /// * `i`: The zero-based index of the storage element to set.
    /// * `val`: The value of type `T` to be stored at the specified index.
    ///
    /// # Returns
    /// * `true` if the index `i` is within the valid range (less than `N`) and
    ///   the value was successfully set.
    /// * `false` if `i` is greater than or equal to `N`.
    ///
    /// # Implementation Details
    /// For better performance in cases where the index `i` is guaranteed to 
    /// be within range, consider using the unchecked variant `set_num_()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from([0_u64, 10, 20, 30]);
    /// let mut num = a_biguint.get_num_(3);
    /// println!("a_biguint.get_num(3).unwrap() = {}", num);
    /// assert_eq!(a_biguint.get_num_(3), 30);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b = a_biguint.set_num(3, 0);
    /// assert_eq!(b, true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// num = a_biguint.get_num_(3);
    /// println!("a_biguint.get_num(3).unwrap() = {}", num);
    /// assert_eq!(num, 0);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.set_num)
    pub fn set_num(&mut self, i: usize, val: T) -> bool
    {
        if i < N
        {
            #[cfg(target_endian = "little")]    { self.number[i] = val; }
            #[cfg(target_endian = "big")]       { self.number[N-1-i] = val; }
            true
        }
        else
        {
            false
        }
    }

    // pub fn set_num_(&mut self, i: usize, val: T)
    /// Sets the i-th internal storage element to the specified value without 
    /// range checking.
    ///
    /// # Arguments
    /// * `i`: The zero-based index of the storage element to set.
    /// * `val`: The value of type `T` to be stored at the specified index.
    ///
    /// # Panics
    /// Panics if the index `i` is greater than or equal to `N`.
    ///
    /// # Implementation Details
    /// This is a performance-oriented unchecked version of `set_num()`. 
    /// Use this method only when the index `i` is guaranteed to be within the 
    /// valid range.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from([10_u128, 20]);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mut num = a_biguint.get_num_(1);
    /// println!("a_biguint.get_num_(1) = {}", num);
    /// assert_eq!(num, 20);
    /// 
    /// a_biguint.set_num_(1, 300);
    /// num = a_biguint.get_num_(1);
    /// println!("a_biguint.get_num_(1) = {}", num);
    /// assert_eq!(num, 300);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.set_num_)
    #[inline]
    pub const fn set_num_(&mut self, i: usize, val: T)
    {
        #[cfg(target_endian = "little")]    { self.number[i] = val; }
        #[cfg(target_endian = "big")]       { self.number[N-1-i] = val; }
    }

    // pub fn get_number(&self) -> &[T; N]
    /// Returns an immutable reference to the internal storage array.
    ///
    /// # Returns
    /// A reference to the underlying array of type `[T; N]`.
    ///
    /// # Implementation Details
    /// This method provides read-only access to the internal data for 
    /// borrowing without transferring ownership.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// if let Ok(a_biguint) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a_biguint.get_number();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
    /// ```
    #[inline]
    pub fn get_number(&self) -> &[T; N]
    {
        &self.number
    }

    // pub fn get_number_mut(&mut self) -> &mut [T; N]
    /// Returns a mutable reference to the internal storage array.
    ///
    /// # Returns
    /// A mutable reference to the underlying array of type `[T; N]`.
    ///
    /// # Implementation Details
    /// This method provides direct mutable access to the internal storage,
    /// allowing for in-place modifications of the raw data.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// if let Ok(a_biguint) = "12345678909876543210123456789098765432101234567890987654321012345678909876543".parse::<U256>()
    /// {
    ///     let arr = a_biguint.get_number_mut();
    ///     println!("arr = {:?}", arr);
    ///     assert_eq!(arr, &[169027903, 1302152522, 3897323189, 3259190507, 1179716839, 4196280276, 2015458651, 457926681]);
    /// }
    /// ```
    #[inline]
    pub fn get_number_mut(&mut self) -> &mut [T; N]
    {
        &mut self.number
    }

    // pub fn set_number(&mut self, val: &[T; N])
    /// Updates the internal storage with the contents of the provided array.
    ///
    /// # Arguments
    /// * `val`: A reference to an array of type `[T; N]` containing the new 
    ///   values to be stored.
    ///
    /// # Implementation Details
    /// This method performs a bulk update of the entire storage array using 
    /// slice copying. This operation does not modify the internal state flags.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("arr = {:?}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let arr = [1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    /// a_biguint.set_number(&arr);
    /// println!("arr = {:?}", a_biguint);
    /// assert_eq!(a_biguint.get_number(), &arr);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_number(&mut self, val: &[T; N])
    {
        self.number.copy_from_slice(val);
    }

    // fn copy_within<R>(&mut self, src: R, dest: usize)
    /// Copies elements from one part of the internal storage array to another
    /// using an efficient `memmove` operation.
    ///
    /// # Arguments
    /// * `src`: The range of indices within the storage array to copy from. 
    ///   Indices are counted from the Least Significant Bit (LSB) to the Most 
    ///   Significant Bit (MSB), regardless of endianness.
    /// * `dest`: The starting index where the source range will be copied to. 
    ///   The destination range will have the same length as `src`.
    ///
    /// # Panics
    /// * Panics if the internal storage bit-width is 128 bits or less, as 
    ///   certain operations may result in undefined behavior or a panic in 
    ///   specific environments.
    /// * Panics if either the source or destination range exceeds the bounds 
    ///   of the storage array, or if the end of `src` is before the start.
    ///
    /// # Implementation Details
    /// The two ranges may overlap; the copy operation correctly handles 
    /// overlapping regions. This method directly manipulates the underlying 
    /// storage array.
    #[inline]
    fn copy_within<R>(&mut self, src: R, dest: usize)
    where R: RangeBounds<usize>
    {
        #[cfg(target_endian = "little")]
        self.number.copy_within(src, dest);

        #[cfg(target_endian = "big")]
        {
            let mut start: usize;
            let mut end: usize;
    
            match src.end_bound()
            {
                Excluded(s) =>  { start = (N - s); },
                Included(s) =>  { start = (N - 1 - s); },
                Unbounded =>    { start = 0; }
            }
            match src.start_bound()
            {
                Excluded(s) =>  { end = (N - s); },
                Included(s) =>  { end = (N - 1 - s); },
                Unbounded =>    { end = N - 1; }
            }
            let new_src = Range::<&usize> { start: &start, end: &end };
            let new_dest = N - 1 - dest;
            self.number.copy_within(new_src, new_dest);
        }
    }

    // pub fn set_zero(&mut self)
    /// Resets the `BigUInt` instance to zero.
    ///
    /// # Implementation Details
    /// This method sets all internal storage elements to zero and clears
    /// all status flags.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// a_biguint.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U256::zero());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_zero(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::zero()); }
    }

    // pub fn is_zero(&self) -> bool
    /// Checks if the `BigUInt` instance is zero.
    ///
    /// # Returns
    /// * `true` if all internal storage elements are zero.
    /// * `false` if the number has a non-zero value.
    ///
    /// # Implementation Details
    /// This method evaluates the numeric magnitude of the instance and does 
    /// not consider internal status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U1024::zero();
    /// let mut b_zero = a_biguint.is_zero();
    /// if b_zero
    /// {
    ///     println!("a_biguint is Zero");
    ///     assert_eq!(b_zero, true);
    /// }
    /// else
    /// {
    ///     println!("a_biguint is Not Zero");
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_zero)
    pub fn is_zero(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn set_one(&mut self)
    /// Sets the `BigUInt` instance to the value of one.
    ///
    /// # Implementation Details
    /// This method resets all storage elements to zero and then sets the 
    /// Least Significant Bit (LSB) to 1.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// a_biguint.set_number(&[1_u16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_one();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U256::one());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_one(&mut self)
    {
        self.set_zero();
        #[cfg(target_endian = "little")]    { self.set_num(0, T::one()); }
        #[cfg(target_endian = "big")]       { self.set_num(N-1, T::one()); }
    }

    // pub fn is_one(&self) -> bool
    /// Checks if the `BigUInt` instance is equal to one.
    ///
    /// # Returns
    /// * `true` if the magnitude of the number is exactly one.
    /// * `false` if the number has any other value.
    ///
    /// # Implementation Details
    /// This method evaluates the numeric magnitude and does not consider 
    /// internal status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::one();
    /// let mut b_one = a_biguint.is_one();
    /// if b_one
    /// {
    ///     println!("a_biguint is One");
    ///     assert_eq!(b_one, true);
    /// }
    /// else
    /// {
    ///     println!("a_biguint is Not One");
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_one)
    pub fn is_one(&self) -> bool
    {
        if self.get_num_(0) != T::one()
            { return false; }

        for i in 1..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn is_zero_or_one(&self) -> bool
    /// Checks if the `BigUInt` instance is either zero or one.
    ///
    /// # Returns
    /// * `true` if the magnitude of the number is 0 or 1.
    /// * `false` if the magnitude is 2 or greater.
    ///
    /// # Implementation Details
    /// This method is a shortcut for `self.is_zero() || self.is_one()`, 
    /// optimized for checking small constants.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// println!("a_biguint = {}", a_biguint);
    /// let b_zero_or_one = a_biguint.is_zero_or_one();
    /// if b_zero_or_one
    /// {
    ///     println!("a_biguint is One or Zero.");
    ///     assert_eq!(b_zero_or_one, true);
    /// }
    /// else
    /// {
    ///     println!("a_biguint is Neither One nor Zero.");
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_zero_or_one)
    pub fn is_zero_or_one(&self) -> bool
    {
        if self.get_num_(0) > T::one()
            { return false; }

        for i in 1..N
        {
            if self.get_num_(i) != T::zero()
                { return false; }
        }
        true
    }

    // pub fn set_max(&mut self)
    /// Sets the `BigUInt` instance to its maximum possible value.
    ///
    /// # Implementation Details
    /// This method sets every bit in the internal storage to 1, effectively 
    /// representing the largest value attainable for the current bit-width.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_max();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_max(&mut self)
    {
        for i in 0..N
            { self.set_num(i, T::max()); }
    }

    // pub fn set_submax(&mut self, size_in_bits: u32)
    /// Sets the `BigUInt` instance to the maximum value that fits within the
    /// specified number of bits.
    ///
    /// # Arguments
    /// * `size_in_bits`: The number of low-order bits to set to 1.
    ///
    /// # Implementation Details
    /// This method sets all bits from the Least Significant Bit (LSB) up to
    /// `size_in_bits` to 1, while initializing all remaining bits up to the
    /// Most Significant Bit (MSB) to 0.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_submax(200_u32);
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_submax(&mut self, size_in_bits: u32)
    {
        let size_t_bits = T::size_in_bits();
        if size_in_bits >= self.length_in_bits()
        {
            self.set_max();
            return;
        }
        else if size_in_bits == 0
        {
            self.set_zero();
            return;
        }

        let chunk_num = size_in_bits / size_t_bits;
        let piece_num = size_in_bits % size_t_bits;
        let zero = T::zero();
        let max = T::max();
        self.reset_all_flags();
        for i in 0..chunk_num
            { self.set_num_(i as usize, max); }
        for i in chunk_num as usize..N
            { self.set_num_(i, zero); }
        if piece_num != 0
            { self.set_num_(chunk_num as usize, max >> T::u32_as_smalluint(size_t_bits - piece_num)); }
    }

    // pub fn set_halfmax(&mut self)
    /// Sets the `BigUInt` instance to the maximum value of its lower-half 
    /// bit-width.
    ///
    /// # Implementation Details
    /// This method sets all bits in the lower half of the total bit-width to 1
    /// and initializes the upper half to 0.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_halfmax();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_halfmax(&mut self)
    {
        self.set_submax(self.length_in_bits() >> 1);
    }

    // pub fn is_max(&self) -> bool
    /// Checks if the `BigUInt` instance is equal to its maximum possible 
    /// value.
    ///
    /// # Returns
    /// * `true` if every bit in the internal storage is set to 1.
    /// * `false` otherwise.
    ///
    /// # Implementation Details
    /// This method evaluates the numeric magnitude and does not consider 
    /// internal status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::max();
    /// println!("Is {} the 256-bit maximum? - {}", a_biguint, a_biguint.is_max());
    /// assert_eq!(a_biguint.is_max(), true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_max)
    pub fn is_max(&self) -> bool
    {
        for i in 0..N
        {
            if self.get_num_(i) != T::max()
                { return false; }
        }
        true
    }

    // pub fn set_msb(&mut self)
    /// Sets the Most Significant Bit (MSB) of the `BigUInt` to 1.
    ///
    /// # Implementation Details
    /// This method modifies only the highest bit of the internal storage 
    /// array.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_msb();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_msb(&mut self)
    {
        let mut highest = self.get_num_(N-1);
        highest.set_msb();
        self.set_num_(N-1, highest);
    }

    // pub fn reset_msb(&mut self)
    /// Resets the Most Significant Bit (MSB) of the `BigUInt` to 0.
    ///
    /// # Implementation Details
    /// This method clears only the highest bit of the internal storage 
    /// array.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_msb();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn reset_msb(&mut self)
    {
        let mut highest = self.get_num_(N-1);
        highest.reset_msb();
        self.set_num_(N-1, highest);
    }

    // pub fn set_lsb(&mut self)
    /// Sets the Least Significant Bit (LSB) of the `BigUInt` to 1.
    ///
    /// # Implementation Details
    /// This method modifies only the lowest bit of the internal storage 
    /// array.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_lsb();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_lsb(&mut self)
    {
        let mut lowest = self.get_num_(0);
        lowest.set_lsb();
        self.set_num_(0, lowest);
    }

    // pub fn reset_lsb(&mut self)
    /// Resets the Least Significant Bit (LSB) of the `BigUInt` to 0.
    ///
    /// # Implementation Details
    /// This method clears only the lowest bit of the internal storage 
    /// array.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::one();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_lsb();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn reset_lsb(&mut self)
    {
        let mut lowest = self.get_num_(0);
        lowest.reset_lsb();
        self.set_num_(0, lowest);
    }

    // pub fn set_uint<U>(&mut self, val: U)
    /// Sets the `BigUInt` instance to the value of a primitive unsigned 
    /// integer.
    ///
    /// # Arguments
    /// * `val`: The primitive unsigned integer value to set (e.g., `u8`, 
    ///   `u16`, `u32`, `u64`, `u128`).
    ///
    /// # Panics
    /// * Panics if the bit-width of `U` is greater than the total bit-width
    ///   of the `BigUInt`.
    ///
    /// # Implementation Details
    /// This method is optimized for initializing a `BigUInt` with a small 
    /// constant value. It clears the existing value and all status flags 
    /// before setting the new value.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U1024::new();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_uint(340282366920938463453374607431768211455_u128);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "340282366920938463453374607431768211455");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn set_uint<U>(&mut self, val: U)
    where U: TraitsBigUInt<U>
    {
        let size_t = T::size_in_bytes();
        let size_v = U::size_in_bytes();
        let mut share = SharedValues::<T, U>::from_src(val);
        
        self.set_zero();
        if size_t >= size_v
        {
            unsafe { self.set_num_(0, share.des); }
        }
        else    // size_v is multiple of size_t.
        {
            let size_t_bits = size_t * 8;
            #[cfg(target_endian = "little")]
            for i in 0..size_v/size_t
            {
                unsafe { self.set_num_(i as usize, share.des); }
                unsafe { share.src >>= U::u32_as_smalluint(size_t_bits); }
            }
            #[cfg(target_endian = "big")]
            {
                let mut i = size_v/size_t - 1;
                loop
                {
                    unsafe { self.set_num_(i, share.des); }
                    unsafe { share.src <<= U::usize_as_smalluint(size_t_bits); }
                    if i == 0
                        { break; }
                    i -= 1;
                }
            }
        }
    }

    // pub fn is_uint<U>(&self, val: U) -> bool
    /// Checks if the `BigUInt` instance is equal to a primitive unsigned 
    /// integer.
    ///
    /// # Arguments
    /// * `val`: The primitive unsigned integer value to compare against.
    ///
    /// # Returns
    /// * `true` if the magnitude of the number is equal to `val`.
    /// * `false` otherwise.
    ///
    /// # Implementation Details
    /// This method is optimized for comparing a `BigUInt` with small 
    /// constant values and provides similar functionality to `eq_uint()`.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U1024::one() + 50_u16;
    /// println!("Question: Is a_biguint 51?\nAnswer: {}", a_biguint.is_uint(51_u32));
    /// assert_eq!(a_biguint.is_uint(51_u16), true);
    /// assert_eq!(a_biguint.is_uint(50_u16), false);
    /// ```
    pub fn is_uint<U>(&self, val: U) -> bool
    where U: TraitsBigUInt<U>
    {
        let size_t = T::size_in_bytes();
        let size_v = U::size_in_bytes();
        let mut share = SharedValues::<T, U>::from_src(val);
        
        if size_t >= size_v
        {
            if unsafe { self.get_num_(0) != share.des }
                { return false; }
            for i in 1..N
            {
                if self.get_num_(i) != T::zero()
                    { return false; }
            }
        }
        else    // size_v is multiple of size_t.
        {
            let size_t_bits = size_t * 8;
            #[cfg(target_endian = "little")]
            for i in 0..size_v/size_t
            {
                if unsafe { self.get_num_(i as usize) != share.des }
                    { return false; }
                unsafe { share.src >>= U::u32_as_smalluint(size_t_bits); }
            }
            #[cfg(target_endian = "big")]
            {
                let mut i = size_v/size_t - 1;
                loop
                {
                    if unsafe { self.get_num_(i) != share.des }
                        { return false; }
                    if i == 0
                        { break; }
                    unsafe { share.src <<= U::usize_as_smalluint(size_t_bits); }     
     
                    i -= 1;          
                }
            }
            for i in size_v/size_t..N as u32
            {
                if self.get_num_(i as usize) != T::zero()
                    { return false; }
            }
        }
        true
    }

    // pub fn is_odd(&self) -> bool
    /// Checks if the `BigUInt` instance represents an odd number.
    ///
    /// # Returns
    /// * `true` if the Least Significant Bit (LSB) is 1.
    /// * `false` if the LSB is 0.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::new();
    /// a_biguint.set_uint(340282366920938463453374697431768211455_u128);
    /// if a_biguint.is_odd()
    ///     { println!("{} is odd", a_biguint); }
    /// else
    ///     { println!("{} is even", a_biguint); }
    /// assert_eq!(a_biguint.is_odd(), true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_odd)
    #[inline]
    pub fn is_odd(&self) -> bool
    {
        self.get_num_(0).is_odd()
    }

    // pub fn is_even(&self) -> bool
    /// Checks if the `BigUInt` instance represents an even number.
    ///
    /// # Returns
    /// * `true` if the Least Significant Bit (LSB) is 0.
    /// * `false` if the LSB is 1.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::new();
    /// a_biguint.set_uint(169743176821145534028236692093846345739_u128);
    /// if a_biguint.is_even()
    ///     { println!("{} is even", a_biguint); }
    /// else
    ///     { println!("{} is odd", a_biguint); }
    /// assert_eq!(a_biguint.is_even(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_even)
    #[inline]
    pub fn is_even(&self) -> bool
    {
        !self.is_odd()
    }

    // pub fn is_msb_set(&self) -> bool
    /// Checks if the Most Significant Bit (MSB) of the `BigUInt` is set to 1.
    ///
    /// # Returns
    /// * `true` if the highest bit of the total bit-width is 1.
    /// * `false` if the highest bit is 0.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(169743176821145534028236692093846345739_u128);
    /// if a_biguint.is_msb_set()
    ///     { println!("{} is greater than halfmax ({}).", a_biguint, U256::halfmax()); }
    /// else
    ///     { println!("{} is less than or equal to halfmax ({}).", a_biguint, U256::halfmax()); }
    /// assert_eq!(a_biguint.is_msb_set(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.is_msb_set)
    #[inline]
    pub fn is_msb_set(&self) -> bool
    {
        self.get_num_(N-1).is_msb_set()
    }



    /***** METHODS TO CHECK BITS *****/

    // pub fn count_ones(&self) -> u32
    /// Returns the number of ones in the binary representation of the 
    /// `BigUInt`.
    /// 
    /// # Returns
    /// The total count of bits set to 1 across all internal storage elements.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} is {} in binary and has {} ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.count_ones());
    /// assert_eq!(a_biguint.count_ones(), 107);
    /// ```
    pub fn count_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.get_num_(i).count_ones(); }
        res
    }

    // pub fn count_zeros(&self) -> u32
    /// Returns the number of zeros in the binary representation of the 
    /// `BigUInt`.
    /// 
    /// # Returns
    /// The total count of bits set to 0 across all internal storage elements.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = "100000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.count_zeros());
    /// assert_eq!(a_biguint.count_zeros(), 149);
    /// ```
    pub fn count_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
            { res += self.get_num_(i).count_zeros(); }
        res
    }

    // pub fn leading_ones(&self) -> u32
    /// Returns the number of leading ones in the binary representation of 
    /// the `BigUInt`.
    /// 
    /// # Returns
    /// The count of consecutive bits set to 1 starting from the Most 
    /// Significant Bit (MSB).
    /// 
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("100000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// println!("{} is {} in binary and has {} leading ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_ones());
    /// assert_eq!(a_biguint.leading_ones(), 2);
    /// ```
    pub fn leading_ones(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_max()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_ones(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_ones()
    }

    // pub fn leading_zeros(&self) -> u32
    /// Returns the number of leading zeros in the binary representation of 
    /// the `BigUInt`.
    /// 
    /// # Returns
    /// The count of consecutive bits set to 0 starting from the Most 
    /// Significant Bit (MSB).
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = "100000000000000000000000000000000000000000000000000000000000000000000000000".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} leading zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_zeros());
    /// assert_eq!(a_biguint.leading_zeros(), 10);
    /// ```
    pub fn leading_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_zero()
                { res += T::size_in_bits().into_u32(); }
            else
                { return res + self.get_num_(i).leading_zeros(); }
            i -= 1;
        }
        res + self.get_num_(0).leading_zeros()
    }

    // pub fn trailing_ones(&self) -> u32
    /// Returns the number of trailing ones in the binary representation of 
    /// the `BigUInt`.
    /// 
    /// # Returns
    /// The count of consecutive bits set to 1 starting from the Least 
    /// Significant Bit (LSB).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use std::str::FromStr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("111111111111111111111111111111111111111111111111111111111111111111111111111111").unwrap();
    /// println!("{} is {} in binary and has {} leading ones in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_ones());
    /// assert_eq!(a_biguint.trailing_ones(), 3);
    /// ```
    pub fn trailing_ones(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_max()
            {
                res += T::size_in_bits().into_u32();
            }
            else
            {
                res += self.get_num_(i).trailing_ones();
                break;
            }
        }
        res
    }

    // pub fn trailing_zeros(&self) -> u32
    /// Returns the number of trailing zeros in the binary representation of 
    /// the `BigUInt`.
    /// 
    /// # Returns
    /// The count of consecutive bits set to 0 starting from the Least 
    /// Significant Bit (LSB).
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = "111111111111111111111111111111111111111111111111111111111111111111111111111111".parse::<U256>().unwrap();
    /// println!("{} is {} in binary and has {} leading zeros in binary.", a_biguint, a_biguint.to_string_with_radix_and_stride(2, 10).unwrap(), a_biguint.leading_zeros());
    /// assert_eq!(a_biguint.trailing_zeros(), 0);
    /// ```
    pub fn trailing_zeros(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_zero()
            {
                res += T::size_in_bits().into_u32();
            }
            else
            {
                res += self.get_num_(i).trailing_zeros();
                break;
            }
        }
        res
    }

    // pub fn leading_max_elements(&self) -> u32
    /// Returns the number of leading storage elements that are set to their 
    /// maximum possible value.
    /// 
    /// # Returns
    /// The count of consecutive elements of type `T` that have all bits set 
    /// to 1, starting from the high-order end.
    /// 
    /// # Implementation Details
    /// An element is considered "maximum" if it equals `T::max()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999_88888888", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} leading max elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 2).unwrap(), a_biguint.leading_max_elements());
    /// assert_eq!(a_biguint.leading_max_elements(), 4);
    /// ```
    pub fn leading_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_max()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0).is_max()
            { res + 1 }
        else
            { res }
    }

    // pub fn leading_zero_elements(&self) -> u32
    /// Returns the number of leading storage elements that are set to zero.
    /// 
    /// # Returns
    /// The count of consecutive elements of type `T` that are set to 0, 
    /// starting from the high-order end.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00000000_FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_99999999", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} leading zero elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), a_biguint.leading_zero_elements());
    /// assert_eq!(a_biguint.leading_zero_elements(), 1);
    /// ```
    pub fn leading_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        let mut i = N-1;
        while i != 0
        {
            if self.get_num_(i).is_zero()
                { res += 1; }
            else
                { return res; }
            i -= 1;
        }
        if self.get_num_(0).is_zero()
            { res + 1 }
        else
            { res }
    }

    // pub fn trailing_max_elements(&self) -> u32
    /// Returns the number of trailing storage elements that are set to their 
    /// maximum possible value.
    /// 
    /// # Returns
    /// The count of consecutive elements of type `T` that have all bits set 
    /// to 1, starting from the low-order end.
    /// 
    /// # Implementation Details
    /// An element is considered "maximum" if it equals `T::max()`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("88888888_99999999_AAAAAAAA_BBBBBBBB_CCCCCCCC_DDDDDDDD_EEEEEEEE_FFFFFFFF", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} trailing max elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 4).unwrap(),a_biguint.trailing_max_elements());
    /// assert_eq!(a_biguint.trailing_max_elements(), 2);
    /// ```
    pub fn trailing_max_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_max()
                { res += 1; }
            else
                { return res; }
        }
        res
    }

    // pub fn trailing_zero_elements(&self) -> u32
    /// Returns the number of trailing storage elements that are set to zero.
    /// 
    /// # Returns
    /// The count of consecutive elements of type `T` that are set to 0, 
    /// starting from the low-order end.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("FFFFFFFF_EEEEEEEE_DDDDDDDD_CCCCCCCC_BBBBBBBB_AAAAAAAA_9999999_900000000", 16).unwrap();
    /// println!("{} is {} in hexadecimal and has {} trailing zero elements in array.", a_biguint, a_biguint.to_string_with_radix_and_stride(16, 2).unwrap(),a_biguint.trailing_zero_elements());
    /// assert_eq!(a_biguint.trailing_zero_elements(), 4);
    /// ```
    pub fn trailing_zero_elements(&self) -> u32
    {
        let mut res = 0_u32;
        for i in 0..N
        {
            if self.get_num_(i).is_zero()
                { res += 1; }
            else
                { return res; }
        }
        res
    }



    /***** METHODS FOR COMPARISON WITH UINT *****/

    // pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    /// Compares the `BigUInt` instance with a primitive unsigned integer.
    ///
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    ///
    /// # Returns
    /// * `Some(Ordering::Greater)` if `self` is greater than `other`.
    /// * `Some(Ordering::Less)` if `self` is less than `other`.
    /// * `Some(Ordering::Equal)` if `self` is equal to `other`.
    ///
    /// # Implementation Details
    /// This method is optimized for comparing with small constant values. 
    /// For values of the same type `T`, using standard comparison operators 
    /// (`<`, `>`, etc.) is recommended for better readability.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let res = UU32::from_uint(100_u8).partial_cmp_uint(90_u128).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("100 > 90"); }
    ///     Ordering::Less => { println!("100 < 90"); }
    ///     Ordering::Equal => { println!("100 = 90"); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.partial_cmp_uint)
    pub fn partial_cmp_uint<U>(&self, other: U) -> Option<Ordering>
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.partial_cmp(&Self::from_uint(other));
        }

        // if U::size_in_bytes() <= T::size_in_bytes()
        let t_other = T::num::<U>(other);
        if self.get_num_(0) > t_other
        {
            return Some(Ordering::Greater);
        }
        else if self.get_num_(0) < t_other
        {
            for idx in 1..N
            {
                if self.get_num_(idx) != T::zero()
                    { return Some(Ordering::Greater); }
            }
            return Some(Ordering::Less);
        }
        else    // if self.number[0] == other
        {
            for idx in 1..N
            {
                if self.get_num_(idx) != T::zero()
                    { return Some(Ordering::Greater); }
            }
        }
        Some(Ordering::Equal)
    }

    // pub fn lt_uint<U>(&self, other: U) -> bool
    /// Checks if the `BigUInt` instance is less than a primitive unsigned 
    /// integer.
    /// 
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is strictly less than `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint.lt_uint(b_uint);
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.lt_uint)
    #[inline]
    pub fn lt_uint<U>(&self, other: U) -> bool
    where U: TraitsBigUInt<U>
    {
        self.partial_cmp_uint(other).unwrap().is_lt()
    }

    // pub fn gt_uint<U>(&self, other: U) -> bool
    /// Checks if the `BigUInt` instance is greater than a primitive unsigned 
    /// integer.
    /// 
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is strictly greater than `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint.gt_uint(b_uint);
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.gt_uint)
    #[inline]
    pub fn gt_uint<U>(&self, other: U) -> bool
    where U: TraitsBigUInt<U>
    {
        self.partial_cmp_uint(other).unwrap().is_gt()
    }

    // pub fn le_uint<U>(&self, other: U) -> bool
    /// Checks if the `BigUInt` instance is less than or equal to a primitive 
    /// unsigned integer.
    /// 
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is less than or equal to `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint.le_uint(b_uint);
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.le_uint)
    #[inline]
    pub fn le_uint<U>(&self, other: U) -> bool
    where U: TraitsBigUInt<U>
    {
        self.partial_cmp_uint(other).unwrap().is_le()
    }

    // pub fn ge_uint<U>(&self, other: U) -> bool 
    /// Checks if the `BigUInt` instance is greater than or equal to a 
    /// primitive unsigned integer.
    /// 
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is greater than or equal to `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint.ge_uint(b_uint);
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.ge_uint)
    #[inline]
    pub fn ge_uint<U>(&self, other: U) -> bool
    where U: TraitsBigUInt<U>
    {
        self.partial_cmp_uint(other).unwrap().is_ge()
    }

    // pub fn eq_uint<U>(&self, other: U) -> bool
    /// Checks if the `BigUInt` instance is equal to a primitive unsigned 
    /// integer.
    /// 
    /// # Arguments
    /// * `other`: The primitive unsigned integer value to compare against.
    /// 
    /// # Returns
    /// * `true` if the magnitude of the number is equal to `other`.
    /// * `false` otherwise.
    /// 
    /// # Implementation Details
    /// This method is optimized for comparing a `BigUInt` with small 
    /// constant values and provides functionality similar to `is_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint.eq_uint(b_uint);
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.eq_uint)
    pub fn eq_uint<U>(&self, other: U) -> bool
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { self.eq(&Self::from_uint(other)) }
        else // if U::size_in_bytes() <= T::size_in_bytes()
            { self.partial_cmp_uint(other).unwrap().is_eq() }
    }

    // pub fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    /// Compares two `BigUInt` instances.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `Some(Ordering::Greater)` if `self` is greater than `other`.
    /// * `Some(Ordering::Less)` if `self` is less than `other`.
    /// * `Some(Ordering::Equal)` if `self` is equal to `other`.
    /// 
    /// # Example 1
    /// ```
    /// use std::cmp::Ordering;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str1 = "70000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num_str2 = "60000000000000000000000000000000000000000000000000000000000000000000000000000";
    /// let num1 = num_str1.parse::<UU32>().unwrap();
    /// let num2 = num_str2.parse::<UU32>().unwrap();
    /// 
    /// let res = num1.partial_cmp(&num2).unwrap();
    /// match res
    /// {
    ///     Ordering::Greater => { println!("{} > {}", num1, num2); }
    ///     Ordering::Less => { println!("{} < {}", num1, num2); }
    ///     Ordering::Equal => { println!("{} = {}", num1, num2); }
    /// }
    /// assert_eq!(res, Ordering::Greater);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.partial_cmp)
    #[cfg(target_endian = "little")]
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        let mut idx = N - 1;
        loop
        {
            if self.get_num_(idx) > other.get_num_(idx)
                { return Some(Ordering::Greater); }
            else if self.get_num_(idx) < other.get_num_(idx)
                { return Some(Ordering::Less); }
            if idx == 0
                { break; }
            idx -= 1;
        }
        Some(Ordering::Equal)
    }

    // pub fn lt(&self, other: &Self) -> bool
    /// Checks if the `BigUInt` instance is less than another `BigUInt`.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is strictly less than `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.lt(&b_biguint);
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.lt)
    #[inline]
    pub fn lt(&self, other: &Self) -> bool
    {
        self.partial_cmp(other).unwrap().is_lt()
    }

    // pub fn gt(&self, other: &Self) -> bool
    /// Checks if the `BigUInt` instance is greater than another `BigUInt`.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is strictly greater than `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.gt(&b_biguint);
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.gt)
    #[inline]
    pub fn gt(&self, other: &Self) -> bool
    {
        self.partial_cmp(other).unwrap().is_gt()
    }

    // pub fn le(&self, other: &Self) -> bool
    /// Checks if the `BigUInt` instance is less than or equal to another 
    /// `BigUInt`.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is less than or equal to `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.le(&b_biguint);
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.le)
    #[inline]
    pub fn le(&self, other: &Self) -> bool
    {
        self.partial_cmp(other).unwrap().is_le()
    }

    // pub fn ge(&self, other: &Self) -> bool
    /// Checks if the `BigUInt` instance is greater than or equal to another 
    /// `BigUInt`.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `true` if `self` is greater than or equal to `other`.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint.ge(&b_biguint);
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.ge)
    #[inline]
    pub fn ge(&self, other: &Self) -> bool
    {
        self.partial_cmp(other).unwrap().is_ge()
    }

    // pub fn eq(&self, other: &Self) -> bool
    /// Checks if the `BigUInt` instance is equal to another `BigUInt`.
    /// 
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    /// 
    /// # Returns
    /// * `true` if all internal storage elements are equal.
    /// * `false` otherwise.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint.eq(&b_biguint);
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.eq)
    pub fn eq(&self, other: &Self) -> bool
    {
        for idx in 0..N
        {
            if self.get_num_(idx) != other.get_num_(idx)
                { return false; }
        }
        true
    }



    /***** ARITHMATIC OPERATIONS WITH UINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    /// Calculates `self + rhs + carry` and returns the result with a 
    /// carry-out bit.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    /// * `carry`: The carry-in bit to include in the addition.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the sum and the carry-out bit.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The carry-out bit is also 
    /// reflected in the `OVERFLOW` flag of the returned instance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = a_biguint.carrying_add_uint(num_uint, false);
    /// println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722605");
    /// assert_eq!(carry, false);
    /// assert_eq!(sum.is_overflow(), false);
    /// assert_eq!(sum.is_underflow(), false);
    /// assert_eq!(sum.is_divided_by_zero(), false);
    /// assert_eq!(sum.is_infinity(), false);
    /// assert_eq!(sum.is_undefined(), false);
    /// assert_eq!(sum.is_left_carry(), false);
    /// assert_eq!(sum.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.carrying_add_uint)
    pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        carrying_calc!(self, Self::carrying_add_assign_uint, rhs, carry);
    }

    // pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    /// Calculates `self + rhs + carry` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    /// * `carry`: The carry-in bit to include in the addition.
    ///
    /// # Returns
    /// The carry-out bit resulting from the addition.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The `OVERFLOW` flag of `self` 
    /// is updated to reflect the carry-out bit. Flags are cumulative and 
    /// will remain set if they were set by a previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let carry = a_biguint.carrying_add_assign_uint(num_uint, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302573");
    /// assert_eq!(carry, false);
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.carrying_add_assign_uint)
    pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.carrying_add_assign(&Self::from_uint(rhs), carry);
        }

        let flags = self.get_all_flags();
        self.reset_all_flags();
        // if rhs.length_in_bytes() <= T::size_in_bytes()
        let zero = T::zero();
        let (mut num, mut c) = self.get_num_(0).carrying_add(T::num::<U>(rhs), carry);
        self.set_num_(0, num);
        if c
        {
            for i in 1..N
            {
                (num, c) = self.get_num_(i).carrying_add(zero, c);
                self.set_num_(i, num);
                if !c
                    { break; }
            }
            if c
                { self.set_overflow(); }
        }
        self.set_flag_bit(flags);
        c
    }

    // pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self + rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the sum.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) addition. If the operation 
    /// results in an overflow, the `OVERFLOW` flag of the result is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.wrapping_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_add_uint)
    pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.wrapping_add(&Self::from_uint(rhs))
        }
        else // if U::size_in_bytes() <= T::size_in_bytes()
        {
            let (res, _) = self.carrying_add_uint(rhs, false);
            res
        }
    }

    // pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self + rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) addition. If an overflow 
    /// occurs, the `OVERFLOW` flag of `self` is set. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_add_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU64::max());
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_add_assign_uint)
    #[inline]
    pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
    where U: TraitsBigUInt<U>
    {
        self.carrying_add_assign_uint(rhs, false);
    }

    // pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self + rhs` and returns a tuple with the result and an 
    /// overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the sum and a boolean indicating 
    /// whether an overflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The boolean flag reflects 
    /// only the overflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_uint)
    pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc!(self, Self::overflowing_add_assign_uint, rhs);
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self + rhs` and assigns the result to `self`, returning 
    /// an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to add.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The returned flag reflects 
    /// only the current operation, while the `OVERFLOW` flag of `self` 
    /// remains cumulative.
    /// 
    /// # Alternatives
    /// For operands larger than 128 bits, use 
    /// [`overflowing_add_assign()`](struct@BigUInt#method.overflowing_add_assign).
    /// 
    /// # Example 1
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_assign_uint)
    pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_add_assign_uint, rhs);
    }


    /*** SUBTRACTION ***/

    // pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    /// Calculates `self - rhs - borrow` and returns the result with a 
    /// borrow-out bit.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    /// * `borrow`: The borrow-in bit to include in the subtraction.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the difference and the borrow-out 
    /// bit.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The borrow-out bit is also 
    /// reflected in the `UNDERFLOW` flag of the returned instance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, false);
    /// println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    /// assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528175");
    /// assert_eq!(borrow, false);
    /// assert_eq!(dif.is_underflow(), false);
    /// assert_eq!(dif.is_overflow(), false);
    /// assert_eq!(dif.is_divided_by_zero(), false);
    /// assert_eq!(dif.is_infinity(), false);
    /// assert_eq!(dif.is_undefined(), false);
    /// assert_eq!(dif.is_left_carry(), false);
    /// assert_eq!(dif.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.borrowing_sub_uint)
    pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        carrying_calc!(self, Self::borrowing_sub_assign_uint, rhs, borrow);
    }

    // pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    /// Calculates `self - rhs - borrow` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    /// * `borrow`: The borrow-in bit to include in the subtraction.
    ///
    /// # Returns
    /// The borrow-out bit resulting from the subtraction.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The `UNDERFLOW` flag of 
    /// `self` is updated to reflect the borrow-out bit. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, false);
    /// println!("After a_biguint -= {}, a_biguint = {}\tborrow = {}", num_uint, a_biguint, borrow);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948207");
    /// assert_eq!(borrow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.borrowing_sub_assign_uint)
    pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            return self.borrowing_sub_assign(&Self::from_uint(rhs), borrow);
        }

        let mut flags = self.get_all_flags();
        // if U::size_in_bytes() <= T::size_in_bytes()
        let (mut num, mut b) = self.get_num_(0).borrowing_sub(T::num::<U>(rhs), borrow);
        self.set_num_(0, num);
        if b
        {
            for i in 1..N
            {
                (num, b) = self.get_num_(i).borrowing_sub(T::zero(), b);
                self.set_num_(i, num);
                if !b
                    { break; }
            }
            if b
                { flags |= Self::UNDERFLOW; }
        }
        self.set_all_flags(flags);
        b
    }

    // pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self - rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the difference.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) subtraction. If the operation 
    /// results in an underflow, the `UNDERFLOW` flag of the result is set.
    /// 
    /// # Alternatives
    /// * If `rhs` is bigger tham `ui128`, the method
    ///   [wrapping_sub()](struct@BigUInt#method.wrapping_sub)
    ///   is proper rather than this method.
    /// * You may be interested in extra subtraction methods.
    ///   In order to use any one of
    ///   [checked_sub_uint()](trait_big_uint_more/trait.BigUInt_More.html#tymethod.checked_sub_uint),
    ///   [unchecked_sub_uint()](trait_big_uint_more/trait.BigUInt_More.html#tymethod.unchecked_sub_uint),
    ///   [saturating_sub_uint()](trait_big_uint_more/trait.BigUInt_More.html#tymethod.saturating_sub_uint), and
    ///   [safe_sub_uint()](trait_big_uint_more/trait.BigUInt_More.html#tymethod.safe_sub_uint),
    ///   you need to import (use) the trait `BigUInt_More`.
    ///   In order to use any one of
    ///   [modular_sub_uint()](trait.BigUInt_Modular.html#tymethod.modular_sub_uint),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    ///   In order to use any one of
    ///   [panic_free_modular_sub_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_sub_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub_uint(1_u8);
    /// println!("{} - 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_sub_uint)
    pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.wrapping_sub(&Self::from_uint(rhs))
        }
        else // if U::size_in_bytes() <= T::size_in_bytes()
        {
            let (res, _) = self.borrowing_sub_uint(rhs, false);
            res
        }
    }

    // pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self - rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) subtraction. If an underflow 
    /// occurs, the `UNDERFLOW` flag of `self` is set. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_sub_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_sub_assign_uint)
    #[inline]
    pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
    where U: TraitsBigUInt<U>
    {
        self.borrowing_sub_assign_uint(rhs, false);
    }

    // pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self - rhs` and returns a tuple with the result and an 
    /// underflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the difference and a boolean 
    /// indicating whether an underflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The boolean flag reflects 
    /// only the underflow from the current operation. If underflow occurs, 
    /// the `UNDERFLOW` flag of the result is set.
    /// 
    /// # Alternatives
    /// For divisors larger than 128 bits, use 
    /// [`overflowing_sub()`](struct@BigUInt#method.overflowing_sub).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let (res, underflow) = a_biguint.overflowing_sub_uint(1_u8);
    /// println!("{} - 1 = {}\nunderflow = {}", a_biguint, res, underflow);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(underflow, false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_sub_uint)
    pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc!(self, Self::overflowing_sub_assign_uint, rhs);
    }

    // pub fn overflowing_sub_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self - rhs` and assigns the result to `self`, returning 
    /// an underflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to subtract.
    ///
    /// # Returns
    /// `true` if an underflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The returned flag reflects 
    /// only the current operation, while the `UNDERFLOW` flag of `self` 
    /// remains cumulative.
    /// 
    /// # Alternatives
    /// For divisors larger than 128 bits, use 
    /// [`overflowing_sub_assign()`](struct@BigUInt#method.overflowing_sub_assign).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign_uint(1_u8);
    /// println!("After a_biguint.overflowing_sub_assign_uint(1_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_sub_assign_uint)
    pub fn overflowing_sub_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        underflowing_calc_assign!(self, Self::wrapping_sub_assign_uint, rhs);
    }

    // pub fn abs_diff_uint<U>(&self, other: U) -> Self
    /// Calculates the absolute difference between `self` and a primitive 
    /// unsigned integer.
    ///
    /// # Arguments
    /// * `other`: The primitive unsigned integer to compare against.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the absolute difference 
    /// `|self - other|`.
    ///
    /// # Implementation Details
    /// This method does not modify status flags such as `OVERFLOW` or 
    /// `UNDERFLOW`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    /// let res = a_biguint.abs_diff_uint(num_uint);
    /// println!("| {} - {} | = {}", a_biguint, num_uint, res);
    /// assert_eq!(res.to_string(), "115792089237316195423570985008687907853066609319396769656704041438214461985024");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.abs_diff_uint)
    pub fn abs_diff_uint<U>(&self, other: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
        {
            self.abs_diff(&Self::from_uint(other))
        }
        else // if rhs.length_in_bytes() <= T::size_in_bytes()
        {
            let t_other = T::num::<U>(other);
            if self.lt_uint(t_other)
                { Self::from_uint(t_other - self.get_num_(0)) }
            else
                { self.wrapping_sub_uint(t_other) }
        }
    }



    /*** MULTIPLICATION ***/

    // pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    /// Calculates `self * rhs + carry` and returns the result as a tuple 
    /// of (low, high) parts.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    /// * `carry`: An additional `BigUInt` value to add to the product.
    ///
    /// # Returns
    /// A tuple `(Self, Self)` representing the 512-bit result (for a 256-bit 
    /// `BigUInt`), where the first element is the low-order part and the 
    /// second is the high-order part.
    ///
    /// # Implementation Details
    /// This method performs long multiplication. If the high-order part of 
    /// the result is non-zero, the `OVERFLOW` flag of the low-order part 
    /// instance will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_low_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_high_biguint = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// let (res_low, res_high) = a_low_biguint.carrying_mul_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// let (res_high, res_higher) = a_high_biguint.carrying_mul_uint(b_uint, res_high);
    /// println!("{}:{} X {} = {}:{}:{}", a_high_biguint, a_low_biguint, b_uint, res_higher, res_high, res_low);
    /// assert_eq!(res_higher.to_string(), "0");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// assert_eq!(res_low.is_overflow(), false);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.carrying_mul_uint)
    pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    where U: TraitsBigUInt<U>
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.carrying_mul_assign_uint(rhs, carry);
        (low, high)
    }

    // pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    /// Calculates `self * rhs + carry` and assigns the low-order part to 
    /// `self`, returning the high-order part.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    /// * `carry`: An additional `BigUInt` value to add to the product.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the high-order (overflow) bits 
    /// of the calculation.
    ///
    /// # Implementation Details
    /// This method performs long multiplication. If the returned high-order 
    /// part is non-zero, the `OVERFLOW` flag of `self` will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_low_biguint = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_high_biguint = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// 
    /// println!("Originally, a_low_biguint = {}", a_low_biguint);
    /// assert_eq!(a_low_biguint.is_overflow(), false);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// println!("Originally, a_high_biguint = {}\n", a_high_biguint);
    /// assert_eq!(a_high_biguint.is_overflow(), false);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// 
    /// let res_high = a_low_biguint.carrying_mul_assign_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// println!("After a_low_biguint.carrying_mul_assign_uint(225_u8, 0),\na_low_biguint = {}", a_low_biguint);
    /// assert_eq!(a_low_biguint.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// assert_eq!(a_low_biguint.is_overflow(), false);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// let res_higher = a_high_biguint.carrying_mul_assign_uint(b_uint, res_high);
    /// println!("After a_high_biguint.carrying_mul_assign_uint(225_u8, res_higher),\na_high_biguint = {}\nres_higher = {}", a_high_biguint, res_higher);
    /// assert_eq!(res_higher.to_string(), "0");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(a_high_biguint.is_overflow(), false);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.carrying_mul_assign_uint)
    pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    where U: TraitsBigUInt<U>
    {
        let mut high = self.widening_mul_assign_uint(rhs);
        if self.overflowing_add_assign(&carry)
            { high.wrapping_add_assign_uint(1_u8); }
        high
    }

    // pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    /// Calculates `self * rhs` and returns the result as a tuple of 
    /// (low, high) parts.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Returns
    /// A tuple `(Self, Self)` representing the full product, where the first 
    /// element is the low-order part and the second is the high-order part.
    ///
    /// # Implementation Details
    /// This method performs long multiplication. If the high-order part of 
    /// the result is non-zero, the `OVERFLOW` flag of the low-order part 
    /// instance will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    /// println!("{} X {} = {}:{}", a_biguint, b_uint, res_high, res_low);
    /// assert_eq!(res_high.to_string(), "1");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res_low.is_overflow(), true);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.widening_mul_uint)
    pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    where U: TraitsBigUInt<U>
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.widening_mul_assign_uint(rhs);
        (low, high)
    }

    // pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    /// Calculates `self * rhs` and assigns the low-order part to `self`, 
    /// returning the high-order part.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the high-order (overflow) bits 
    /// of the product.
    ///
    /// # Implementation Details
    /// This method performs long multiplication. If the returned high-order 
    /// part is non-zero, the `OVERFLOW` flag of `self` will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u64;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let res_high = a_biguint.widening_mul_assign_uint(b_uint);
    /// println!("After a_biguint.widening_mul_assign_uint(248_u8),\na_biguint = {}\nres_high = {}", a_biguint, res_high);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res_high.to_string(), "1");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    ///     
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.widening_mul_assign_uint)
    pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { self.widening_mul_assign(&Self::from_uint(rhs)) }
        else // if U::size_in_bytes() <= T::size_in_bytes()
            { (Self::method_widening_mul_assign_uint)(self, T::num::<U>(rhs)) }
    }

    // Using carrying_mul()
    fn widening_mul_assign_uint_1(&mut self, rhs: T) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let zero = T::zero();
        let i_n = N-self.leading_zero_elements() as usize;
        let mut lower;
        let mut higher = zero;
        for i in 0..i_n
        {
            (lower, higher) = self.get_num_(i).carrying_mul(rhs, higher);
            self.set_num_(i, lower);
        }
        if !higher.is_zero()
        {
            if i_n < N
            {
                self.set_num_(i_n, higher);
            }
            else
            {
                self.set_overflow();
                high.set_num_(0, higher);
            }
        }
        high
    }

    // Using shift_left()
    fn widening_mul_assign_uint_2(&mut self, rhs: T) -> Self
    {
        let mut high = Self::zero();
        if rhs.is_zero()
        {
            self.set_zero();
            return high;
        }
        if self.is_zero()
            { return high; }

        let mut rhs = rhs;
        let mut adder = self.clone();
        self.set_zero();
        loop
        {
            if rhs.is_odd()
            {
                if self.overflowing_add_assign(&adder)
                    { high.wrapping_add_assign_uint(1_u8); }
            }
            rhs >>= T::one();
            if rhs == T::zero()
                { break; }
            adder.shift_left_assign(1_u8);
        }
        high
    }

    // pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self * rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the product.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) multiplication. If the operation 
    /// results in an overflow, the `OVERFLOW` flag of the result is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u16;
    /// let res = a_biguint.wrapping_mul_uint(b_uint);
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_mul_uint)
    pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::wrapping_mul_assign_uint, rhs);
    }

    // pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self * rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) multiplication. If an overflow 
    /// occurs, the `OVERFLOW` flag of `self` is set. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u16;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "12380187429816690342769003185818648605085375388281194656994643364900608");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_mul_assign_uint(b_uint);
    /// println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_mul_assign_uint)
    pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
    where U: TraitsBigUInt<U>
    {
        if U::size_in_bytes() > T::size_in_bytes()
            { self.wrapping_mul_assign(&Self::from_uint(rhs)) }
        else // if U::size_in_bytes() <= T::size_in_bytes()
            { (Self::method_wrapping_mul_assign_uint)(self, T::num::<U>(rhs)) }
    }

    // Using carrying_mul()
    fn wrapping_mul_assign_uint_1(&mut self, rhs: T)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let zero = T::zero();
        let mut lower;
        let mut higher = zero;
        let i_n = N - self.leading_zero_elements() as usize;
        for i in 0..i_n
        {
            (lower, higher) = self.get_num_(i).carrying_mul(rhs, higher);
            self.set_num_(i, lower);
        }
        if !higher.is_zero()
        {
            if i_n < N
                { self.set_num_(i_n, higher); }
            else
                { self.set_overflow(); }
        }
    }

    // Using shift_left()
    fn wrapping_mul_assign_uint_2(&mut self, rhs: T)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let mut trhs = rhs;
        let mut adder = Self::from_array(self.get_number().clone());
        self.set_zero();
        loop
        {
            if trhs.is_odd()
                { self.wrapping_add_assign(&adder); }
            trhs >>= T::one();
            if trhs.is_zero()
                { break; }
            adder.shift_left_assign(1_u8);
        }
        if adder.is_overflow()
            { self.set_overflow(); }
    }

    // pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self * rhs` and returns a tuple with the result and an 
    /// overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the product and a boolean 
    /// indicating whether an overflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication. The boolean flag 
    /// reflects only the overflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    /// println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    /// assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_mul_uint)
    pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc!(self, Self::overflowing_mul_assign_uint, rhs);
    }

    // pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self * rhs` and assigns the result to `self`, returning 
    /// an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer to multiply by.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication. The returned flag 
    /// reflects only the current operation, while the `OVERFLOW` flag of 
    /// `self` remains cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    /// println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
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
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_mul_assign_uint)
    pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_mul_assign_uint, rhs);
    }

    /*
    // pub fn expanding_mul<U, const M: usize>(&self, rhs: U) -> BigUInt<T, M>
    // where U: SmallUInt + Copy + Clone + Display + Debug + ToString
    //         + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
    //         + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
    //         + Rem<Output=U> + RemAssign
    //         + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
    //         + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
    //         + BitXor<Output=U> + BitXorAssign + Not<Output=U>
    //         + PartialEq + PartialOrd
    // {

    //     let (low, high) = self.widening_mul_uint(rhs);
    //     low.into_biguint::<T, M>()
    //     BigUInt::<T, M>::new()
    // }
    */


    /*** DIVISION ***/

    pub(super) fn common_divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    where U: TraitsBigUInt<U>
    {
        if self.is_zero()
        {
            (Self::zero(), U::zero())
        }
        else if self.lt_uint(rhs)
        {
            (Self::zero(), SharedValues::<U, T>::from_src(self.get_num_(0)).get_des())
        }
        else if self.eq_uint(rhs)
        {
            (Self::one(), U::zero())
        }
        else if U::size_in_bytes() <= T::size_in_bytes()
        {
            let mut quotient = Self::zero();
            let size_rhs = rhs.length_in_bits() - rhs.leading_zeros();
            let size_self = self.length_in_bits() - self.leading_zeros();
            let mut remainder = SharedValues::<U, T>::from_src(self.get_upper_portion(size_rhs).get_num_(0)).get_des();
            let mut position = size_self - size_rhs;
            loop
            {
                if remainder >= rhs
                {
                    quotient.set_lsb();
                    remainder = remainder.wrapping_sub(rhs);
                }
                if position == 0
                    { break; }
                position -= 1;
                quotient.shift_left_assign(1_u8);
                remainder <<= U::one();
                if self.is_bit_set_(position)
                    { remainder.set_lsb(); }
            }
            (quotient, remainder)
        }
        else    // if U::size_in_bytes() > T::size_in_bytes()
        {
            let (quotient, remainder) = self.divide_fully(&Self::from_uint(rhs));
            (quotient, remainder.into_uint::<U>())
        }
    }

    // pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    /// Divides `self` by a primitive unsigned integer and returns both the 
    /// quotient and the remainder.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// A tuple `(Self, U)` where the first element is the quotient of type 
    /// `BigUInt` and the second is the remainder of type `U`.
    ///
    /// # Implementation Details
    /// This method is the fundamental building block for other division and 
    /// remainder operations with primitive integers. It handles the 
    /// multi-precision division logic internally.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.divide_fully_uint)
    pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
    where U: TraitsBigUInt<U>
    {
        if rhs.is_zero()
            { panic!(); }
        self.common_divide_fully_uint(rhs)
    }

    // pub fn wrapping_div_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by a primitive unsigned integer and returns the 
    /// quotient.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the quotient.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// provides standard division behavior consistent with other "wrapping" 
    /// methods.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let quotient = dividend.wrapping_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_div_uint)
    pub fn wrapping_div_uint<U>(&self, rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc_div!(self, Self::divide_fully_uint, rhs);
    }

    // pub fn wrapping_div_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by a primitive unsigned integer and assigns the 
    /// quotient to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Implementation Details
    /// This method performs in-place division. Status flags are cumulative 
    /// and will remain set if they were set by a previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_div_assign_uint)
    pub fn wrapping_div_assign_uint<U>(&mut self, rhs: U)
    where U: TraitsBigUInt<U>
    {
        biguint_calc_to_calc_assign!(self, Self::wrapping_div_uint, rhs);
    }

    // pub fn overflowing_div_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Divides `self` by a primitive unsigned integer and returns a tuple 
    /// with the quotient and an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the quotient and a boolean 
    /// indicating whether an overflow occurred.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, the returned 
    /// boolean flag is always `false`. This method is provided for 
    /// consistency with other overflowing operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, overflow) = dividend.overflowing_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_div_uint)
    pub fn overflowing_div_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_div!(self, Self::divide_fully_uint, rhs);
    }

    // pub fn overflowing_div_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Divides `self` by a primitive unsigned integer and assigns the 
    /// quotient to `self`, returning an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// always returns `false`. This method is provided for consistency with 
    /// other overflowing operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_div_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_div_assign_uint)
    pub fn overflowing_div_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_div_assign_uint, rhs);
    }

    // pub fn wrapping_rem_uint<U>(&self, rhs: U) -> U
    /// Divides `self` by a primitive unsigned integer and returns the 
    /// remainder.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// The remainder of type `U` resulting from the division.
    ///
    /// # Implementation Details
    /// Since the remainder of a division by a primitive integer `U` always 
    /// fits within `U`, this method returns the result as a primitive type 
    /// rather than a `BigUInt`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let remainder = dividend.wrapping_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_rem_uint)
    pub fn wrapping_rem_uint<U>(&self, rhs: U) -> U
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc_rem!(self, Self::divide_fully_uint, rhs);
    }

    // pub fn wrapping_rem_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by a primitive unsigned integer and assigns the 
    /// remainder to `self`.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Implementation Details
    /// After the calculation, `self` will be updated to store the remainder 
    /// as its new value. Status flags are cumulative and will remain set if 
    /// they were set by a previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 87_u8;
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.wrapping_rem_assign_uint)
    pub fn wrapping_rem_assign_uint<U>(&mut self, rhs: U)
    where U: TraitsBigUInt<U>
    {
        let flags = self.get_all_flags();
        let (_, remainder) = self.divide_fully_uint(rhs);
        self.set_uint(remainder);
        self.set_all_flags(flags);
    }

    // pub fn overflowing_rem_uint<U>(&self, rhs: U) -> (U, bool)
    /// Divides `self` by a primitive unsigned integer and returns a tuple 
    /// with the remainder and an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// A tuple `(U, bool)` containing the remainder and a boolean indicating 
    /// whether an overflow occurred.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, the returned 
    /// boolean flag is always `false`. This method is provided for 
    /// consistency with other overflowing operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 8);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_rem_uint)
    pub fn overflowing_rem_uint<U>(&self, rhs: U) -> (U, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_rem!(self, Self::divide_fully_uint, rhs);
    }

    // pub fn overflowing_rem_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Divides `self` by a primitive unsigned integer and assigns the 
    /// remainder to `self`, returning an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: The primitive unsigned integer divisor.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// always returns `false`. This method is provided for consistency with 
    /// other overflowing operations. Status flags are cumulative.
    /// 
    /// # Alternatives
    /// For divisors larger than 128 bits, use 
    /// [`overflowing_rem_assign()`](struct@BigUInt#method.overflowing_rem_assign).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u16;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_rem_assign_uint)
    pub fn overflowing_rem_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        self.wrapping_rem_assign_uint(rhs);
        false
    }



    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH UINT *****/

    // pub fn pow_uint<U>(&self, exp: U) -> Self
    /// Raises `self` to the power of `exp`.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the result of `self` raised to the 
    /// power of `exp`.
    ///
    /// # Implementation Details
    /// This method uses the binary exponentiation (exponentiation by 
    /// squaring) algorithm for efficient calculation. The operation performs 
    /// wrapping multiplication.
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let res = a_biguint.pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.pow_uint)
    pub fn pow_uint<U>(&self, exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::pow_assign_uint, exp);
    }

    // pub fn pow_assign_uint<U>(&mut self, exp: U)
    /// Raises `self` to the power of `exp` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Implementation Details
    /// This method performs in-place binary exponentiation. The operation 
    /// uses wrapping multiplication. Status flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// a_biguint.pow_assign_uint(exp);
    /// println!("After a_biguint.pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.pow_assign_uint)
    pub fn pow_assign_uint<U>(&mut self, exp: U)
    where U: TraitsBigUInt<U>
    {
        general_pow_assign!(self, Self::common_pow_assign_uint, exp);
    }

    // pub fn wrapping_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `self` to the power of `exp` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the result.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) exponentiation. If the 
    /// operation results in an overflow, the `OVERFLOW` flag of the result 
    /// is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u32;
    /// let mut res = a_biguint.wrapping_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.wrapping_pow_uint)
    pub fn wrapping_pow_uint<U>(&self, exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::wrapping_pow_assign_uint, exp);
    }

    // pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    /// Raises `self` to the power of `exp` with wrapping and assigns the 
    /// result to `self`.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) exponentiation in-place. If 
    /// an overflow occurs, the `OVERFLOW` flag of `self` is set. Flags are 
    /// cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// a_biguint.wrapping_pow_assign_uint(exp);
    /// println!("After a_biguint.wrapping_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.wrapping_pow_assign_uint)
    pub fn wrapping_pow_assign_uint<U>(&mut self, exp: U)
    where U: TraitsBigUInt<U>
    {
        general_pow_assign!(self, Self::common_pow_assign_uint, exp);
    }

    pub(super) fn common_pow_assign_uint<U>(&mut self, exp: U)
    where U: TraitsBigUInt<U>
    {
        if self.is_zero_or_one()
            { return; }

        let zero = U::zero();
        let one = U::one();
        let multiplier = self.clone();
        self.set_one();
        if exp.is_zero()
            { return; }

        let mut bit_check = one << U::u32_as_smalluint(exp.length_in_bits() - 1 - exp.leading_zeros());
        if !bit_check.is_zero()
        {
            self.wrapping_mul_assign(&multiplier);
            bit_check >>= one;
        }
        while !bit_check.is_zero()
        {
            self.wrapping_mul_assign(&self.clone());
            if (bit_check & exp) != zero
                { self.wrapping_mul_assign(&multiplier); }
            bit_check >>= one;
        }
    }

    // pub fn overflowing_pow_uint<U>(&self, exp: U) -> (Self, bool)
    /// Raises `self` to the power of `exp` and returns a tuple with the 
    /// result and an overflow flag.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the result and a boolean indicating 
    /// whether an overflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication. The boolean flag 
    /// reflects only the overflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let (res, overflow) = a_biguint.overflowing_pow_uint(exp);
    /// println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res, overflow);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(overflow, false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.overflowing_pow_uint)
    pub fn overflowing_pow_uint<U>(&self, exp: U) -> (Self, bool)
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc!(self, Self::overflowing_pow_assign_uint, exp);
    }
    
    // pub fn overflowing_pow_assign_uint<U>(&mut self, exp: U) -> bool
    /// Raises `self` to the power of `exp` and assigns the result to `self`, 
    /// returning an overflow flag.
    ///
    /// # Arguments
    /// * `exp`: The primitive unsigned integer exponent.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication in-place. The returned 
    /// flag reflects only the current operation, while the `OVERFLOW` flag 
    /// of `self` remains cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// let overflow = a_biguint.overflowing_pow_assign_uint(exp);
    /// println!("After a_biguint.overflowing_pow_assign_uint({}), a_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.overflowing_pow_assign_uint)
    pub fn overflowing_pow_assign_uint<U>(&mut self, exp: U) -> bool
    where U: TraitsBigUInt<U>
    {
        biguint_overflowing_calc_assign!(self, Self::pow_assign_uint, exp);
    }

    // pub fn iroot_uint<U>(&self, exp: U) -> Self
    /// Calculates the integer n-th root of `self`.
    /// 
    /// # Arguments
    /// * `exp`: The primitive unsigned integer root to calculate.
    /// 
    /// # Returns
    /// A new `BigUInt` instance containing the integer n-th root.
    /// 
    /// # Implementation Details
    /// This method calculates the largest integer `x` such that `x^exp <= self`. 
    /// The operation is guaranteed not to overflow.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 8_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "100000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.iroot_uint)
    pub fn iroot_uint<U>(&self, exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        general_calc_iroot!(self, Self::common_iroot_uint, exp);
    }

    // pub fn iroot_assign_uint<U>(&mut self, exp: U)
    /// Calculates the integer n-th root of `self` and assigns it to `self`.
    /// 
    /// # Arguments
    /// * `exp`: The primitive unsigned integer root to calculate.
    /// 
    /// # Implementation Details
    /// This method calculates the largest integer `x` such that `x^exp <= self` 
    /// and performs an in-place update. The operation is guaranteed not to 
    /// overflow. Status flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 8_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "100000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.iroot_assign_uint)
    pub fn iroot_assign_uint<U>(&mut self, exp: U)
    where U: TraitsBigUInt<U>
    {
        biguint_calc_to_calc_assign!(self, Self::iroot_uint, exp);
    }

    pub(super) fn common_iroot_uint<U>(&self, exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        let mut highest = (Self::size_in_bits() - self.leading_zeros()).wrapping_div(exp.into_u32());
        if highest.is_zero()
        {
            return Self::one();
        }
        let mut high;
        let mut low;
        let mut mid;
        let mut adder;
        let mut sum;
        let mut res = Self::zero();
        let maximum = highest.wrapping_sub(1);
        loop
        {
            high = highest;
            low = 0;
            if high == 0
            {
                return res;
            }
            else    // if high > 0
            {
                loop
                {
                    mid = (high + low) >> 1;
                    adder = Self::generate_check_bits_(mid);
                    sum = res.wrapping_add(&adder);
                    let (sq, b_overflow) = sum.overflowing_pow_uint(exp);
                    if !b_overflow && (sq < *self)
                    {
                        if mid == maximum
                        {
                            res = sum;
                            break;
                        }
                        else if mid == low
                        { 
                            res = sum;
                            if mid == 0
                                { highest = 0; }
                            break;
                        }
                        low = mid;
                    }
                    else if b_overflow || (sq > *self)
                    {
                        if mid == low
                        { 
                            highest = mid;
                            break;
                        }
                        high = mid;
                    }
                    else    // if sq == self
                    {
                        return sum;
                    }
                }
            }
        }
    }

    // pub fn ilog_uint<U>(&self, base: U) -> Self
    /// Calculates the integer logarithm of `self` with the specified base.
    /// 
    /// # Arguments
    /// * `base`: The primitive unsigned integer base for the logarithm. 
    ///   Must be greater than 1.
    /// 
    /// # Returns
    /// A new `BigUInt` instance containing the floor of the logarithm.
    /// 
    /// # Implementation Details
    /// This method calculates the largest integer `x` such that `base^x <= self`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// let res = a_biguint.ilog_uint(base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog_uint)
    pub fn ilog_uint<U>(&self, base: U) -> Self
    where U: TraitsBigUInt<U>
    {
        general_calc_ilog!(self, Self::common_ilog_uint, base);
    }

    // pub fn ilog_assign_uint<U>(&mut self, base: U)
    /// Calculates the integer logarithm of `self` with the specified base 
    /// and assigns the result to `self`.
    /// 
    /// # Arguments
    /// * `base`: The primitive unsigned integer base for the logarithm. 
    ///   Must be greater than 1.
    /// 
    /// # Implementation Details
    /// This method calculates the largest integer `x` such that `base^x <= self` 
    /// and performs an in-place update. Status flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// a_biguint.ilog_assign_uint(base);
    /// println!("After a_biguint.ilog_assign_uint({}), a_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog_assign_uint)
    pub fn ilog_assign_uint<U>(&mut self, base: U)
    where U: TraitsBigUInt<U>
    {
        biguint_calc_to_calc_assign!(self, Self::ilog_uint, base);
    }

    pub(super) fn common_ilog_uint<U>(&self, base: U) -> Self
    where U: TraitsBigUInt<U>
    {
        general_calc_common_ilog!(self, Self::wrapping_div_assign_uint, base);
    }



    
    /***** ARITHMATIC OPERATIONS WITH BIGUINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    /// Calculates `self + rhs + carry` and returns the result with a 
    /// carry-out bit.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    /// * `carry`: The carry-in bit to include in the addition.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the sum and the carry-out bit.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The carry-out bit is also 
    /// reflected in the `OVERFLOW` flag of the returned instance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    /// let a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// let (c_biguint_lo, carry) = a_biguint_lo.carrying_add(&b_biguint_lo, false);
    /// let (c_biguint_hi, overflow) = a_biguint_hi.carrying_add(&b_biguint_hi, carry);
    /// println!("{}:{} + {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    /// assert_eq!(overflow, false);
    /// assert_eq!(c_biguint_hi.is_overflow(), false);
    /// assert_eq!(c_biguint_hi.is_underflow(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    /// assert_eq!(carry, true);
    /// assert_eq!(c_biguint_lo.is_overflow(), true);
    /// assert_eq!(c_biguint_lo.is_underflow(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// ``` 
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.carrying_add)
    pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    {
        carrying_calc!(self, Self::carrying_add_assign, rhs, carry);
    }

    // pub fn carrying_add_assign(&mut self, rhs: &Self, carry: bool) -> bool
    /// Calculates `self + rhs + carry` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    /// * `carry`: The carry-in bit to include in the addition.
    ///
    /// # Returns
    /// The carry-out bit resulting from the addition.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The `OVERFLOW` flag of `self` 
    /// is updated to reflect the carry-out bit. Flags are cumulative and 
    /// will remain set if they were set by a previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    /// let mut a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// println!("Originally, a_biguint_hi = {}\na_biguint_lo = {}\nb_biguint_hi = {}\nb_biguint_lo = {}", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// print!("Operation is: {}:{} + {}:{} ", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// 
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// 
    /// let carry = a_biguint_lo.carrying_add_assign(&b_biguint_lo, false);
    /// let overflow = a_biguint_hi.carrying_add_assign(&b_biguint_hi, carry);
    /// 
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// println!("After a_biguint_lo.carrying_add_assign(&b_biguint_lo, false), a_biguint_lo = {}", a_biguint_lo);
    /// println!("After a_biguint_hi.carrying_add_assign(&b_biguint_hi, {}), a_biguint_hi = {}", carry, a_biguint_hi);
    /// 
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_biguint_lo.is_overflow(), true);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.carrying_add_assign)
    pub fn carrying_add_assign(&mut self, rhs: &Self, carry: bool) -> bool
    {
        let mut c = carry;
        let mut num;
        let i_l = self.leading_zero_elements() as usize;
        let j_l = rhs.leading_zero_elements() as usize;
        let ij_n = N - if i_l < j_l {i_l} else {j_l};
        for i in 0..ij_n
        {
            (num, c) = self.get_num_(i).carrying_add(rhs.get_num_(i), c);
            self.set_num_(i, num);
        }
        if c
        {
            if ij_n < N
            {
                self.set_num_(ij_n, T::one());
                c = false;
            }
            else
            {
                self.set_overflow();
            }
        }
        c
    }

    // pub fn wrapping_add(&self, rhs: &Self) -> Self
    /// Calculates `self + rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the sum.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) addition. If the operation 
    /// results in an overflow, the `OVERFLOW` flag of the result is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.wrapping_add(&one_biguint);
    /// println!("{} + {} = {}", a_biguint, one_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_add)
    pub fn wrapping_add(&self, rhs: &Self) -> Self
    {
        let (res, _) = self.carrying_add(rhs, false); 
        res
    }

    // pub fn wrapping_add_assign(&mut self, rhs: &Self)
    /// Calculates `self + rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) addition. If an overflow 
    /// occurs, the `OVERFLOW` flag of `self` is set. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one_biguint = U512::one();
    /// a_biguint.wrapping_add_assign(&one_biguint);
    /// println!("After a_biguint.wrapping_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U512::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_add_assign)
    #[inline]
    pub fn wrapping_add_assign(&mut self, rhs: &Self)
    {
        self.carrying_add_assign(rhs, false);
    }

    // pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self + rhs` and returns a tuple with the result and an 
    /// overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the sum and a boolean indicating 
    /// whether an overflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The boolean flag reflects 
    /// only the overflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let (res, overflow) = a_biguint.overflowing_add(&one_biguint);
    /// println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, res, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_add)
    pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc!(self, Self::overflowing_add_assign, rhs);
    }

    // pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self + rhs` and assigns the result to `self`, returning 
    /// an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to add.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping addition. The returned flag reflects 
    /// only the current operation, while the `OVERFLOW` flag of `self` 
    /// remains cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one_biguint = U512::one();
    /// let overflow = a_biguint.overflowing_add_assign(&one_biguint);
    /// println!("After a_biguint.overflowing_add_assign(&U512::one()), a_biguint = {}, overflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint, U512::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_add_assign)
    pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_add_assign, rhs);
    }



    /*** SUBTRACTION ***/

    // pub fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool)
    /// Calculates `self - rhs - borrow` and returns the result with a 
    /// borrow-out bit.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    /// * `borrow`: The borrow-in bit to include in the subtraction.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the difference and the borrow-out 
    /// bit.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The borrow-out bit is also 
    /// reflected in the `UNDERFLOW` flag of the returned instance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    /// let a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// let (c_biguint_lo, borrow) = a_biguint_lo.borrowing_sub(&b_biguint_lo, false);
    /// let (c_biguint_hi, unerflow) = a_biguint_hi.borrowing_sub(&b_biguint_hi, borrow);
    /// 
    /// println!("{}:{} - {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, overflow = {}", borrow, unerflow);
    /// assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    /// assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    /// assert_eq!(borrow, true);
    /// assert_eq!(c_biguint_lo.is_underflow(), true);
    /// assert_eq!(c_biguint_lo.is_overflow(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(unerflow, false);
    /// assert_eq!(c_biguint_hi.is_underflow(), false);
    /// assert_eq!(c_biguint_hi.is_overflow(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.borrowing_sub)
    pub fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool)
    {
        carrying_calc!(self, Self::borrowing_sub_assign, rhs, borrow);
    }

    // pub fn borrowing_sub_assign(&mut self, rhs: &Self, borrow: bool) -> bool
    /// Calculates `self - rhs - borrow` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    /// * `borrow`: The borrow-in bit to include in the subtraction.
    ///
    /// # Returns
    /// The borrow-out bit resulting from the subtraction.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The `UNDERFLOW` flag of 
    /// `self` is updated to reflect the borrow-out bit. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    /// let mut a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    /// let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, underflow = {}", borrow, underflow);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    /// assert_eq!(borrow, true);
    /// assert_eq!(a_biguint_lo.is_underflow(), true);
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.borrowing_sub_assign)
    pub fn borrowing_sub_assign(&mut self, rhs: &Self, borrow: bool) -> bool
    {
        let mut	b = borrow;
        let mut num;
        let i_l = self.leading_zero_elements() as usize;
        let j_l = rhs.leading_zero_elements() as usize;
        let ij_n = if i_l < j_l {N-i_l} else {N};
        for i in 0..ij_n
        {
            (num, b) = self.get_num_(i).borrowing_sub(rhs.get_num_(i), b);
            self.set_num_(i, num);
        }
        if b
        {
            for i in ij_n..N
                { self.set_num_(i, T::max()); }
            self.set_underflow();
        }
        b
    }

    // pub fn wrapping_sub(&self, rhs: &Self) -> Self
    /// Calculates `self - rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the difference.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) subtraction. If the operation 
    /// results in an underflow, the `UNDERFLOW` flag of the result is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub(&U512::one());
    /// println!("{} - 1 = {}", a_biguint, res);
    /// assert_eq!(res, U512::zero());
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_sub)
    pub fn wrapping_sub(&self, rhs: &Self) -> Self
    {
        let (res, _) = self.borrowing_sub(rhs, false);
        res
    }

    // pub fn wrapping_sub_assign(&mut self, rhs: &Self)
    /// Calculates `self - rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) subtraction. If an underflow 
    /// occurs, the `UNDERFLOW` flag of `self` is set. Flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_sub_assign(&U512::one());
    /// println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U512::zero());
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_sub_assign)
    #[inline]
    pub fn wrapping_sub_assign(&mut self, rhs: &Self)
    {
        self.borrowing_sub_assign(rhs, false);
    }

    // pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self - rhs` and returns a tuple with the result and an 
    /// underflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the difference and a boolean 
    /// indicating whether an underflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The boolean flag reflects 
    /// only the underflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let (res, underflow) = a_biguint.overflowing_sub(&U512::one());
    /// println!("{} - 1 = {}, underflow = {}", a_biguint, res, underflow);
    /// assert_eq!(underflow, false);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_sub)
    pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc!(self, Self::overflowing_sub_assign, rhs);
    }

    // pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self - rhs` and assigns the result to `self`, returning 
    /// an underflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to subtract.
    ///
    /// # Returns
    /// `true` if an underflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping subtraction. The returned flag reflects 
    /// only the current operation, while the `UNDERFLOW` flag of `self` 
    /// remains cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign(&U512::one());
    /// println!("After a_biguint.overflowing_sub_assign(&U512::one()), a_biguint = {}, underflow = {}", a_biguint, underflow);
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_sub_assign)
    pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    {
        underflowing_calc_assign!(self, Self::wrapping_sub_assign, rhs);
    }

    // pub fn abs_diff(&self, other: &Self) -> Self
    /// Calculates the absolute difference between `self` and another 
    /// `BigUInt`.
    ///
    /// # Arguments
    /// * `other`: A reference to the `BigUInt` to compare against.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the absolute difference 
    /// `|self - other|`.
    ///
    /// # Implementation Details
    /// This method does not modify status flags such as `OVERFLOW` or 
    /// `UNDERFLOW`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    /// let b_biguint = U256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    /// let c_biguint = a_biguint.abs_diff(&b_biguint);
    /// let d_biguint = b_biguint.abs_diff(&a_biguint);
    /// println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c_biguint);
    /// println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d_biguint);
    /// assert_eq!(c_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(d_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(d_biguint.is_overflow(), false);
    /// assert_eq!(d_biguint.is_underflow(), false);
    /// assert_eq!(d_biguint.is_divided_by_zero(), false);
    /// assert_eq!(d_biguint.is_infinity(), false);
    /// assert_eq!(d_biguint.is_undefined(), false);
    /// assert_eq!(d_biguint.is_left_carry(), false);
    /// assert_eq!(d_biguint.is_right_carry(), false);
    /// ```
    pub fn abs_diff(&self, other: &Self) -> Self
    {
        if self < other
            { other.wrapping_sub(self) }
        else
            { self.wrapping_sub(other) }
    }



    /*** MULTIPLICATION ***/

    // pub fn carrying_mul(&self, rhs: &Self, carry: Self) -> (Self, Self)
    /// Calculates `self * rhs + carry` and returns the result as a tuple 
    /// of (low, high) parts.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    /// * `carry`: An additional `BigUInt` value to add to the product.
    ///
    /// # Returns
    /// A tuple `(Self, Self)` representing the full double-width result, 
    /// where the first element is the low-order part and the second is the 
    /// high-order part.
    ///
    /// # Implementation Details
    /// This method performs multi-precision multiplication. If the high-order 
    /// part of the result is non-zero, the `OVERFLOW` flag of the low-order 
    /// part instance will be set.
    /// 
    /// # Alternatives
    /// For operands larger than 128 bits, use 
    /// [`carrying_mul_uint()`](struct@BigUInt#method.carrying_mul_uint) 
    /// for better performance.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_low = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint_high = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_biguint = UU32::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    /// let (res_biguint_low, res_biguint_high) = a_biguint_low.carrying_mul(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// let (res_biguint_high, res_biguint_higher) = a_biguint_high.carrying_mul(&b_biguint, res_biguint_high);
    /// 
    /// println!("{}:{} X {} = {}:{}:{}", a_biguint_high, a_biguint_low, b_biguint, res_biguint_higher, res_biguint_high, res_biguint_low);
    /// assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    /// assert_eq!(res_biguint_high.is_overflow(), true);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.carrying_mul)
    pub fn carrying_mul(&self, rhs: &Self, carry: Self) -> (Self, Self)
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.carrying_mul_assign(rhs, carry);
        (low, high)
    }

    // pub fn carrying_mul_assign(&mut self, rhs: &Self, carry: Self) -> Self
    /// Calculates `self * rhs + carry` and assigns the low-order part to 
    /// `self`, returning the high-order part.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    /// * `carry`: An additional `BigUInt` value to add to the product.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the high-order (overflow) bits 
    /// of the calculation.
    ///
    /// # Implementation Details
    /// This method performs multi-precision multiplication. If the returned 
    /// high-order part is non-zero, the `OVERFLOW` flag of `self` will be set. 
    /// Status flags are cumulative.
    /// 
    /// # Alternatives
    /// For operands larger than 128 bits, use 
    /// [`carrying_mul_assign_uint()`](struct@BigUInt#method.carrying_mul_assign_uint) 
    /// for better performance.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_low = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint_high = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_biguint = U256::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    /// println!("Originally, a_biguint_low = {}\na_biguint_high = {}", a_biguint_low, a_biguint_high);
    /// assert_eq!(a_biguint_low.is_overflow(), false);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.is_overflow(), false);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_higher = a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high);
    /// println!("After a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero()),\na_biguint_low = {}", a_biguint_low);
    /// println!("After a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high),\na_biguint_high = {}", a_biguint_high);
    /// println!("res_biguint_higher = {}", res_biguint_higher);
    /// assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    /// assert_eq!(a_biguint_high.is_overflow(), true);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    /// assert_eq!(a_biguint_low.is_overflow(), true);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.carrying_mul_assign)
    pub fn carrying_mul_assign(&mut self, rhs: &Self, carry: Self) -> Self
    {
        let mut high = self.widening_mul_assign(rhs);
        if self.overflowing_add_assign(&carry)
            { high.wrapping_add_assign_uint(1_u8); }
        high
    }

    // pub fn widening_mul(&self, rhs: &Self) -> (Self, Self)
    /// Calculates `self * rhs` and returns the full product as a tuple 
    /// of (low, high) parts.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// A tuple `(Self, Self)` representing the full double-width product, 
    /// where the first element is the low-order part and the second is the 
    /// high-order part.
    ///
    /// # Implementation Details
    /// This method performs multi-precision multiplication. If the high-order 
    /// part of the result is non-zero, the `OVERFLOW` flag of the low-order 
    /// part instance will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    /// let (res_biguint_low, res_biguint_high) = a_biguint.widening_mul(&b_biguint);
    /// 
    /// println!("{} X {} = {}:{}", a_biguint, b_biguint, res_biguint_high, res_biguint_low);
    /// assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(reres_biguint_highs.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.widening_mul)
    pub fn widening_mul(&self, rhs: &Self) -> (Self, Self)
    {
        let mut low = Self::from_array(self.get_number().clone());
        let high = low.widening_mul_assign(rhs);
        (low, high)
    }

    // pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    /// Calculates `self * rhs` and assigns the low-order part to `self`, 
    /// returning the high-order part.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// A new `BigUInt` instance representing the high-order (overflow) bits 
    /// of the product.
    ///
    /// # Implementation Details
    /// This method performs multi-precision multiplication. If the returned 
    /// high-order part is non-zero, the `OVERFLOW` flag of `self` will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint.widening_mul_assign(&b_biguint);
    /// println!("After a_biguint.widening_mul_assign(&b_biguint),\na_biguint = {}\nres_biguint_high = {}", a_biguint, res_biguint_high);
    /// assert_eq!(a_biguint.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.widening_mul_assign)
    #[inline]
    pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    {
        (Self::method_widening_mul_assign)(self, rhs)
    }

    fn widening_mul_assign_1(&mut self, rhs: &Self) -> Self
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return Self::zero();
        }
        if self.is_zero()
            { return Self::zero(); }

        let operand = self.clone();
        self.set_zero();
        let zero = T::zero();
        let mut high_biguint = Self::zero();
        let mut low_uint: T;
        let i_n = N - rhs.leading_zero_elements() as usize;
        let j_n = N - operand.leading_zero_elements() as usize;
        for i in 0..i_n
        {
            let mut high_uint = zero;
            for j in 0..j_n
            {
                let operandd_j = operand.get_num_(j);
                let rhs_i = rhs.get_num_(i);
                (low_uint, high_uint) = operandd_j.carrying_mul(rhs_i, high_uint);
                let mut ij = i + j;
                let num_biguint: &mut Self;
                if ij < N
                {
                    num_biguint = self;
                }
                else
                {
                    ij -= N;
                    num_biguint = &mut high_biguint;
                }
                let num_uint_ij = num_biguint.get_num_(ij);
                let (num, overflow) = num_uint_ij.overflowing_add(low_uint);
                num_biguint.set_num_(ij, num);
                if overflow
                    { high_uint = high_uint.wrapping_add(T::one()); }
            }
            let c = i + j_n;
            if c < N
                { self.set_num_(c, high_uint); }
            else
                { high_biguint.set_num_(c - N, high_uint); }
        }
        if !high_biguint.is_zero()
            { self.set_overflow(); }
        high_biguint
    }

    fn widening_mul_assign_2(&mut self, rhs: &Self) -> Self
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return Self::zero();
        }
        if self.is_zero()
            { return Self::zero(); }

        let flags = self.get_all_flags();
        let adder = self.clone();
        let size_t_bits_minus_one = T::size_in_bits() - 1;
        let mut high = Self::zero();
        let mut chunk = N - 1 - rhs.leading_zero_elements() as usize;
        let mut piece = T::size_in_bits() - 1 - rhs.get_num_(chunk).leading_zeros();
        self.set_zero();
        self.reset_all_flags();
        loop
        {
            let num = rhs.get_num_(chunk);
            if num.is_zero()
            {
                self.shift_left_assign(size_t_bits_minus_one);
            }
            else
            {
                loop
                {
                    if num.is_bit_set_(piece)
                    {
                        if self.overflowing_add_assign(&adder)
                            { high.wrapping_add_assign_uint(1_u8); }
                    }
                    if piece == 0
                        { break; }
                    piece -= 1;
                    high.shift_left_assign(1_u8);
                    if self.is_msb_set()
                        { high.set_lsb(); }
                    self.shift_left_assign(1_u8);
                }
            }
            if chunk == 0
                { break; }
            chunk -= 1;
            high.shift_left_assign(1_u8);
            if self.is_msb_set()
                { high.set_lsb(); }
            self.shift_left_assign(1_u8);
            piece = T::size_in_bits() - 1;
        }
        if self.is_left_carry()
        {
            self.reset_left_carry();
            self.set_overflow();
        }
        self.set_flag_bit(flags);
        if high.is_left_carry()
        {
            high.reset_left_carry();
            high.set_overflow();
        }
        high
    }

    // pub fn wrapping_mul(&self, rhs: &Self) -> Self
    /// Calculates `self * rhs` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the product.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) multiplication. If the 
    /// operation results in an overflow, the `OVERFLOW` flag of the result 
    /// is set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.wrapping_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_mul)
    pub fn wrapping_mul(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::wrapping_mul_assign, rhs);
    }

    // pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    /// Calculates `self * rhs` with wrapping and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) multiplication in-place. If 
    /// an overflow occurs, the `OVERFLOW` flag of `self` is set. Flags are 
    /// cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_mul_assign(&b_biguint);
    /// println!("After a_biguint.wrapping_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_mul_assign)
    #[inline]
    pub fn wrapping_mul_assign(&mut self, rhs: &Self)
    {
        (Self::method_wrapping_mul_assign)(self, rhs);
    }

    fn wrapping_mul_assign_1(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let operand = Self::from_array(self.get_number().clone());
        let zero = T::zero();
        let one = T::one();
        let i_n = N - rhs.leading_zero_elements() as usize;
        let j_n = N - operand.leading_zero_elements() as usize;
        let mut lower;
        let mut higher;
        let mut sum;
        let mut overflow;
        let mut ij = 0_usize;
        self.set_zero();
        for i in 0..i_n
        {
            higher = zero;
            for j in 0..j_n
            {
                ij = i + j;
                if ij >= N
                {
                    self.set_overflow();
                    ij -= 1;
                    break;
                }
                (lower, higher) = operand.get_num_(j).carrying_mul(rhs.get_num_(i), higher);
                (sum, overflow) = self.get_num_(ij).overflowing_add(lower);
                self.set_num_(ij, sum);
                if overflow
                    { higher += one; }
            }

            ij += 1;
            if !higher.is_zero()
            {
                if ij < N
                {
                    (sum, overflow) = self.get_num_(ij).overflowing_add(higher);
                    self.set_num_(ij, sum);
                    while overflow
                    {
                        ij += 1;
                        if ij >= N
                        {
                            self.set_overflow();
                            break;
                        }
                        (sum, overflow) = self.get_num_(ij).overflowing_add(one);
                        self.set_num_(ij, sum);
                    }
                }
                else
                {
                    self.set_overflow();
                }
            }
        }
    }

    fn wrapping_mul_assign_2(&mut self, rhs: &Self)
    {
        if rhs.is_zero()
        {
            self.set_zero();
            return;
        }
        if self.is_zero()
            { return; }

        let flags = self.get_all_flags();
        self.reset_all_flags();

        let adder = Self::from_array(self.get_number().clone());
        let size_t_bits_minus_one = T::size_in_bits()-1;
        let mut chunk = N - 1 - rhs.leading_zero_elements() as usize;
        let mut piece = T::size_in_bits() - 1 - rhs.get_num_(chunk).leading_zeros();
        self.set_zero();
        loop
        {
            let num = rhs.get_num_(chunk);
            if num.is_zero()
            {
                self.shift_left_assign(size_t_bits_minus_one);
            }
            else
            {
                loop
                {
                    if num.is_bit_set_(piece)
                        { self.wrapping_add_assign(&adder); }
                    if piece == 0
                        { break; }
                    piece -= 1;
                    self.shift_left_assign(1_u8);
                }
            }
            if chunk == 0
                { break; }
            chunk -= 1;
            self.shift_left_assign(1_u8);
            piece = T::size_in_bits() - 1;
        }
        if self.is_left_carry()
        {
            self.reset_left_carry();
            self.set_overflow();
        }
        self.set_flag_bit(flags);
    }

    // pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self * rhs` and returns a tuple with the result and an 
    /// overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the product and a boolean 
    /// indicating whether an overflow occurred.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication. The boolean flag 
    /// reflects only the overflow from the current operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let (res, overflow) = a_biguint.overflowing_mul(&b_biguint);
    /// println!("{} X {} = {}, {}", a_biguint, b_biguint, res, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_mul)
    pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc!(self, Self::overflowing_mul_assign, rhs);
    }

    // pub fn overflowing_mul_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self * rhs` and assigns the result to `self`, returning 
    /// an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// This method performs wrapping multiplication in-place. The returned 
    /// flag reflects only the current operation, while the `OVERFLOW` flag 
    /// of `self` remains cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_mul_assign(&b_biguint);
    /// println!("After a_biguint.overflowing_mul_assign(&b_biguint), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_mul_assign)
    pub fn overflowing_mul_assign(&mut self, rhs: &Self) -> bool
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_mul_assign, rhs);
    }

    // pub fn expanding_mul<const M: usize>(&self, rhs: &Self) -> BigUInt<T, M>
    /// Calculates `self * rhs` and returns the product as a `BigUInt` of a 
    /// different specified size.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to multiply by.
    ///
    /// # Returns
    /// A new `BigUInt<T, M>` instance containing the full or truncated 
    /// product.
    ///
    /// # Implementation Details
    /// This method performs multi-precision multiplication and fits the 
    /// result into a target width `M`. If the product exceeds the capacity 
    /// of `BigUInt<T, M>`, the `OVERFLOW` flag will be set.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    /// let res_biguint: U512 = a_biguint.expanding_mul(&b_biguint);
    /// 
    // /// println!("{} X {} = {}", a_biguint, b_biguint, res_biguint);
    // /// assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    // /// assert_eq!(res_biguint_high.is_overflow(), false);
    // /// assert_eq!(res_biguint_high.is_underflow(), false);
    // /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    // /// assert_eq!(res_biguint_high.is_infinity(), false);
    // /// assert_eq!(res_biguint_high.is_undefined(), false);
    // /// assert_eq!(res_biguint_high.is_left_carry(), false);
    // /// assert_eq!(reres_biguint_highs.is_right_carry(), false);
    // /// 
    // /// assert_eq!(res_biguint_low.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    // /// assert_eq!(res_biguint_low.is_overflow(), true);
    // /// assert_eq!(res_biguint_low.is_underflow(), false);
    // /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    // /// assert_eq!(res_biguint_low.is_infinity(), false);
    // /// assert_eq!(res_biguint_low.is_undefined(), false);
    // /// assert_eq!(res_biguint_low.is_left_carry(), false);
    // /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    // /// 
    // /// # For more examples,
    // /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.widening_mul)
    pub fn expanding_mul<const M: usize>(&self, rhs: &Self) -> BigUInt<T, M>
    {
        let mut target = BigUInt::<T, M>::new();
        if M > N
        {
            let (low, high) = self.widening_mul(rhs);
            let k = if M < 2 * N {M - N} else {N};
            unsafe { copy_nonoverlapping(low.get_number().as_ptr(), target.get_number_mut().as_mut_ptr(), N); }
            unsafe { copy_nonoverlapping(high.get_number().as_ptr(), target.get_number_mut().as_mut_ptr().add(N), k); }
            for i in k..N
            {
                if !high.get_num_(i).is_zero()
                {
                    target.set_overflow();
                    break;
                }
            }
        }
        else
        {
            let res = self.wrapping_mul(rhs);
            unsafe { copy_nonoverlapping(res.get_number().as_ptr(), target.get_number_mut().as_mut_ptr(), M); }
            target.set_all_flags(res.get_all_flags());
            for i in M..N
            {
                if !res.get_num_(i).is_zero()
                {
                    target.set_overflow();
                    break;
                }
            }
        }
        target
    }


    /*** DIVISION ***/

    pub(super) fn common_divide_fully(&self, rhs: &Self) -> (Self, Self)
    {
        if self.is_zero()
            { return (Self::zero(), Self::zero()); }
        else if self.lt(rhs)
            { return (Self::zero(), self.clone()); }
        else if self.eq(rhs)
            { return (Self::one(), Self::zero()); }

        let mut quotient = Self::zero();
        let size_rhs = rhs.length_in_bits() - rhs.leading_zeros();
        let size_self = self.length_in_bits() - self.leading_zeros();
        let mut remainder = self.get_upper_portion(size_rhs);
        remainder.reset_all_flags();
        let mut position = size_self - size_rhs;
        loop
        {
            if *rhs <= remainder
            {
                quotient.set_lsb();
                remainder.wrapping_sub_assign(rhs); 
            }
            if position == 0
                { break; }
            position -= 1;
            quotient.shift_left_assign(1_u8);
            remainder.shift_left_assign(1_u8);
            if self.is_bit_set_(position)
                { remainder.set_lsb(); }
        }
        (quotient, remainder)
    }

    // pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    /// Divides `self` by another `BigUInt` and returns both the quotient and 
    /// the remainder.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// A tuple `(Self, Self)` where the first element is the quotient and the 
    /// second is the remainder.
    ///
    /// # Implementation Details
    /// This method is the core building block for other division and 
    /// remainder operations. It handles full multi-precision division logic.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (quotient, remainder) = dividend.divide_fully(&divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.divide_fully)
    pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    {
        if rhs.is_zero()
            { panic!(); }
        self.common_divide_fully(rhs)
    }

    // pub fn wrapping_div(&self, rhs: &Self) -> Self
    /// Divides `self` by another `BigUInt` and returns the quotient.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the quotient.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// provides standard division behavior consistent with other "wrapping" 
    /// methods.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.wrapping_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_div)
    pub fn wrapping_div(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_div!(self, Self::divide_fully, rhs);
    }

    // pub fn wrapping_div_assign(&mut self, rhs: &Self)
    /// Divides `self` by another `BigUInt` and assigns the quotient to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Implementation Details
    /// This method performs in-place division. Status flags are cumulative 
    /// and will remain set if they were set by a previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// a_biguint.wrapping_div_assign(&divisor);
    /// println!("After a_biguint.wrapping_div_assign(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_div_assign)
    pub fn wrapping_div_assign(&mut self, rhs: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::wrapping_div, rhs);
    }

    // pub fn overflowing_div(&self, rhs: &Self) -> (Self, bool)
    /// Divides `self` by another `BigUInt` and returns a tuple with the 
    /// quotient and an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the quotient and a boolean 
    /// indicating whether an overflow occurred.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, the returned 
    /// boolean flag is always `false`. This method is provided for 
    /// consistency with other overflowing operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let (quotient, overflow) = dividend.overflowing_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_div)
    pub fn overflowing_div(&self, rhs: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc_div!(self, Self::divide_fully, rhs);
    }

    // pub fn overflowing_div_assign(&mut self, rhs: &Self) -> bool
    /// Divides `self` by another `BigUInt` and assigns the result to 
    /// `self`, returning an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// always returns `false`. This method is provided for consistency with 
    /// other overflowing operations. Status flags are cumulative.
    /// 
    /// # Alternatives
    /// For divisors larger than 128 bits, use 
    /// [`overflowing_div_assign()`](struct@BigUInt#method.overflowing_div_assign).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_div_assign(&divisor);
    /// println!("After a_biguint.overflowing_div_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_div_assign)
    pub fn overflowing_div_assign(&mut self, rhs: &Self) -> bool
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_div_assign, rhs);
    }

    // pub fn wrapping_rem(&self, rhs: &Self) -> Self
    /// Divides `self` by another `BigUInt` and returns the remainder.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the remainder.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// provides standard remainder behavior consistent with other "wrapping" 
    /// methods.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.wrapping_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_rem)
    pub fn wrapping_rem(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc_rem!(self, Self::divide_fully, rhs);
    }

    // pub fn wrapping_rem_assign(&mut self, rhs: &Self)
    /// Divides `self` by another `BigUInt` and assigns the remainder to `self`.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Implementation Details
    /// This method performs in-place calculation of the remainder. Status 
    /// flags are cumulative and will remain set if they were set by a 
    /// previous operation.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint.wrapping_rem_assign(&divisor);
    /// println!("After a_biguint.wrapping_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.wrapping_rem_assign)
    pub fn wrapping_rem_assign(&mut self, rhs: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::wrapping_rem, rhs);
    }

    // pub fn overflowing_rem(&self, rhs: &Self) -> (Self, bool)
    /// Divides `self` by another `BigUInt` and returns a tuple with the 
    /// remainder and an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// A tuple `(Self, bool)` containing the remainder and a boolean 
    /// indicating whether an overflow occurred.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, the returned 
    /// boolean flag is always `false`. This method is provided for 
    /// consistency with other overflowing operations.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (remainder, overflow) = dividend.overflowing_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(overflow, false);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_rem)
    pub fn overflowing_rem(&self, rhs: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc_rem!(self, Self::divide_fully, rhs);
    }

    // pub fn overflowing_rem_assign(&mut self, rhs: &Self) -> bool
    /// Divides `self` by another `BigUInt` and assigns the remainder to 
    /// `self`, returning an overflow flag.
    ///
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` divisor.
    ///
    /// # Returns
    /// `true` if an overflow occurred in this operation, `false` otherwise.
    ///
    /// # Implementation Details
    /// Since division between `BigUInt` types does not overflow, this method 
    /// always returns `false`. This method is provided for consistency with 
    /// other overflowing operations. Status flags are cumulative.
    /// 
    /// # Alternatives
    /// For divisors larger than 128 bits, use 
    /// [`overflowing_rem_assign_uint()`](struct@BigUInt#method.overflowing_rem_assign_uint) 
    /// for better performance if the divisor fits in a primitive type.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = UU32::from_uint(87_u8);
    /// let overflow = a_biguint.overflowing_rem_assign(&divisor);
    /// println!("After a_biguint.overflowing_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.overflowing_rem_assign)
    pub fn overflowing_rem_assign(&mut self, rhs: &Self) -> bool
    {
        biguint_overflowing_calc_assign!(self, Self::wrapping_rem_assign, rhs);
    }



    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH BIGUINT *****/

    // pub fn next_power_of_two(&self) -> Self
    /// Returns the smallest power of two greater than or equal to `self`.
    /// 
    /// # Returns
    /// A new `BigUInt` instance representing the next power of two.
    ///
    /// # Implementation Details
    /// If the resulting value overflows the current bit-width, it returns zero. 
    /// The `OVERFLOW` flag will be set in such cases.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.next_power_of_two();
    /// println!("The next power of two is {}.", res);
    /// assert_eq!(res.to_string(), "170141183460469231731687303715884105728");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.next_power_of_two)
    pub fn next_power_of_two(&self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::next_power_of_two_assign);
    }

    // pub fn next_power_of_two_assign(&mut self)
    /// Finds the smallest power of two greater than or equal to `self` 
    /// and assigns the result to `self`.
    ///
    /// # Implementation Details
    /// If the resulting value overflows the current bit-width, `self` is 
    /// reset to zero and the `OVERFLOW` flag is set. Status flags are 
    /// cumulative.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.next_power_of_two_assign();
    /// println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "170141183460469231731687303715884105728");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.next_power_of_two_assign)
    pub fn next_power_of_two_assign(&mut self)
    {
        if !self.is_power_of_two()
        {
            let flags = self.get_all_flags();
            let bit_pos = Self::size_in_bits() - 1 - self.leading_zeros();
            self.turn_check_bits(bit_pos);
            self.shift_left_assign(1_u8);
            if self.is_left_carry()
            {
                self.reset_left_carry();
                self.set_overflow();
            }
            self.set_flag_bit(flags);
        }
    }
    
    // pub fn is_power_of_two(&self) -> bool
    /// Checks if the `BigUInt` instance is a power of two.
    /// 
    /// # Returns
    /// * `true` if the number is exactly a power of two (e.g., 1, 2, 4, 8...).
    /// * `false` otherwise.
    ///
    /// # Implementation Details
    /// Zero is not considered a power of two. This method evaluates the 
    /// magnitude and does not consider status flags.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.is_power_of_two();
    /// println!("Is {} the power of two? - {}.", a_biguint, res);
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.is_power_of_two)
    #[inline]
    pub fn is_power_of_two(&self) -> bool
    {
        self.count_ones() <= 1
    }

    // pub fn pow(&self, exp: &Self) -> Self
    /// Raises `self` to the power of `exp`.
    ///
    /// # Arguments
    /// * `exp`: A reference to the `BigUInt` exponent.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the result of `self` raised to the 
    /// power of `exp`.
    ///
    /// # Implementation Details
    /// This method uses the binary exponentiation (exponentiation by 
    /// squaring) algorithm for efficient calculation. The operation performs 
    /// wrapping multiplication.
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.pow)
    pub fn pow(&self, exp: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::pow_assign, exp);
    }

    // pub fn pow_assign(&mut self, exp: &Self)
    /// Raises `self` to the power of `exp` and assigns the result to `self`.
    ///
    /// # Arguments
    /// * `exp`: A reference to the `BigUInt` exponent.
    ///
    /// # Implementation Details
    /// This method performs in-place binary exponentiation. The operation 
    /// uses wrapping multiplication. Status flags are cumulative.
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// a_biguint.pow_assign(&exp);
    /// println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.pow_assign)
    pub fn pow_assign(&mut self, exp: &Self)
    {
        general_pow_assign!(self, Self::common_pow_assign, exp);
    }

    pub(super) fn common_pow_assign(&mut self, exp: &Self)
    {
        if self.is_zero_or_one()
            { return; }

        let multiplier = self.clone();
        self.set_one();
        if exp.is_zero()
            { return; }

        let mut bit_check = Self::one();
        bit_check.shift_left_assign(exp.length_in_bits() - exp.leading_zeros() - 1);
        if !bit_check.is_zero()
        {
            self.wrapping_mul_assign(&multiplier); 
            bit_check.shift_right_assign(1_u8);
        }
        while !bit_check.is_zero()
        {
            *self = self.wrapping_mul(self);
            if !(bit_check.and(exp).is_zero())
                { self.wrapping_mul_assign(&multiplier); }
            bit_check.shift_right_assign(1_u8);
        }
    }

    // pub fn wrapping_pow(&self, exp: &Self) -> Self
    /// Raises `self` to the power of `exp` with wrapping at the type boundary.
    ///
    /// # Arguments
    /// * `exp`: A reference to the `BigUInt` exponent.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the result.
    ///
    /// # Implementation Details
    /// This method performs wrapping (modular) exponentiation. If the 
    /// operation results in an overflow, the `OVERFLOW` flag of the result 
    /// is set.
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.wrapping_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.wrapping_pow)
    pub fn wrapping_pow(&self, exp: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::wrapping_pow_assign, exp);
    }

    // pub fn wrapping_pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow() internally.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// - The method [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint)
    ///   is more efficient than this method `wrapping_pow_assign()` when the
    ///   exponent `exp` is primitive unsigned integral data type
    ///   such as u8, u16, u32, u64, and u128.
    ///   If `exp` is the primitive unsigned integral data type number, use
    ///   the method [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint).
    /// - You may be interested in extra exponentiation methods
    ///   In order to use 
    ///   [saturating_pow_assign()](trait_big_more/trait.BigUInt_More.html#tymethod.saturating_pow_assign),
    ///   you need to import (use) the trait `BigUInt_More`.
    ///   In order to use any one of
    ///   [modular_pow_assign()](trait.BigUInt_Modular.html#tymethod.modular_pow_assign),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    ///   In order to use any one of
    ///   [panic_free_modular_pow_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_pow_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// a_biguint.wrapping_pow_assign(&exp);
    /// println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.wrapping_pow_assign)
    pub fn wrapping_pow_assign(&mut self, exp: &Self)
    {
        general_pow_assign!(self, Self::common_pow_assign, exp);
    }

    // pub fn overflowing_pow(&self, exp: &Self) -> (Self, bool)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring, 
    /// wrapping around at the boundary of the
    /// type `Self`, and returns a tuple of the result along with
    /// a boolean indicating whether an overflow would occur.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns a tuple of the result of raising `self` to the power of `exp`,
    /// using exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self` along with a boolean
    /// indicating whether an arithmetic overflow would occur.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
    /// - If overflowing did not happen in the current operation, the second
    ///   element of the output tuple will be false even if the `OVERFLOW` flag
    ///   of `self` was already set because of previous operation of `self`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint)
    /// is a bit faster than this method `overflowing_pow()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let (res, overflow) = a_biguint.overflowing_pow(&exp);
    /// println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.overflowing_pow)
    pub fn overflowing_pow(&self, exp: &Self) -> (Self, bool)
    {
        biguint_overflowing_calc!(self, Self::overflowing_pow_assign, exp);
    }

    // pub fn overflowing_pow_assign(&mut self, exp: &Self) -> bool
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring, 
    /// wrapping around at the boundary of the type `Self`, and
    /// assigns the result to `self` back, and
    /// returns a boolean indicating whether an overflow would occur.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns bool indicating whether an overflow happened.
    /// It returns `true` if overflow happened. Otherwise, it returns `false`.
    /// 
    /// # Argument
    /// The argument `exp` is of `&Self` type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - If overflowing did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint)
    /// is a bit faster than this method `overflow_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let overflow = a_biguint.overflowing_pow_assign(&exp);
    /// println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.overflowing_pow_assign)
    pub fn overflowing_pow_assign(&mut self, exp: &Self) -> bool
    {
        biguint_overflowing_calc_assign!(self, Self::pow_assign, exp);
    }

    // pub fn iroot(&self, exp: &Self) -> Self
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned.
    /// 
    /// # Features
    /// If `exp` is greater than zero and `self` is greater than 1,
    /// the result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [iroot_uint()](struct@BigUInt#method.iroot_uint)
    /// is a bit faster than this method `iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [iroot_uint()](struct@BigUInt#method.iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "100000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_pow)
    pub fn iroot(&self, exp: &Self) -> Self
    {
        general_calc_iroot!(self, Self::common_iroot, exp);
    }

    // pub fn iroot_assign(&mut self, exp: &Self)
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Features
    /// - If the exact value of `exp`-th root of `self` can be expressed with
    ///   `Self`-typed unsigned integer, it will be assigned to `self`.
    ///   Otherwise, the `Self`-typed biggest unsigned integer that is less
    ///   than the exact value of `exp`-th root of `self` will be assigned
    ///   to `self`.
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// [iroot_assign_uint()](struct@BigUInt#method.iroot_assign_uint)
    /// is a bit faster than this method `iroot_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [iroot_assign_uint()](struct@BigUInt#method.iroot_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(8_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "100000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.iroot_assign)
    pub fn iroot_assign(&mut self, exp: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::iroot, exp);
    }

    pub(super) fn common_iroot(&self, exp: &Self) -> Self
    {
        if exp.gt_uint(u128::MAX)
            { Self::one() }
        else
            { self.common_iroot_uint(exp.into_u128()) }
    }

    // pub fn isqrt(&self) -> Self
    /// Calculates the square root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// If the exact value of the square root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of the square root of `self` will be returned.
    /// 
    /// # Features
    /// The result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let res = a_biguint.isqrt();
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.isqrt)
    pub fn isqrt(&self) -> Self
    {
        if self.is_zero()
            { Self::zero() }
        else if self.is_one()
            { Self::one() }
        else
            { self.common_iroot_uint(2_u8) }
    }

    // pub fn isqrt_assign(&mut self)
    /// Calculates the square root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - If the exact value of the square root of `self` can be expressed with
    ///   `Self`-typed unsigned integer, it will be assigned to `self`.
    ///   Otherwise, the `Self`-typed biggest unsigned integer that is less
    ///   than the exact value of the second root of `self` will be assigned
    ///   to `self`.
    /// - The result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.isqrt_assign();
    /// println!("After a_biguint.isqrt_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.isqrt_assign)
    pub fn isqrt_assign(&mut self)
    {
        biguint_calc_to_calc_assign!(self, Self::isqrt);
    }

    // pub fn ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Output
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [ilog2()](struct@BigUInt#method.ilog2)
    /// can produce results more efficiently for base 2, and
    /// [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog)
    pub fn ilog(&self, base: &Self) -> Self
    {
        general_calc_ilog!(self, Self::common_ilog, base);
    }

    // pub fn ilog_assign(&mut self, base: &Self)
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [ilog2_assign()](struct@BigUInt#method.ilog2_assign)
    /// can produce results more efficiently for base 2, and
    /// [ilog10_assign()](struct@BigUInt#method.ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// a_biguint.ilog_assign(&base);
    /// println!("After a_biguint.ilog_assign({}), a_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog_assign)
    pub fn ilog_assign(&mut self, base: &Self)
    {
        biguint_calc_to_calc_assign!(self, Self::ilog, base);
    }

    pub(super) fn common_ilog(&self, base: &Self) -> Self
    {
        general_calc_common_ilog!(self, Self::wrapping_div_assign, base);
    }

    // pub fn ilog2(&self) -> Self
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [ilog_uint()](struct@BigUInt#method.ilog_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.ilog2();
    /// println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog2)
    pub fn ilog2(&self) -> Self
    {
        if self.is_zero()
            { panic!(); }
        Self::from_uint(self.length_in_bits() as u64 - self.leading_zeros() as u64 - 1_u64)
    }

    // pub fn ilog2_assign(&mut self)
    /// Calculates the base 2 logarithm of the `BigUInt` instance, rounded down,
    /// and assigns the result to `self`.
    ///
    /// # Panics
    /// * Panics if the internal storage bit-width is 128 bits or less, as 
    ///   certain operations may result in undefined behavior or a panic in 
    ///   specific environments.
    /// * Panics if the magnitude of the number is zero, as the logarithm of 
    ///   zero is undefined.
    ///
    /// # Implementation Details
    /// After the calculation, `self` is updated to store the floor of the 
    /// base 2 logarithm. Status flags are cumulative.
    /// 
    /// # Alternatives
    /// This method is highly optimized for base 2. For other bases, use 
    /// [`ilog_assign_uint()`](struct@BigUInt#method.ilog_assign_uint). For 
    /// base 10 specifically, [`ilog10_assign()`](struct@BigUInt#method.ilog10_assign) 
    /// provides better performance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(64_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog2_assign();
    /// println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog2_assign)
    pub fn ilog2_assign(&mut self)
    {
        biguint_calc_to_calc_assign!(self, Self::ilog2);
    }

    // pub fn ilog10(&self) -> Self
    /// Returns the base 10 logarithm of the `BigUInt` instance, rounded down.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the floor of the base 10 logarithm.
    ///
    /// # Panics
    /// * Panics if the internal storage bit-width is 128 bits or less, as 
    ///   certain operations may result in undefined behavior or a panic in 
    ///   specific environments.
    /// * Panics if the magnitude of the number is zero, as the logarithm of 
    ///   zero is undefined.
    ///
    /// # Implementation Details
    /// This method calculates the largest integer `x` such that `10^x <= self`.
    /// 
    /// # Alternatives
    /// This method is highly optimized for base 10. For other bases, use 
    /// [`ilog_uint()`](struct@BigUInt#method.ilog_uint). For base 2 
    /// specifically, [`ilog2()`](struct@BigUInt#method.ilog2) provides 
    /// better performance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog10)
    #[inline]
    pub fn ilog10(&self) -> Self
    {
        self.ilog_uint(10_u8)
    }

    // pub fn ilog10_assign(&mut self)
    /// Calculates the base 10 logarithm of the `BigUInt` instance, rounded down,
    /// and assigns the result to `self`.
    ///
    /// # Panics
    /// * Panics if the internal storage bit-width is 128 bits or less, as 
    ///   certain operations may result in undefined behavior or a panic in 
    ///   specific environments.
    /// * Panics if the magnitude of the number is zero, as the logarithm of 
    ///   zero is undefined.
    ///
    /// # Implementation Details
    /// After the calculation, `self` is updated to store the floor of the 
    /// base 10 logarithm. Status flags are cumulative.
    /// 
    /// # Alternatives
    /// This method is provided for convenience. For other bases, use 
    /// [`ilog_assign_uint()`](struct@BigUInt#method.ilog_assign_uint). For 
    /// base 2 specifically, [`ilog2_assign()`](struct@BigUInt#method.ilog2_assign) 
    /// provides better performance.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10000_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog10_assign();
    /// println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.ilog10_assign)
    #[inline]
    pub fn ilog10_assign(&mut self)
    {
        self.ilog_assign_uint(10_u8)
        // For the future upgrade, the following code is remained.
        // let flag = self.get_all_flags();
        // let log10 = self.ilog10();
        // self.set_number(log10.get_number());
        // self.set_flag_bit(flag);
    }

    

    /***** METHODS FOR BIT OPERATION *****/

    // pub fn shift_left<U>(&self, n: U) -> Self
    /// Shifts the bits of the `BigUInt` to the left by `n` positions.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to shift to the left.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the left-shifted value.
    ///
    /// # Implementation Details
    /// Bits shifted out from the Most Significant Bit (MSB) are lost. If any 
    /// non-zero bits are shifted out, the `LEFT_CARRY` flag of the result 
    /// will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.shift_left(n);
    /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), true);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.shift_left)
    pub fn shift_left<U>(&self, n: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::shift_left_assign, n);
    }

    // pub fn shift_left_assign<U>(&mut self, n: U)
    /// Shifts the bits of the `BigUInt` to the left by `n` positions 
    /// in-place.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to shift to the left.
    ///
    /// # Implementation Details
    /// Bits shifted out from the Most Significant Bit (MSB) are lost. If any 
    /// non-zero bits are shifted out, the `LEFT_CARRY` flag of `self` 
    /// will be set. Status flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 3_u8;
    /// a_biguint.shift_left_assign(n);
    /// println!("After a_biguint.shift_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.shift_left_assign)
    pub fn shift_left_assign<U>(&mut self, n: U)
    where U: TraitsBigUInt<U>
    {
        if n.into_u128() >= self.length_in_bits().into_u128()
        {
            if !self.is_zero()
                { self.set_left_carry(); }
            self.set_zero();
            return;
        }

        let size_t_bits = T::size_in_bits();    // The maximum of size_t_bits is 128. So, it can be cantained in even u8.
        let chunk_num = n.wrapping_div(U::u32_as_smalluint(size_t_bits)).into_usize();
        let piece_num = n.wrapping_rem(U::u32_as_smalluint(size_t_bits)).into_usize();
        let zero = T::zero();
        if chunk_num > 0
        {
            for i in N-chunk_num..N
            {
                if !self.get_num_(i).is_zero()
                {
                    self.set_left_carry();
                    break;
                }
            }
            self.copy_within(0..N-chunk_num, chunk_num);
            for idx in 0..chunk_num
                { self.set_num_(idx, zero); }
        }
        if piece_num == 0
            { return; }
        if !(self.get_num_(N-1) >> T::u32_as_smalluint(size_t_bits - piece_num as u32)).is_zero()
            { self.set_left_carry(); }

        let mut num: T;
        let mut carry = zero;
        let shl = T::usize_as_smalluint(piece_num);
        let shr = T::u32_as_smalluint(size_t_bits - piece_num as u32);
        for idx in chunk_num..N
        {
            num = (self.get_num_(idx) << shl) | carry;
            carry = if piece_num.is_zero()
                        { zero }
                    else
                        { self.get_num_(idx) >> shr };
            self.set_num_(idx, num);
        }
        if !carry.is_zero()
            { self.set_left_carry(); }
    }

    // pub fn shift_right<U>(&self, n: U) -> Self
    /// Shifts the bits of the `BigUInt` to the right by `n` positions.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to shift to the right.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the right-shifted value.
    ///
    /// # Implementation Details
    /// Bits shifted out from the Least Significant Bit (LSB) are lost. If any 
    /// non-zero bits are shifted out, the `RIGHT_CARRY` flag of the result 
    /// will be set.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.shift_right(n);
    /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.shift_right)
    pub fn shift_right<U>(&self, n: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::shift_right_assign, n);
    }

    // pub fn shift_right_assign<U>(&mut self, n: U)
    /// Shifts the bits of the `BigUInt` to the right by `n` positions 
    /// in-place.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to shift to the right.
    ///
    /// # Implementation Details
    /// Bits shifted out from the Least Significant Bit (LSB) are lost. If any 
    /// non-zero bits are shifted out, the `RIGHT_CARRY` flag of `self` 
    /// will be set. Status flags are cumulative.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 3_u8;
    /// a_biguint.shift_right_assign(n);
    /// println!("After a_biguint.shift_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.shift_right_assign)
    pub fn shift_right_assign<U>(&mut self, n: U)
    where U: TraitsBigUInt<U>
    {
        if n.into_u128() >= self.length_in_bits().into_u128()
        {
            if !self.is_zero()
                { self.set_right_carry(); }
            self.set_zero();
            return;
        }

        let size_t_bits = T::size_in_bits();    // The maximum of size_t_bits is 128. So, it can be cantained in even u8.
        let chunk_num = n.wrapping_div(U::u32_as_smalluint(size_t_bits)).into_usize();
        let piece_num = n.wrapping_rem(U::u32_as_smalluint(size_t_bits)).into_usize();
        let zero = T::zero();
        if chunk_num > 0
        {
            for i in 0..chunk_num
            {
                if !self.get_num_(i).is_zero()
                {
                    self.set_right_carry();
                    break;
                }
            }
            self.copy_within(chunk_num..N, 0);
            for idx in N-chunk_num..N
                { self.set_num_(idx, zero); }
        }
        if piece_num == 0
            { return; }
        if !(self.get_num_(0) << T::u32_as_smalluint(size_t_bits - piece_num as u32)).is_zero()
            { self.set_right_carry(); }


        let mut num: T;
        let mut carry = zero;
        let mut idx = N - 1 - chunk_num;
        let shr = T::usize_as_smalluint(piece_num);
        let shl = T::u32_as_smalluint(size_t_bits - piece_num as u32);
        loop
        {
            num = (self.get_num_(idx) >> shr) | carry;
            carry = if piece_num.is_zero()
                        { zero }
                    else
                        { self.get_num_(idx) << shl };
            self.set_num_(idx, num);
            if idx == 0
                { break; }
            idx -= 1;
        }
        if !carry.is_zero()
            { self.set_right_carry(); }
    }

    // pub fn rotate_left<U>(&self, n: U) -> Self
    /// Rotates the bits of the `BigUInt` to the left by `n` positions.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to rotate to the left.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the left-rotated value.
    ///
    /// # Implementation Details
    /// Unlike shifting, rotation fills the vacated bit positions with the 
    /// bits that were pushed out from the other end. This operation does not 
    /// modify status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.rotate_left(n);
    /// println!("{} <<< {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.rotate_left)
    #[inline]
    pub fn rotate_left<U>(&self, n: U) -> Self
    where U: TraitsBigUInt<U>
    {
       biguint_calc_assign_to_calc!(self, Self::rotate_left_assign, n);
    }

    // pub fn rotate_left_assign<U>(&mut self, n: U)
    /// Rotates the bits of the `BigUInt` to the left by `n` positions 
    /// in-place.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to rotate to the left.
    ///
    /// # Implementation Details
    /// Unlike shifting, rotation fills the vacated bit positions with the 
    /// bits that were pushed out from the other end. This operation does not 
    /// modify status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 3_u8;
    /// a_biguint.rotate_left_assign(n);
    /// println!("After a_biguint.rotate_left_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010110_01100111_10000111_11111000_00000111_11111111_11111000_00000000_00000111_11111111_11111111_11111000_00000000_00000000_00000111_11111111_11111111_11111111_11111000_00000000_00000000_00000000_00000101_10011100_01111000_01111100_00011111_10000001_11111100_00000111_11111000_00000101");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.rotate_left_assign)
    #[inline]
    pub fn rotate_left_assign<U>(&mut self, n: U)
    where U: TraitsBigUInt<U>
    {
        calc_rotate_assign!(self, Self::shift_left_assign, Self::shift_right, n);
    }

    // pub fn rotate_right<U>(&self, n: U) -> Self
    /// Rotates the bits of the `BigUInt` to the right by `n` positions.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to rotate to the right.
    ///
    /// # Returns
    /// A new `BigUInt` instance containing the right-rotated value.
    ///
    /// # Implementation Details
    /// Unlike shifting, rotation fills the vacated bit positions with the 
    /// bits that were pushed out from the other end. This operation does not 
    /// modify status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.rotate_right(n);
    /// println!("{} >>> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.rotate_right)
    #[inline]
    pub fn rotate_right<U>(&self, n: U) -> Self
    where U: TraitsBigUInt<U>
    {
        biguint_calc_assign_to_calc!(self, Self::rotate_right_assign, n);
    }

    // pub fn rotate_right_assign<U>(&mut self, n: U)
    /// Rotates the bits of the `BigUInt` to the right by `n` positions 
    /// in-place.
    ///
    /// # Arguments
    /// * `n`: The number of bit positions to rotate to the right.
    ///
    /// # Implementation Details
    /// Unlike shifting, rotation fills the vacated bit positions with the 
    /// bits that were pushed out from the other end. This operation does not 
    /// modify status flags.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let n = 3_u8;
    /// a_biguint.rotate_right_assign(n);
    /// println!("After a_biguint.rotate_right_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101_01011001_10011110_00011111_11100000_00011111_11111111_11100000_00000000_00011111_11111111_11111111_11100000_00000000_00000000_00011111_11111111_11111111_11111111_11100000_00000000_00000000_00000000_00010110_01110001_11100001_11110000_01111110_00000111_11110000_00011111_11100000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.rotate_right_assign)
    #[inline]
    pub fn rotate_right_assign<U>(&mut self, n: U)
    where U: TraitsBigUInt<U>
    {
        calc_rotate_assign!(self, Self::shift_right_assign, Self::shift_left, n);
    }

    // pub fn and(&self, rhs: &Self) -> Self
    /// Performs a bitwise AND operation between `self` and `rhs`.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the AND operation 
    ///   with.
    /// 
    /// # Returns
    /// A new `BigUInt` instance containing the result of the bitwise AND.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array. Status flags are not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.and(&b_biguint);
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.and)
    pub fn and(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::and_assign, rhs);
    }

    // pub fn and_assign(&mut self, rhs: &Self)
    /// Performs a bitwise AND operation between `self` and `rhs` in-place.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the AND operation 
    ///   with.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array and assigns the result to `self`. Status flags are not 
    /// modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint.and_assign(&b_biguint);
    /// println!("After a_biguint.and_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.and_assign)
    pub fn and_assign(&mut self, rhs: &Self)
    {
        bitcalc!(self, &, rhs);
    }

    // pub fn or(&self, rhs: &Self) -> Self
    /// Performs a bitwise OR operation between `self` and `rhs`.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the OR operation 
    ///   with.
    /// 
    /// # Returns
    /// A new `BigUInt` instance containing the result of the bitwise OR.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array. Status flags are not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.or(&b_biguint);
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.iroot_assign)
    pub fn or(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::or_assign, rhs);
    }

    // pub fn or_assign(&mut self, rhs: &Self)
    /// Performs a bitwise OR operation between `self` and `rhs` in-place.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the OR operation 
    ///   with.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array and assigns the result to `self`. Status flags are not 
    /// modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint.or_assign(&b_biguint);
    /// println!("After a_biguint.or_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.or_assign)
    pub fn or_assign(&mut self, rhs: &Self)
    {
        bitcalc!(self, |, rhs);
    }

    // pub fn xor(&self, rhs: &Self) -> Self
    /// Performs a bitwise XOR operation between `self` and `rhs`.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the XOR operation 
    ///   with.
    /// 
    /// # Returns
    /// A new `BigUInt` instance containing the result of the bitwise XOR.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array. Status flags are not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.xor(&b_biguint);
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.xor)
    pub fn xor(&self, rhs: &Self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::xor_assign, rhs);
    }

    // pub fn xor_assign(&mut self, rhs: &Self)
    /// Performs a bitwise XOR operation between `self` and `rhs` in-place.
    /// 
    /// # Arguments
    /// * `rhs`: A reference to the `BigUInt` to perform the XOR operation 
    ///   with.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array and assigns the result to `self`. Status flags are not 
    /// modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint.xor_assign(&b_biguint);
    /// println!("After a_biguint.xor_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.xor_assign)
    pub fn xor_assign(&mut self, rhs: &Self)
    {
        bitcalc!(self, ^, rhs);
    }

    // pub fn flip(&self) -> Self
    /// Performs a bitwise NOT operation on the `BigUInt`.
    /// 
    /// # Returns
    /// A new `BigUInt` instance with every bit flipped (inverted).
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array. Status flags are not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.flip();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.flip)
    pub fn flip(&self) -> Self
    {
        let mut res = self.clone();
        res.flip_assign();
        res
    }

    // pub fn flip_assign(&mut self)
    /// Performs a bitwise NOT operation on the `BigUInt` in-place.
    ///
    /// # Implementation Details
    /// This method performs the operation element-wise across the internal 
    /// storage array and assigns the result to `self`. Status flags are not 
    /// modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.flip_assign();
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.flip_assign)
    pub fn flip_assign(&mut self)
    {
        for idx in 0..N
            { self.set_num_(idx, !self.get_num_(idx)); }
    }

    // pub fn reverse_bits(&self) -> Self
    /// Returns a new `BigUInt` with the order of all bits reversed.
    /// 
    /// # Returns
    /// A new `BigUInt` instance where the Least Significant Bit (LSB) becomes 
    /// the Most Significant Bit (MSB), and vice versa.
    ///
    /// # Implementation Details
    /// This method reverses the bits within each storage element and also 
    /// reverses the order of the elements themselves. Status flags are 
    /// not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.reverse_bits();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.reverse_bits)
    pub fn reverse_bits(&self) -> Self
    {
        let mut res = self.clone();
        res.reverse_bits_assign();
        res
    }

    // pub fn reverse_bits_assign(&mut self)
    /// Reverses the order of all bits in the `BigUInt` in-place.
    /// 
    /// # Implementation Details
    /// This method reverses the bits within each storage element and also 
    /// reverses the order of the elements themselves. Status flags are 
    /// not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reverse_bits_assign();
    /// println!("After a_biguint.reverse_bits_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000001_11111100_00001111_11000001_11110000_11110001_11001101_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_00001111_00110011_01010101");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.reverse_bits_assign)
    pub fn reverse_bits_assign(&mut self)
    {
        let mut low: T;
        let mut high: T;
        for i in 0..N/2
        {
            low = self.get_num_(i).reverse_bits();
            high = self.get_num_(N-1-i).reverse_bits();
            self.set_num_(N-1-i, low);
            self.set_num_(i, high);
        }
        if N.is_odd()
            { self.set_num_(N/2+1, self.get_num_(N/2+1).reverse_bits()); }
    }

    // pub fn swap_bytes(&self) -> Self
    /// Returns a new `BigUInt` with the order of all bytes reversed.
    /// 
    /// # Returns
    /// A new `BigUInt` instance where the byte order is reversed across 
    /// the entire bit-width.
    /// 
    /// # Implementation Details
    /// This method reverses the byte order within each storage element and 
    /// also reverses the order of the elements themselves. Status flags are 
    /// not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.swap_bytes();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.swap_bytes)
    pub fn swap_bytes(&self) -> Self
    {
        let mut res = self.clone();
        res.swap_bytes_assign();
        res
    }

    // pub fn swap_bytes_assign(&mut self)
    /// Reverses the order of all bytes in the `BigUInt` in-place.
    /// 
    /// # Implementation Details
    /// This method reverses the byte order within each storage element and 
    /// also reverses the order of the elements themselves. Status flags are 
    /// not modified.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.swap_bytes_assign();
    /// println!("After a_biguint.swap_bytes_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.swap_bytes_assign)
    pub fn swap_bytes_assign(&mut self)
    {
        for i in 0..N/2
        {
            let tmp = self.get_num_(i).swap_bytes();
            self.set_num_(i, self.get_num_(N-1-i).swap_bytes());
            self.set_num_(N-1-i, tmp);
        }
    }



    /***** METHODS FOR CONVERTING INTO OTHER TYPES WITH/WITHOUT LOSS *****/

    // pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    /// Converts `self` into a `BigUInt` of a different storage type and size.
    /// 
    /// # Returns
    /// A new `BigUInt<U, M>` instance containing the value of `self`, 
    /// truncated or zero-extended as necessary.
    ///
    /// # Implementation Details
    /// This method copies the numeric magnitude. If the target bit-width is 
    /// smaller than the current one, the higher-order bits are truncated. 
    /// Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::number::U256_with_u128;
    /// use std::fmt::Write;
    /// 
    /// let mut a_biguint = U256_with_u128::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// a_biguint.set_overflow();
    /// a_biguint.set_underflow();
    /// a_biguint.set_undefined();
    /// a_biguint.set_infinity();
    /// a_biguint.set_divided_by_zero();
    /// a_biguint.set_left_carry();
    /// a_biguint.set_right_carry();
    /// 
    /// let b_biguint: BigUInt<u16, 32> = a_biguint.into_biguint();
    /// println!("a_biguint = {0} = {0:?}", a_biguint);
    /// println!("b_biguint = {0} = {0:?}", b_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// 
    /// assert_eq!(b_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_underflow(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint.is_left_carry(), false);
    /// assert_eq!(b_biguint.is_right_carry(), false);
    /// 
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", a_biguint)
    /// {
    ///     Ok(_) =>    { assert_eq!(a_txt, "BigUInt { number: [340282346638528863123979975818481827584, 227032875824372601055702174981657985279], flag: 127 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// let mut b_txt = String::new();
    /// match write!(&mut b_txt, "{:?}", b_biguint)
    /// {
    ///     Ok(_) => { assert_eq!(b_txt, "BigUInt { number: [65280, 16256, 33776, 36623, 179, 0, 65280, 65535, 255, 0, 65535, 255, 65280, 255, 61695, 43724, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], flag: 0 }"); },
    ///     Err(_) =>   { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_biguint)
    #[inline]
    pub fn into_biguint<U, const M: usize>(&self) -> BigUInt<U, M>
    where U: TraitsBigUInt<U>
    {
        BigUInt::<U, M>::from_biguint(&self)
    }

    // pub fn into_uint<U>(&self) -> U
    /// Converts the `BigUInt` instance into a primitive unsigned integer.
    /// 
    /// # Returns
    /// The primitive unsigned integer of type `U` representing the value of 
    /// `self`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds the capacity of type `U`, the 
    /// higher-order bits are truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u128: u128 = a_biguint.into_uint();
    /// let b_u64: u64 = a_biguint.into_uint();
    /// let b_u32: u32 = a_biguint.into_uint();
    /// let b_u16: u16 = a_biguint.into_uint();
    /// let b_u8: u8 = a_biguint.into_uint();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u128, 16384545419507531775_u128);
    /// assert_eq!(b_u64, 16384545419507531775_u64);
    /// assert_eq!(b_u32, 4294967295_u32);
    /// assert_eq!(b_u16, 65535_u16);
    /// assert_eq!(b_u8, 255_u8);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_uint)
    pub fn into_uint<U>(&self) -> U
    where U: TraitsBigUInt<U>
    {
        if T::size_in_bytes() >= U::size_in_bytes()
        {
            SharedValues::<U, T>::from_src(self.get_num_(0)).get_des()
        }
        else
        {
            match U::size_in_bytes()
            {
                2 => { U::u16_as_smalluint(self.into_u16()) },
                4 => { U::u32_as_smalluint(self.into_u32()) },
                8 => { U::u64_as_smalluint(self.into_u64()) },
                _ => { U::u128_as_smalluint(self.into_u128()) },
            }
        }
    }

    // pub fn into_u128(&self) -> u128
    /// Converts the `BigUInt` instance into a primitive `u128`.
    /// 
    /// # Returns
    /// The lowest 128 bits of the `BigUInt` as a `u128`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds 128 bits, the higher-order bits are 
    /// truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u128 = a_biguint.into_u128();
    /// println!("u128 of {} = {}", a_biguint, b_u128);
    /// assert_eq!(b_u128, 16384545419507531775_u128);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_u128)
    pub fn into_u128(&self) -> u128
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    for i in 0..if 16 < N {16} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 8 < N {8} else {N}
                        { num.set_ushort_(i, self.get_num_(i).into_u16()); }
                },
            4 => {
                    for i in 0..if 4 < N {4} else {N}
                        { num.set_uint_(i, self.get_num_(i).into_u32()); }
                },
            8 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ulong_(i, self.get_num_(i).into_u64()); }
                },
            _ => { return self.get_num_(0).into_u128(); },
        }
        num.get()
    }

    // pub fn into_u64(&self) -> u64
    /// Converts the `BigUInt` instance into a primitive `u64`.
    /// 
    /// # Returns
    /// The lowest 64 bits of the `BigUInt` as a `u64`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds 64 bits, the higher-order bits are 
    /// truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(16384545419507531775_u64);
    /// let b_u64: u64 = a_biguint.into_u64();
    /// println!("u64 of {} = {}", a_biguint, b_u64);
    /// assert_eq!(b_u64, 16384545419507531775_u64);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_u64)
    pub fn into_u64(&self) -> u64
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    for i in 0..if 8 < N {8} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 4 < N {4} else {N}
                    { num.set_ushort_(i, self.get_num_(i).into_u16()); }
                },
            4 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_uint_(i, self.get_num_(i).into_u32()); }
                },
            8 => { return self.get_num_(0).into_u64(); },
            _ => { num.set(self.number[0].into_u128()); },
        }
        num.get_ulong_(0)
    }

    // pub fn into_u32(&self) -> u32
    /// Converts the `BigUInt` instance into a primitive `u32`.
    /// 
    /// # Returns
    /// The lowest 32 bits of the `BigUInt` as a `u32`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds 32 bits, the higher-order bits are 
    /// truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u32 = a_biguint.into_u32();
    /// println!("u32 of {} = {}", a_biguint, b_u32);
    /// assert_eq!(b_u32, 163_u32);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_u32)
    pub fn into_u32(&self) -> u32
    {
        let mut num = LongerUnion::new();
        match T::size_in_bytes()
        {
            1 => {
                    for i in 0..if 4 < N {4} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ushort_(i, self.get_num_(i).into_u16()); }
                },
            4 => { return self.get_num_(0).into_u32(); },
            8 => { num.set_ulong_(0, self.get_num_(0).into_u64()); },
            _ => { num.set(self.get_num_(0).into_u128()); },
        }
        num.get_uint_(0)
    }

    // pub fn into_u16(&self) -> u16
    /// Converts the `BigUInt` instance into a primitive `u16`.
    /// 
    /// # Returns
    /// The lowest 16 bits of the `BigUInt` as a `u16`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds 16 bits, the higher-order bits are 
    /// truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u16 = a_biguint.into_u16();
    /// println!("u16 of {} = {}", a_biguint, b_u16);
    /// assert_eq!(b_u16, 163_u16);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_u16)
    pub fn into_u16(&self) -> u16
    {
        let mut num = LongerUnion::new();
        match size_of::<T>()
        {
            1 => {
                    for i in 0..if 2 < N {2} else {N}
                        { num.set_ubyte_(i, self.get_num_(i).into_u8()); }
                },
            2 => { return self.get_num_(0).into_u16(); },
            4 => { num.set_uint_(0, self.get_num_(0).into_u32()); },
            8 => { num.set_ulong_(0, self.get_num_(0).into_u64()); },
            _ => { num.set(self.get_num_(0).into_u128()); },
        }
        num.get_ushort_(0)
    }

    // pub fn into_u8(&self) -> u8
    /// Converts the `BigUInt` instance into a primitive `u8`.
    /// 
    /// # Returns
    /// The lowest 8 bits of the `BigUInt` as a `u8`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds 8 bits, the higher-order bits are 
    /// truncated. Internal status flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(163_u8);
    /// let b_u8: u8 = a_biguint.into_u8();
    /// println!("u8 of {} = {}", a_biguint, b_u8);
    /// assert_eq!(b_u8, 163_u8);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_u8)
    #[inline]
    pub fn into_u8(&self) -> u8
    {
        self.get_num_(0).into_u8()
    }

    // pub fn into_usize(&self) -> usize
    /// Converts the `BigUInt` instance into a primitive `usize`.
    /// 
    /// # Returns
    /// The lowest `usize` bits of the `BigUInt` as a `usize`.
    /// 
    /// # Implementation Details
    /// If the magnitude of `self` exceeds the pointer bit-width of the target 
    /// architecture, the higher-order bits are truncated. Internal status 
    /// flags are not copied.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(65280_u16);
    /// let b_usize = a_biguint.into_usize();
    /// println!("usize of {} = {}", a_biguint, b_usize);
    /// assert_eq!(b_usize, 65280_usize);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.into_usize)
    #[inline]
    pub fn into_usize(&self) -> usize
    {
        // #[cfg(target_pointer_width = "128")]    return self.into_u128().into_usize();
        #[cfg(target_pointer_width = "64")]     return self.into_u64().into_usize();
        #[cfg(target_pointer_width = "32")]     return self.into_u32().into_usize();
        #[cfg(target_pointer_width = "16")]     return self.into_u16().into_usize();
        // #[cfg(target_pointer_width = "8")]      return self.into_u8().into_usize();
    }

    // pub fn to_be(&self) -> Self
    /// Converts the `BigUInt` instance to big-endian byte order.
    /// 
    /// # Returns
    /// A new `BigUInt` instance in big-endian representation.
    ///
    /// # Implementation Details
    /// On a big-endian architecture, this is a no-op. On a little-endian 
    /// architecture, every byte in the internal storage is swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_be();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_be)
    #[inline]
    pub fn to_be(&self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::to_be_assign);
    }

    // pub fn to_be_assign(&mut self)
    /// Converts the `BigUInt` instance to big-endian byte order in-place.
    /// 
    /// # Implementation Details
    /// On a big-endian architecture, this is a no-op. On a little-endian 
    /// architecture, every byte in the internal storage is swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.to_be_assign();
    /// println!("After a_biguint.to_be_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_be_assign)
    #[inline]
    pub fn to_be_assign(&mut self)
    {
        #[cfg(target_endian = "little")]    self.swap_bytes_assign();
    }

    // pub fn to_be_bytes(&self) -> [T; N]
    /// Returns the memory representation of the `BigUInt` as a big-endian 
    /// byte array.
    /// 
    /// # Returns
    /// An array of type `[T; N]` in big-endian (network) byte order.
    ///
    /// # Implementation Details
    /// On a big-endian architecture, this returns a clone of the internal 
    /// storage. On a little-endian architecture, the bytes are swapped 
    /// before returning.
    /// 
    /// # Example 1
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_be_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => {
    ///             #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
    ///             #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
    ///         },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_be_bytes)
    #[inline]
    pub fn to_be_bytes(&self) -> [T; N]
    {
        return self.to_be().get_number().clone();
    }

    // pub fn to_le(&self) -> Self
    /// Converts the `BigUInt` instance to little-endian byte order.
    /// 
    /// # Returns
    /// A new `BigUInt` instance in little-endian representation.
    ///
    /// # Implementation Details
    /// On a little-endian architecture, this is a no-op. On a big-endian 
    /// architecture, every byte in the internal storage is swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_le();
    /// println!("{} => {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// #[cfg(target_endian = "big")]       assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_le)
    #[inline]
    pub fn to_le(&self) -> Self
    {
        biguint_calc_assign_to_calc!(self, Self::to_le_assign);
    }
    // pub fn to_le_assign(&mut self)
    /// Converts the `BigUInt` instance to little-endian byte order in-place.
    /// 
    /// # Implementation Details
    /// On a little-endian architecture, this is a no-op. On a big-endian 
    /// architecture, every byte in the internal storage is swapped.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.to_le_assign();
    /// println!("After a_biguint.to_le_assign(), a_biguint = {}.", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// #[cfg(target_endian = "little")]    assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// #[cfg(target_endian = "big")]       assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_10000000_00111111_11110000_10000011_00001111_10001111_10110011_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_11111111_11111111_00000000_11111111_11110000_11001100_10101010");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_le_assign)
    #[inline]
    pub fn to_le_assign(&mut self)
    {
        #[cfg(target_endian = "big")]   self.swap_bytes_assign();
    }

    // pub fn to_le_bytes(&self) -> [T; N]
    /// Returns the memory representation of the `BigUInt` as a little-endian 
    /// byte array.
    /// 
    /// # Returns
    /// An array of type `[T; N]` in little-endian byte order.
    ///
    /// # Implementation Details
    /// On a little-endian architecture, this returns a clone of the internal 
    /// storage. On a big-endian architecture, the bytes are swapped 
    /// before returning.
    /// 
    /// # Example 1
    /// ```
    /// use std::fmt::Write;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = a_biguint.to_le_bytes();
    /// println!("{:?} => {:?}", a_biguint, res);
    /// let mut a_txt = String::new();
    /// match write!(&mut a_txt, "{:?}", res)
    /// {
    ///     Ok(_) => {
    ///             #[cfg(target_endian = "little")]    assert_eq!(a_txt, "[0, 255, 128, 63, 240, 131, 15, 143, 179, 0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 240, 204, 170]");
    ///             #[cfg(target_endian = "big")]       assert_eq!(a_txt, "[170, 204, 240, 255, 0, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 179, 143, 15, 131, 240, 63, 128, 255, 0]");
    ///         },
    ///     Err(_) => { panic!("Error"); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_le_bytes)
    #[inline]
    pub fn to_le_bytes(&self) -> [T; N]
    {
        return self.to_le().get_number().clone();
    }

    // pub fn to_string_with_radix_and_stride_and_delimiter(&self, radix: usize, stride: usize, delimiter: &str) -> Result<String, NumberErr>
    /// Converts the `BigUInt` into a string with a specified radix, stride, 
    /// and custom delimiter.
    /// 
    /// # Arguments
    /// * `radix`: The base for the string representation (2 to 62).
    /// * `stride`: The number of digits between delimiters.
    /// * `delimiter`: The string used to separate digit groups.
    /// 
    /// # Returns
    /// * `Ok(String)` containing the formatted numeric string.
    /// * `Err(NumberErr::OutOfValidRadixRange)` if the radix is not 
    ///   between 2 and 62.
    /// 
    /// # Implementation Details
    /// For bases up to 36, digits 10-35 are represented as 'A'-'Z' or 
    /// 'a'-'z' (case-insensitive). For bases 37-62, case sensitivity is 
    /// used ('A'-'Z' for 10-35, 'a'-'z' for 36-61).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride_and_delimiter(10, 3, ",").unwrap(), "77,255,284,354,385,016,970,177,264,758,879,158,019,392,010,587,479,561,699,232,008,238,232,688,983,808");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_string_with_radix_and_stride_and_delimiter)
    pub fn to_string_with_radix_and_stride_and_delimiter(&self, radix: usize, stride: usize, delimiter: &str) -> Result<String, NumberErr>
    {
        let res = self.to_string_with_radix_and_stride(radix, stride);
        match res
        {
            Ok(txt) =>  { Ok(txt.replace("_", delimiter)) },
            Err(_) =>   { res },
        }
    }

    // pub fn to_string_with_radix_and_stride(&self, radix: usize, stride: usize) -> Result<String, NumberErr>
    /// Converts the `BigUInt` into a string with a specified radix and 
    /// stride, using '_' as the default delimiter.
    /// 
    /// # Arguments
    /// * `radix`: The base for the string representation (2 to 62).
    /// * `stride`: The number of digits between '_' delimiters.
    /// 
    /// # Returns
    /// * `Ok(String)` containing the formatted numeric string.
    /// * `Err(NumberErr::OutOfValidRadixRange)` if the radix is not 
    ///   between 2 and 62.
    /// 
    /// # Implementation Details
    /// Grouping with '_' is applied every `stride` digits to improve 
    /// readability. For more details on character mapping for large bases, 
    /// see `to_string_with_radix_and_stride_and_delimiter()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix_and_stride(10, 3).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 3).unwrap(), "77_255_284_354_385_016_970_177_264_758_879_158_019_392_010_587_479_561_699_232_008_238_232_688_983_808");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_string_with_radix_and_stride)
    pub fn to_string_with_radix_and_stride(&self, radix: usize, stride: usize) -> Result<String, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        if stride == 0 
        {
            self.to_string_with_radix(radix)
        }
        else
        {
            let mut stride_num = stride;
            let mut txt = String::new();
            let mut dividend = self.clone();
            let mut remainder;
            loop
            {
                (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_smalluint(radix));
                let r = remainder.into_u32();
                let c = if r < 10     { ('0' as u32 + r) as u8 as char }
                        else if r < 10 + 26 { ('A' as u32 - 10 + r) as u8 as char }
                        else                { ('a' as u32 - 10 - 26 + r) as u8 as char };  // if r < 10 + 26 + 26
                txt.push(c);
                stride_num -= 1;
                if dividend.is_zero()
                    { break; }
                if stride_num == 0
                {
                    txt.push('_');
                    stride_num = stride;
                }
            }
            if txt.len() == 0
                { txt.push('0'); }
            let mut num_str = String::new();
            while let Some(ch) = txt.pop()
                { num_str.push(ch); }
            Ok(num_str)
        }
    }

    // pub fn to_string_with_radix(&self, radix: usize) -> Result<String, NumberErr>
    /// Converts the `BigUInt` into a string with a specified radix.
    /// 
    /// # Arguments
    /// * `radix`: The base for the string representation (2 to 62).
    /// 
    /// # Returns
    /// * `Ok(String)` containing the numeric string without any delimiters.
    /// * `Err(NumberErr::OutOfValidRadixRange)` if the radix is not 
    ///   between 2 and 62.
    /// 
    /// # Implementation Details
    /// This method provides a plain numeric string representation. For more 
    /// details on character mapping for large bases, see 
    /// `to_string_with_radix_and_stride_and_delimiter()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::NumberErr;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint.to_string_with_radix(10).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix(10).unwrap(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.to_string_with_radix)
    pub fn to_string_with_radix(&self, radix: usize) -> Result<String, NumberErr>
    {
        if (radix < 2) || (radix > 10 + 26 + 26)
            { return Err(NumberErr::OutOfValidRadixRange); }

        let mut txt = String::new();
        let zero = Self::zero();
        let mut dividend = self.clone();
        let mut remainder;
        loop
        {
            (dividend, remainder) = dividend.divide_fully_uint(T::usize_as_smalluint(radix));
            let r = remainder.into_u32();
            let c = if r < 10     { ('0' as u32 + r) as u8 as char }
                    else if r < 10 + 26 { ('A' as u32 - 10 + r) as u8 as char }
                    else                { ('a' as u32 - 10 - 26 + r) as u8 as char };  // if r < 10 + 26 + 26
            txt.push(c);
            if dividend == zero
                { break; }
        }
        if txt.len() == 0
            { txt.push('0'); }
        let mut num_str = String::new();
        while let Some(ch) = txt.pop()
            { num_str.push(ch); }
        Ok(num_str)
    }



    /***** FLAG MANIPULATION *****/

    // pub(super) fn set_flag_bit(&mut self, flag: u8)
    /// Enables the status flags specified by the `flag` bitmask.
    ///
    /// # Arguments
    /// * `flag`: A bitmask representing the flags to be enabled. It can be a 
    ///   single flag (e.g., `OVERFLOW`, `UNDERFLOW`, `INFINITY`, 
    ///   `DIVIDED_BY_ZERO`, `UNDEFINED`) or a bitwise OR combination of 
    ///   multiple flags.
    ///
    /// # Panics
    /// * Panics if the internal storage bit-width is 128 bits or less, as 
    ///   certain operations may result in undefined behavior or a panic in 
    ///   specific environments.
    ///
    /// # Implementation Details
    /// This method performs a bitwise OR operation on the internal flag 
    /// register. It only enables the specified flags and does not reset 
    /// any existing status indicators.
    #[inline]
    pub(super) fn set_flag_bit(&mut self, flag: u8)
    {
        self.flag |= flag;
    }

    // pub(super) fn reset_flag_bit(&mut self, flag: u8)
    /// Resets flag bits that `flag` indicates to be `0`.
    /// 
    /// # Arguments
    /// `flag` idicates which flag will be reset, and is one of `OVERFLOW`,
    /// `UNDERFLOW`, `INFINITY`, `DIVIDED_BY_ZERO`, and `UNDEFINED`,
    /// or the 'OR' combination of them. `flag` is of `u8` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    #[inline]
    pub(super) fn reset_flag_bit(&mut self, flag: u8)
    {
        self.flag &= !flag;
    }

    // fn is_flag_bit_on(&self, flag: u8) -> bool
    /// Checks whether or not the flag bits
    /// that `flag` indicates are set to be `1.
    /// 
    /// # Returns
    /// It returns `true` if the flag bits that `flag` indicates are set.
    /// Otherwise, it returns `false`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    #[inline]
    fn is_flag_bit_on(&self, flag: u8) -> bool
    {
        (self.flag & flag) != 0
    }

    // pub(super) fn get_all_flags(&self) -> u8
    /// Gets all the flag bits.
    ///
    /// # Returns
    /// It returns all the flag bits in the way of the 'OR' combination of them.
    /// The output is of `u8` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Utility
    /// It is useful when you need to store all the flags.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let flags = a_biguint.get_all_flags();
    /// a_biguint.set_overflow();
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_all_flags(flags);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub(super) fn get_all_flags(&self) -> u8
    {
        self.flag
    }

    // pub(super) fn set_all_flags(&mut self, flag: u8)
    /// Sets all flag bits indicated by `flag` to be `1`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Utility
    /// It is useful when you need to restore all the flags.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let flags = a_biguint.get_all_flags();
    /// a_biguint.reset_all_flags();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_all_flags(flags);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub(super) fn set_all_flags(&mut self, flag: u8)
    {
        self.flag = flag;
    }

    // pub(super) fn reset_all_flags(&mut self)
    /// Resets all flag bits to be `0`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Utility
    /// It is useful when you need to set all the flags `false`.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::new();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let flags = a_biguint.get_all_flags();
    /// a_biguint.reset_all_flags();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub(super) fn reset_all_flags(&mut self)
    {
        self.flag = 0;
    }

    // pub fn set_overflow(&mut self)
    /// Sets the `OVERFLOW` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate that an arithmetic 
    /// overflow has occurred in a previous operation.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_overflow(&mut self)
    {
        self.set_flag_bit(Self::OVERFLOW);
    }

    // pub fn reset_overflow(&mut self)
    /// Resets the `OVERFLOW` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for arithmetic overflow.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_overflow(&mut self)
    {
        self.reset_flag_bit(Self::OVERFLOW);
    }

    // pub fn is_overflow(&self) -> bool
    /// Checks if the `OVERFLOW` status flag is set.
    ///
    /// # Returns
    /// * `true` if an arithmetic overflow has occurred.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_overflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_overflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::OVERFLOW)
    }

    // pub fn set_underflow(&mut self)
    /// Sets the `UNDERFLOW` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate that an arithmetic 
    /// underflow has occurred in a previous operation.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_underflow(&mut self)
    {
        self.set_flag_bit(Self::UNDERFLOW);
    }

    // pub fn reset_underflow(&mut self)
    /// Resets the `UNDERFLOW` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for arithmetic underflow.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_underflow(&mut self)
    {
        self.reset_flag_bit(Self::UNDERFLOW);
    }

    // pub fn is_underflow(&self) -> bool
    /// Checks if the `UNDERFLOW` status flag is set.
    ///
    /// # Returns
    /// * `true` if an arithmetic underflow has occurred.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_underflow();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_underflow(&self) -> bool
    {
        self.is_flag_bit_on(Self::UNDERFLOW)
    }

    // pub fn set_infinity(&mut self)
    /// Sets the `INFINITY` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate that a value has 
    /// reached or exceeded representable infinity.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_infinity(&mut self)
    {
        self.set_flag_bit(Self::INFINITY);
    }

    // pub fn reset_infinity(&mut self)
    /// Resets the `INFINITY` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for infinite values.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_infinity(&mut self)
    {
        self.reset_flag_bit(Self::INFINITY);
    }

    // pub fn is_infinity(&self) -> bool
    /// Checks if the `INFINITY` status flag is set.
    ///
    /// # Returns
    /// * `true` if the value is infinite.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_infinity();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_infinity(&self) -> bool
    {
        self.is_flag_bit_on(Self::INFINITY)
    }

    // pub fn set_divided_by_zero(&mut self)
    /// Sets the `DIVIDED_BY_ZERO` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate that a division 
    /// by zero has been attempted.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_divided_by_zero(&mut self)
    {
        self.set_flag_bit(Self::DIVIDED_BY_ZERO);
    }

    // pub fn reset_divided_by_zero(&mut self)
    /// Resets the `DIVIDED_BY_ZERO` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for division by zero.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_divided_by_zero(&mut self)
    {
        self.reset_flag_bit(Self::DIVIDED_BY_ZERO);
    }

    // pub fn is_divided_by_zero(&self) -> bool
    /// Checks if the `DIVIDED_BY_ZERO` status flag is set.
    ///
    /// # Returns
    /// * `true` if a division by zero has occurred.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_divided_by_zero();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_divided_by_zero(&self) -> bool
    {
        self.is_flag_bit_on(Self::DIVIDED_BY_ZERO)
    }

    // pub fn set_undefined(&mut self)
    /// Sets the `UNDEFINED` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate that the result of 
    /// an operation is mathematically undefined.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_undefined(&mut self)
    {
        self.set_flag_bit(Self::UNDEFINED);
    }

    // pub fn reset_undefined(&mut self)
    /// Resets the `UNDEFINED` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for mathematically 
    /// undefined results.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_undefined(&mut self)
    {
        self.reset_flag_bit(Self::UNDEFINED);
    }

    // pub fn is_undefined(&self) -> bool
    /// Checks if the `UNDEFINED` status flag is set.
    ///
    /// # Returns
    /// * `true` if the result is mathematically undefined.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_undefined();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_undefined(&self) -> bool
    {
        self.is_flag_bit_on(Self::UNDEFINED)
    }

    // pub fn set_left_carry(&mut self)
    /// Sets the `LEFT_CARRY` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate a carry-out from 
    /// the most significant end of an operation.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn set_left_carry(&mut self)
    {
        self.set_flag_bit(Self::LEFT_CARRY);
    }

    // pub fn reset_left_carry(&mut self)
    /// Resets the `LEFT_CARRY` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for left-side carry bits.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.reset_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_left_carry(&mut self)
    {
        self.reset_flag_bit(Self::LEFT_CARRY);
    }

    // pub fn is_left_carry(&self) -> bool
    /// Checks if the `LEFT_CARRY` status flag is set.
    ///
    /// # Returns
    /// * `true` if a left-side carry has occurred.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_left_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), true);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn is_left_carry(&self) -> bool
    {
        self.is_flag_bit_on(Self::LEFT_CARRY)
    }

    // pub fn set_right_carry(&mut self)
    /// Sets the `RIGHT_CARRY` status flag.
    ///
    /// # Implementation Details
    /// This method updates the internal state to indicate a carry-in from 
    /// the least significant end of an operation.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// ```
    #[inline]
    pub fn set_right_carry(&mut self)
    {
        self.set_flag_bit(Self::RIGHT_CARRY);
    }

    // pub fn reset_right_carry(&mut self)
    /// Resets the `RIGHT_CARRY` status flag.
    ///
    /// # Implementation Details
    /// This method clears the internal indicator for right-side carry bits.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// 
    /// a_biguint.reset_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn reset_right_carry(&mut self)
    {
        self.reset_flag_bit(Self::RIGHT_CARRY);
    }

    // pub fn is_right_carry(&self) -> bool
    /// Checks if the `RIGHT_CARRY` status flag is set.
    ///
    /// # Returns
    /// * `true` if a right-side carry has occurred.
    /// * `false` otherwise.
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("77255284354385016970177264758879158019392010587479561699232008238232688983808").unwrap();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.set_right_carry();
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77255284354385016970177264758879158019392010587479561699232008238232688983808");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), true);
    /// ```
    #[inline]
    pub fn is_right_carry(&self) -> bool
    {
        self.is_flag_bit_on(Self::RIGHT_CARRY)
    }
}
