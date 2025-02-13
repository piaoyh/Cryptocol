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

use std::mem::size_of;
use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, RangeBounds };

use crate::number::{ SmallUInt, LongerUnion, SharedValues, SharedArrays, NumberErr };

// use crate::number::{ ShortUnion, IntUnion, LongUnion, LongerUnion, SizeUnion };

/// # Introduction
/// Trait `Expanded` is for BigUInt
///
/// # Quick start
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::Expanded` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::Expanded;
/// ```
/// If you import (use) `cryptocol::number::Expanded`, all the methods of
/// `Expanded` are available immediately and automagically, as if such
/// primitive data types had the methods from the begining.
/// 
/// ## Example 2
/// ```
/// // to do
/// ```
pub trait Expanded: Clone + Sized //+ Display + + ToString
{
    /*** ADDITION ***/

    /// For information on the implementation for BigUInt, [read here](click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_uint)
    fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    /// For information on the implementation for BigUInt, [read here](click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.overflowing_add_assign_uint)
    fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    /// For information on the implementation for BigUInt, [read here](click [here](../documentation/big_uint_arithmetic_uint/struct.BigUInt.html#method.checked_add_uint)
    fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.overflowing_add_assign)
    fn overflowing_add_assign(&mut self, rhs: &Self) -> bool;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.checked_add)
    fn checked_add(&self, rhs: &Self) -> Option<Self>;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.unchecked_add)
    fn unchecked_add(&self, rhs: &Self) -> Self;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.saturating_add)
    fn saturating_add(&self, rhs: &Self) -> Self;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.saturating_add_assign)
    fn saturating_add_assign(&mut self, rhs: &Self);

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.modular_add)
    fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.modular_add_assign)
    fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self);

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.panic_free_modular_add)
    fn panic_free_modular_add(&self, rhs: &Self, modulo: &Self) -> Self;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.panic_free_modular_add_assign)
    fn panic_free_modular_add_assign(&mut self, rhs: &Self, modulo: &Self);

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.safe_add)
    fn safe_add(&self, rhs: &Self) -> Self;

    /// For information on the implementation for BigUInt, [read here](../big_uint/struct.BigUInt.html#method.safe_add_assign)
    fn safe_add_assign(&mut self, rhs: &Self);


    /*** SUBTRACTION ***/

    fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool);
    fn borrowing_sub_assign(&self, rhs: &Self, borrow: bool) -> bool;
    fn wrapping_sub(&self, rhs: &Self) -> Self;
    fn wrapping_sub_assign(&mut self, rhs: &Self);
    fn overflowing_sub(&self, rhs: &Self) -> (Self, bool);
    fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool;
    fn checked_sub(&self, rhs: &Self) -> Option<Self>;
    fn unchecked_sub(&self, rhs: &Self) -> Self;
    fn saturating_sub(&self, rhs: &Self) -> Self;
    fn saturating_sub_assign(&mut self, rhs: &Self);
    fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self;
    fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self);
    fn panic_free_modular_sub(&self, rhs: &Self, modulo: &Self) -> Self;
    fn panic_free_modular_sub_assign(&mut self, rhs: &Self, modulo: &Self);
    
}