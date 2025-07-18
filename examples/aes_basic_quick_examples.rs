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
    aes_devel();
}

fn aes_devel()
{
    aes_sbox();
    aes_invsbox();
    aes_mc();
    aes_invmc();
    aes_rc();
    aes_test();
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

fn aes_mc()
{
    println!("aes_mc");
    use cryptocol::symmetric::AES_128;
    AES_128::show_MC();
    println!("-------------------------------");
}

fn aes_invmc()
{
    println!("aes_invmc");
    use cryptocol::symmetric::AES_128;
    AES_128::show_InvMC();
    println!("-------------------------------");
}

fn aes_rc()
{
    println!("aes_rc");
    use cryptocol::symmetric::AES_128;
    AES_128::show_RC();
    println!("-------------------------------");
}

fn aes_test()
{
    println!("aes_test");
    use cryptocol::number::LongerUnion;
    use cryptocol::symmetric::{ AES_128, AES_192, AES_256 };

    // AES_192::new_with_key([0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 0x80, 0x90, 0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b]);

    // AES-128
    let mut aes = AES_128::new();
    let msg = LongerUnion::new_with_ubytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    print!("Message =\t");
    for i in 0..16
        { print!("{:02x}", msg.get_ubyte_(i)); }
    println!();
    let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
    print!("Cipher =\t");
    for i in 0..16
        { print!("{:02x}", cipher.get_ubyte_(i)); }
    println!();
    assert_eq!(cipher.get(), 0x3ad78e726c1ec02b7ebfe92b23d9ec34_u128.to_be());
    let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
    print!("Restored =\t");
    for i in 0..16
        { print!("{:02x}", restored.get_ubyte_(i)); }
    println!();
    assert_eq!(restored.get(), 0x80000000000000000000000000000000_u128.to_be());
    
    let mut aes = AES_128::new_with_key_u128(0x10a58869d74be5a374cf867cfb473859_u128.to_be());
    let msg = LongerUnion::new();
    print!("Message =\t");
    for i in 0..16
        { print!("{:02x}", msg.get_ubyte_(i)); }
    println!();
    let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
    print!("Cipher =\t");
    for i in 0..16
        { print!("{:02x}", cipher.get_ubyte_(i)); }
    println!();
    assert_eq!(cipher.get(), 0x6d251e6944b051e04eaa6fb4dbf78465_u128.to_be());
    let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
    print!("Restored =\t");
    for i in 0..16
        { print!("{:02x}", restored.get_ubyte_(i)); }
    println!();
    assert_eq!(restored.get(), 0x00000000000000000000000000000000_u128.to_be());

    let mut aes = AES_128::new_with_key_u128(0x2b7e151628aed2a6abf7158809cf4f3c_u128.to_be());
    let msg = LongerUnion::new_with_ubytes([0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34]);
    print!("Message =\t");
    for i in 0..16
        { print!("{:02x}", msg.get_ubyte_(i)); }
    println!();
    let cipher = LongerUnion::new_with(aes.encrypt_u128(msg.get()));
    print!("Cipher =\t");
    for i in 0..16
        { print!("{:02x}", cipher.get_ubyte_(i)); }
    println!();
    assert_eq!(cipher.get(), 0x3925841d02dc09fbdc118597196a0b32_u128.to_be());
    let restored = LongerUnion::new_with(aes.decrypt_u128(cipher.get()));
    print!("Restored =\t");
    for i in 0..16
        { print!("{:02x}", restored.get_ubyte_(i)); }
    println!();
    assert_eq!(restored.get(), 0x3243f6a8885a308d313198a2e0370734_u128.to_be());
    println!("-------------------------------");
}