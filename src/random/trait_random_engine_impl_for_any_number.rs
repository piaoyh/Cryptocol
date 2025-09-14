// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::random::{ Random_Engine, AnyNumber_Engine_C_Generic, SALT };

impl<const MULTIPLIER: u64, const ADDER: u64>
Random_Engine for AnyNumber_Engine_C_Generic<MULTIPLIER, ADDER>
{
    // #[inline]
    // fn new_with<T, const N: usize>(message: &[T; N]) -> Self
    // where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    //     + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
    //     + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
    //     + Rem<Output=T> + RemAssign
    //     + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
    //     + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
    //     + BitXor<Output=T> + BitXorAssign + Not<Output=T>
    //     + PartialEq + PartialOrd
    // {
    //     let mut res = Self::new();
    //     if N == 8
    //     {
    //         for i in 0..8
    //             { res.set_any_number_(i, message[i].into_u64()); }
    //     }
    //     else
    //     {
    //         let len = if 8 * 8 < T::size_in_bytes() * N {8 *8} else {T::size_in_bytes() * N};
    //         unsafe { copy_nonoverlapping(message.as_ptr() as *const u8, res.as_mut_ptr() as *mut u8, len); }
    //     }
    //     res
    // }


    fn harvest(&mut self, restarted: bool, message: &[u64; 8]) -> [u64; 8]
    {
        let mut any_numbers = [0_u64; 8];
        let salt = if restarted { SALT } else { 0 };
        for i in 0..8
        {
            any_numbers[i] = message[i].wrapping_add(salt)
                                        .wrapping_mul(MULTIPLIER)
                                        .wrapping_add(ADDER)
                                        ^ salt;
        }
        any_numbers
    }
}
