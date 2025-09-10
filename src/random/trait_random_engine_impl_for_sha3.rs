// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::{ Debug, Display };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign,
                BitXor, BitXorAssign, Not, Shl };

use crate::number::{ SmallUInt, SharedArrays };
use crate::hash::Keccak_Generic;
use crate::random::{ Random_Engine, SALT };


impl<const RATE: usize, const PADDING: usize, const ROUNDS: usize, T, const LFSR: u8,
        const THETA_SUB: usize, const THETA_ADD: usize, const THETA_ROT: u32,
        const RHO_MUL_X: usize, const RHO_MUL_Y: usize, const RHO_T: u32,
        const PI_MUL_X: usize, const PI_MUL_Y: usize,
        const CHI_ADD_1: usize, const CHI_ADD_2: usize>
Random_Engine for Keccak_Generic<RATE, PADDING, ROUNDS, T, LFSR,
                                    THETA_SUB, THETA_ADD, THETA_ROT,
                                    RHO_MUL_X, RHO_MUL_Y, RHO_T,
                                    PI_MUL_X, PI_MUL_Y, CHI_ADD_1, CHI_ADD_2>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    fn sow_array(&mut self, message: &[u64; 8])
    {
        self.digest_array(message);
    }

    fn harvest(&mut self, sugar: bool, _: &[u64; 8]) -> [u64; 8]
    {
        self.tangle(if sugar {SALT} else {0});
        let src: [u8; 64] = self.get_hash_value_in_array();
        unsafe { SharedArrays::<u64, 8, u8, 64>::from_src(&src).des }
    }
}
