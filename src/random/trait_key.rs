// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
// #![allow(unused_must_use)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![warn(rustdoc::missing_doc_code_examples)]



pub trait Key
{
    fn change_key(&mut self, sugar: &[u64; 8]);
}
