// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains implementation of external traits.

//#![warn(missing_docs)]
//#![warn(rustdoc::missing_doc_code_examples)]
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

use crate::symmetric::{ DES_Generic, NDES, SmallDES };


impl <S> Add<S> for NDES<S> where S: SmallDES + Clone + Sized
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: S) -> Self::Output
    {
        let mut smalldes = Vec::<S>::new();
        let mut ndes = self.clone();
        smalldes.push(rhs);
        ndes.set_small_des_vec(smalldes);
        ndes
    }
}

impl <S> Sub<S> for NDES<S> where S: SmallDES + Clone + Sized
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: S) -> Self::Output
    {
        let mut rhs = rhs.clone();
        rhs.turn_inverse();
        let mut smalldes = Vec::<S>::new();
        smalldes.push(rhs);
        self.set_small_des_vec(smalldes);
        self
    }
}

                