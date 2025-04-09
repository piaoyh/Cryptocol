// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::random::Random_Engine;
use crate::hash::MD4_Generic;

impl<const H0: u32, const H1: u32, const H2: u32, const H3: u32,
        const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
        const R00: u32, const R01: u32, const R02: u32, const R03: u32,
        const R10: u32, const R11: u32, const R12: u32, const R13: u32,
        const R20: u32, const R21: u32, const R22: u32, const R23: u32>
Random_Engine for MD4_Generic<4, H0, H1, H2, H3,
                                ROUND, K0, K1, K2, 
                                R00, R01, R02, R03,
                                R10, R11, R12, R13,
                                R20, R21, R22, R23>
{

    fn sow_array(&mut self, message: &[u64; 8])
    {
        self.digest_array(message);
    }

    fn harvest(&mut self, sugar: u64, _: &[u64; 8]) -> [u64; 8]
    {
        self.tangle(sugar);
        let a: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(sugar);
        let b: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(sugar);
        let c: [u32; 4] = self.get_hash_value_in_array();
        self.tangle(sugar);
        let d: [u32; 4] = self.get_hash_value_in_array();
        let mut res = [0_u64; 8];
        for i in 0..4
            { res[i] = ((a[i] as u64) << 32) | (b[i] as u64); }
        for i in 0..4
            { res[i+4] = ((c[i] as u64) << 32) | (d[i] as u64); }
        res
    }
}
