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
#![allow(unused_imports)]
#![allow(non_snake_case)]

///// For test during implementation //////
pub fn main()
{
    aes_sbox();
    aes_invsbox();
}

fn aes_sbox()
{
    println!("aes_sbox");
    use cryptocol::symmetric::AES_128;
    AES_128::show_SBox();
    println!("-------------------------------");
}

fn aes_invsbox()
{
    println!("aes_invsbox");
    use cryptocol::symmetric::AES_128;
    AES_128::show_InvSBox();
    println!("-------------------------------");
}