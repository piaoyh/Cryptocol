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

use std::ptr::copy_nonoverlapping;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };


use crate::number::SmallUInt;
use crate::random::Any_Num as Random;
use crate::asymmetric::{ PKCS1V15, RSA_Generic };

impl<const N: usize, T, const MR: usize> PKCS1V15 for RSA_Generic<N, T, MR>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    fn encrypt(&mut self, message: *const u8, length_in_bytes: u64, cipher: *mut u8) -> u64
    {
        let size = T::size_in_bytes() as usize * N;
        if length_in_bytes > size as u64 - 11
            { return 0; }

        let mut m = [T::zero(); N];
        let count = size - length_in_bytes as usize;
        unsafe {
            *((m.as_mut_ptr() as *mut u8).add(1)) = Self::BT;
            let mut any = Random::new();
            for i in 2..count-1
            {
                let mut r = any.random_u8();
                while r == 0
                    { r = any.random_u8(); }
                *((m.as_mut_ptr() as *mut u8).add(i)) = r;
            }
            let ptr = (m.as_mut_ptr() as *mut u8).add(count);
            copy_nonoverlapping(message, ptr, length_in_bytes as usize);
        }
        let c = self.encrypt_unit(&m);
        unsafe { copy_nonoverlapping(c.as_ptr() as *const u8, cipher, size); }
        size as u64
    }

    fn decrypt(&mut self, cipher: *const u8, message: *mut u8) -> u64
    {
        let size = T::size_in_bytes() as usize * N;
        let mut c = [T::zero(); N];
        unsafe { copy_nonoverlapping(cipher, c.as_mut_ptr() as *mut u8, size); }
        let m = self.decrypt_unit(&c);println!("m = {:x?}", m);
        let ptr = m.as_ptr() as *const u8;
        let mut len = 0_usize;
        unsafe {
            if (*ptr != 0) || (*(ptr.add(1)) != Self::BT)
                { println!("Error"); return 0; }
            for i in 2..size
            {
                if *ptr.add(i) == 0
                    { len = i + 1; }
            }
            if len < 12
                { return 0; }
            copy_nonoverlapping(ptr.add(len), message, size - len);
        }
        (size - len) as u64
    }
}
