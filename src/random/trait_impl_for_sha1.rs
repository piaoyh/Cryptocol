// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use crate::hash::SHA1_Generic;
use crate::random::Random_Engine;


impl<const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const H4: u32, const ROUND: usize, const K0: u32, const K1: u32,
    const K2: u32, const K3: u32, const RL1: u32, const RL5: u32, const RL30: u32>
Random_Engine for SHA1_Generic<5, H0, H1, H2, H3, H4,
                                ROUND, K0, K1, K2, K3, RL1, RL5, RL30>
{
    fn sow_array(&mut self, message: &[u64; 8]) -> [u64; 8]
    {
        self.digest_array(message);
        let a: [u32; 5] = self.get_hash_value_in_array();
        let mut res = [0_u64; 8];
        for i in 0..5
            { res[i] = ((a[i] as u64) << 32) | (a[i] as u64); }
        for i in 0..3
            { res[i+5] = ((a[i] as u64) << 32) | (a[i] as u64); }
        res
    }

    fn harvest(&mut self, tangling: u64, _: &[u64; 8]) -> [u64; 8]
    {
        self.tangle(tangling);
        let a: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let b: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let c: [u32; 5] = self.get_hash_value_in_array();
        self.tangle(tangling);
        let d: [u32; 5] = self.get_hash_value_in_array();
        let mut res = [0_u64; 8];
        for i in 0..5
            { res[i] = ((a[i] as u64) << 32) | (b[i] as u64); }
        for i in 0..3
            { res[i+5] = ((c[i] as u64) << 32) | (d[i] as u64); }
        res
    }
}

