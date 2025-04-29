// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]

pub fn main()
{
    sha3_rc();
    // sha3_develop();
    sha3_224();
    sha3_256();
    sha3_384();
    sha3_512();
    shake_128();
    shake_256();
    keccak_224();
    keccak_256();
    keccak_384();
    keccak_512();
}

fn sha3_rc()
{
    println!("sha3_rc");
    fn register(reg: u8, rule: u8) -> (u8, u8)
    {
        ((reg >> 1) | (((reg & rule).count_ones() as u8) << 7), reg & 1)
    }

    fn make_rc(L: usize) -> [u128; 26]
    {
        let ROUNDS: usize = 12 + 2 * L;
        let WIDTH: usize = 1 << L; // = 2_usize.pow(L as u32);
        let mut RC = [0_u128; 26];
        let mut bit = [0_usize; 7];
        for j in 0..7_usize
            { bit[j] = ((1_usize << j) - 1) % WIDTH; }
        let mut state = 1_u8;
        let mut output = 0_u8;
        for i in 0..ROUNDS
        {
            for j in 0..7_usize
            {
                (state, output) = register(state, 0b_0111_0001);
                if output != 0
                    { RC[i] |= 1_u128 << bit[j]; }
            }
        }
        RC
    }

    println!("----------------\nL = 6");
    let rc = make_rc(6);
    for i in 0..24
    {
        println!("RC[{}] = {:#018X}", i, rc[i]);
    }

    let r: [u128; 26] = [
                0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 
                0x8000000080008000, 0x000000000000808B, 0x0000000080000001, 
                0x8000000080008081, 0x8000000000008009, 0x000000000000008A, 
                0x0000000000000088, 0x0000000080008009, 0x000000008000000A, 
                0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 
                0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 
                0x000000000000800A, 0x800000008000000A, 0x8000000080008081, 
                0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
                0, 0 ];
    assert_eq!(rc, r);

    println!("---------------\nL = 5");
    let rc = make_rc(5);
    for i in 0..22
    {
        println!("RC[{}] = {:#010X}", i, rc[i]);
    }

    let r: [u128; 26] = [
                0x00000001, 0x00008082, 0x8000808A, 0x80008000,
                0x0000808B, 0x80000001, 0x80008081, 0x80008009,
                0x0000008A, 0x00000088, 0x80008009, 0x8000000A, 
                0x8000808B, 0x8000008B, 0x80008089, 0x80008003, 
                0x80008002, 0x80000080, 0x0000800A, 0x8000000A,
                0x80008081, 0x80008080, 0, 0, 0, 0 ];
    assert_eq!(rc, r);
    println!("======================");
}

// fn sha3_develop()
// {
//     println!("sha3_develop");
//     use cryptocol::hash::SHA3_224;
//     let mut sha3 = SHA3_224::new();
//     let mut block = [0u8; 144];
//     block[0] = 0b10101100;
//     sha3._initialize_state();
//     sha3._feed_block_to_state(block.as_ptr());
//     sha3._theta();
//     sha3._rho();
//     sha3._pi();
//     sha3._chi();
//     sha3._iota(1);
//     sha3._show_state();
//     println!("======================");
// }

fn sha3_224()
{
    println!("sha3_224");
    use cryptocol::hash::SHA3_224;

    let mut sha3 = SHA3_224::new();
    sha3.absorb_str("");
    let txt = sha3.get_hash_value_in_string(224 / 8);
    println!("sha3 = {}", txt);
    assert_eq!(txt, "6B4E03423667DBB73B6E15454F0EB1ABD4597F9A1B078E3F5B5A6BC7");
    println!("======================");
}

fn sha3_256()
{
    println!("sha3_256");
    use cryptocol::hash::SHA3_256;
    
    let mut sha3 = SHA3_256::new();
    sha3.absorb_str("");
    let txt = sha3.get_hash_value_in_string(256 / 8);
    println!("sha3 = {}", txt);
    assert_eq!(txt, "A7FFC6F8BF1ED76651C14756A061D662F580FF4DE43B49FA82D80A4B80F8434A");
    println!("======================");
}

fn sha3_384()
{
    println!("sha3_384");
    use cryptocol::hash::SHA3_384;
    
    let mut sha3 = SHA3_384::new();
    sha3.absorb_str("");
    let txt = sha3.get_hash_value_in_string(384 / 8);
    println!("sha3 = {}", txt);
    assert_eq!(txt, "0C63A75B845E4F7D01107D852E4C2485C51A50AAAA94FC61995E71BBEE983A2AC3713831264ADB47FB6BD1E058D5F004");
    println!("======================");
}

fn sha3_512()
{
    println!("sha3_512");
    use cryptocol::hash::SHA3_512;
    
    let mut sha3 = SHA3_512::new();
    sha3.absorb_str("");
    let txt = sha3.get_hash_value_in_string(512 / 8);
    println!("sha3 = {}", txt);
    assert_eq!(txt, "A69F73CCA23A9AC5C8B567DC185A756E97C982164FE25859E0D1DCC1475C80A615B2123AF1F5F94C11E3E9402C3AC558F500199D95B6D3E301758586281DCD26");
    println!("======================");
}

fn shake_128()
{
    println!("shake_128");
    use cryptocol::hash::SHAKE_128;
    
    let mut shake = SHAKE_128::new();
    shake.absorb_str("");
    let txt = shake.get_hash_value_in_string(1024 / 8);
    println!("shake = {}", txt);
    assert_eq!(txt, "7F9C2BA4E88F827D616045507605853ED73B8093F6EFBC88EB1A6EACFA66EF263CB1EEA988004B93103CFB0AEEFD2A686E01FA4A58E8A3639CA8A1E3F9AE57E235B8CC873C23DC62B8D260169AFA2F75AB916A58D974918835D25E6A435085B2BADFD6DFAAC359A5EFBB7BCC4B59D538DF9A04302E10C8BC1CBF1A0B3A5120EA");
    println!("======================");
}

fn shake_256()
{
    println!("shake_256");
    use cryptocol::hash::SHAKE_256;
    
    let mut shake = SHAKE_256::new();
    shake.absorb_str("");
    let txt = shake.get_hash_value_in_string(1024 / 8);
    println!("shake = {}", txt);
    assert_eq!(txt, "46B9DD2B0BA88D13233B3FEB743EEB243FCD52EA62B81B82B50C27646ED5762FD75DC4DDD8C0F200CB05019D67B592F6FC821C49479AB48640292EACB3B7C4BE141E96616FB13957692CC7EDD0B45AE3DC07223C8E92937BEF84BC0EAB862853349EC75546F58FB7C2775C38462C5010D846C185C15111E595522A6BCD16CF86");
    println!("======================");
}

fn keccak_224()
{
    println!("keccak_224");
    use cryptocol::hash::KECCAK_224;

    let mut keccak = KECCAK_224::new();
    keccak.absorb_str("");
    let txt = keccak.get_hash_value_in_string(224 / 8);
    println!("keccak = {}", txt);
    assert_eq!(txt, "F71837502BA8E10837BDD8D365ADB85591895602FC552B48B7390ABD");
    println!("======================");
}

fn keccak_256()
{
    println!("keccak_256");
    use cryptocol::hash::KECCAK_256;
    
    let mut keccak = KECCAK_256::new();
    keccak.absorb_str("");
    let txt = keccak.get_hash_value_in_string(256 / 8);
    println!("keccak = {}", txt);
    assert_eq!(txt, "C5D2460186F7233C927E7DB2DCC703C0E500B653CA82273B7BFAD8045D85A470");
    println!("======================");
}

fn keccak_384()
{
    println!("keccak_384");
    use cryptocol::hash::KECCAK_384;
    
    let mut keccak = KECCAK_384::new();
    keccak.absorb_str("");
    let txt = keccak.get_hash_value_in_string(384 / 8);
    println!("keccak = {}", txt);
    assert_eq!(txt, "2C23146A63A29ACF99E73B88F8C24EAA7DC60AA771780CCC006AFBFA8FE2479B2DD2B21362337441AC12B515911957FF");
    println!("======================");
}

fn keccak_512()
{
    println!("keccak_512");
    use cryptocol::hash::KECCAK_512;
    
    let mut keccak = KECCAK_512::new();
    keccak.absorb_str("");
    let txt = keccak.get_hash_value_in_string(512 / 8);
    println!("keccak = {}", txt);
    assert_eq!(txt, "0EAB42DE4C3CEB9235FC91ACFFE746B29C29A8C366B7C60E4E67C466F36A4304C00FA9CAF9D87976BA469BCBE06713B435F091EF2769FB160CDAB33D3670680E");
    println!("======================");
}
