// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]

use std::ptr::{ copy_nonoverlapping, copy };use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, RangeBounds };


use crate::number::{ BigUInt, BigUInt_Prime, BigUInt_Modular,
                    SmallUInt, IntUnion, LongUnion, LongerUnion };
use crate::asymmetric::{ PKCS115, RSA_Generic };
use crate::define_utypes_with;

impl<const N: usize, T, const MR: usize> PKCS115 for RSA_Generic<N, T, MR>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> bool
    {
        let size = T::size_in_bytes() as usize * N;
        if length_in_bytes > size as u64 - 11
            { return false; }

        const BT: u8 = 2;
        let mut m = [T::zero(); N];
        let count = size - length_in_bytes as usize;
        let ptr = unsafe { (m.as_mut_ptr() as *mut u8).add(count) };
        unsafe {
            *((m.as_mut_ptr() as *mut u8).add(1)) = BT;
            for i in 2..count-2
                { *((m.as_mut_ptr() as *mut u8).add(i)) = i as u8; }
            copy_nonoverlapping(message, ptr, length_in_bytes as usize);
        }
        let c = self.encrypt_unit(&m);
        unsafe { copy_nonoverlapping(c.as_ptr() as *const u8, cipher, size); }
        true
    }

    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> bool
    {
        true
    }
}
