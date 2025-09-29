// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.



use crate::symmetric::BigCryptor128;
use crate::random::Key;


impl Key for BigCryptor128
{
    fn change_key(&mut self, _: &[u64; 8])
    {
        self.move_to_next_key();
    }
}
