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
pub trait BigUInt_More: Clone + Sized //+ Display + + ToString
{
    
}