// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


pub fn main()
{
    // sha3_rc();
    // sha3_develop();
    sha3_quick_start1();
    sha3_quick_start2();
    sha3_new();
    sha3_digest();
    sha3_digest_str();
    sha3_digest_string();
    sha3_digest_array();
    sha3_digest_vec();
    sha3_digest_customized();
    sha3_digest_str_customized();
    sha3_digest_string_customized();
    sha3_digest_array_customized();
    sha3_digest_vec_customized();
    sha3_get_hash_value_in_array();
    sha3_get_hash_value_in_vec();
    sha3_get_hash_code_in_vec();
    sha3_get_hash_value_in_string();
    sha3_get_hash_code_in_string();
    sha3_push_hash_value_in_array();
    sha3_get_hash_value();
    sha3_read_hash_value_in_hexadecimal();
    sha3_squeeze();
    sha3_absorb();
    sha3_absorb_str();
    sha3_absorb_string();
    sha3_absorb_array();
    sha3_absorb_vec();
    sha3_absorb_customized();
    sha3_absorb_str_customized();
    sha3_absorb_string_customized();
    sha3_absorb_array_customized();
    sha3_absorb_vec_customized();
    sha3_tangle();
    sha3_get_desirable_l();
    sha3_get_desirable_rounds();
    sha3_get_desirable_b();
    sha3_get_desirable_output_length();
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
    cshake_128();
    cshake_256();
}

// fn sha3_rc()
// {
//     println!("sha3_rc");
//     fn register(reg: u8, rule: u8) -> (u8, u8)
//     {
//         ((reg >> 1) | (((reg & rule).count_ones() as u8) << 7), reg & 1)
//     }

//     #[allow(non_snake_case)]
//     fn make_rc(L: usize) -> [u128; 26]
//     {
//         let ROUNDS: usize = 12 + 2 * L;
//         let WIDTH: usize = 1 << L; // = 2_usize.pow(L as u32);
//         let mut RC = [0_u128; 26];
//         let mut bit = [0_usize; 7];
//         for j in 0..7_usize
//             { bit[j] = ((1_usize << j) - 1) % WIDTH; }
//         let mut state = 1_u8;
//         let mut output;
//         for i in 0..ROUNDS
//         {
//             for j in 0..7_usize
//             {
//                 (state, output) = register(state, 0b_0111_0001);
//                 if output != 0
//                     { RC[i] |= 1_u128 << bit[j]; }
//             }
//         }
//         RC
//     }

//     println!("----------------\nL = 6");
//     let rc = make_rc(6);
//     for i in 0..24
//     {
//         println!("RC[{}] = {:#018X}", i, rc[i]);
//     }

//     let r: [u128; 26] = [
//                 0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 
//                 0x8000000080008000, 0x000000000000808B, 0x0000000080000001, 
//                 0x8000000080008081, 0x8000000000008009, 0x000000000000008A, 
//                 0x0000000000000088, 0x0000000080008009, 0x000000008000000A, 
//                 0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 
//                 0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 
//                 0x000000000000800A, 0x800000008000000A, 0x8000000080008081, 
//                 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
//                 0, 0 ];
//     assert_eq!(rc, r);

//     println!("---------------\nL = 5");
//     let rc = make_rc(5);
//     for i in 0..22
//     {
//         println!("RC[{}] = {:#010X}", i, rc[i]);
//     }

//     let r: [u128; 26] = [
//                 0x00000001, 0x00008082, 0x8000808A, 0x80008000,
//                 0x0000808B, 0x80000001, 0x80008081, 0x80008009,
//                 0x0000008A, 0x00000088, 0x80008009, 0x8000000A, 
//                 0x8000808B, 0x8000008B, 0x80008089, 0x80008003, 
//                 0x80008002, 0x80000080, 0x0000800A, 0x8000000A,
//                 0x80008081, 0x80008080, 0, 0, 0, 0 ];
//     assert_eq!(rc, r);
//     println!("======================");
// }

// fn sha3_develop()
// {
//     println!("sha3_develop");
//     // use cryptocol::hash::SHA3_512;
//     // let mut sha3 = SHA3_512::new();
//     // sha3.digest_str("");
//     // let mut block = [0u8; 144];
//     // block[0] = 0b10101100;
//     // sha3._initialize_state();
//     // sha3._feed_block_to_state(block.as_ptr());
//     // sha3._theta();
//     // sha3._rho();
//     // sha3._pi();
//     // sha3._chi();
//     // sha3._iota(1);
//     // sha3._show_state();
//     // let hash = sha3.get_hash_value_in_string();
//     // println!("hash = {}", hash);
//     println!("======================");
// }

fn sha3_quick_start1()
{
    // SHA-3 Standard family
    use cryptocol::hash::SHA3_512;
    use cryptocol::hash::SHA3_384;
    use cryptocol::hash::SHA3_256;
    use cryptocol::hash::SHA3_224;
    use cryptocol::hash::SHAKE_256;
    use cryptocol::hash::SHAKE_128;
    use cryptocol::hash::cSHAKE_256;
    use cryptocol::hash::cSHAKE_128;

    // Keccak family
    use cryptocol::hash::KECCAK_512;
    use cryptocol::hash::KECCAK_384;
    use cryptocol::hash::KECCAK_256;
    use cryptocol::hash::KECCAK_224;
    use cryptocol::hash::BIG_KECCAK_1536;
    use cryptocol::hash::BIG_KECCAK_1024;
    use cryptocol::hash::BIG_KECCAK_768;
    use cryptocol::hash::BIG_KECCAK_512;
    use cryptocol::hash::BIG_KECCAK_384;
    use cryptocol::hash::BIG_KECCAK_256;
    use cryptocol::hash::BIG_KECCAK_224;
    use cryptocol::hash::SMALL_KECCAK_224;
    use cryptocol::hash::SMALL_KECCAK_256;
    use cryptocol::hash::SMALL_KECCAK_384;
    use cryptocol::hash::SMALLER_KECCAK_128;
    use cryptocol::hash::TINY_KECCAK_64;

    // Keccak-expanded versions
    use cryptocol::hash::SHA3_768;
    use cryptocol::hash::SHAKE_224;
    use cryptocol::hash::SHAKE_384;
    use cryptocol::hash::SHAKE_768;
    use cryptocol::hash::cSHAKE_224;
    use cryptocol::hash::cSHAKE_384;
    use cryptocol::hash::cSHAKE_768;
    use cryptocol::hash::BIG_SHA3_224;
    use cryptocol::hash::BIG_SHA3_256;
    use cryptocol::hash::BIG_SHA3_384;
    use cryptocol::hash::BIG_SHA3_512;
    use cryptocol::hash::BIG_SHA3_768;
    use cryptocol::hash::BIG_SHA3_1024;
    use cryptocol::hash::BIG_SHA3_1536;
    use cryptocol::hash::BIG_SHAKE_128;
    use cryptocol::hash::BIG_SHAKE_256;
    use cryptocol::hash::BIG_SHAKE_384;
    use cryptocol::hash::BIG_SHAKE_512;
    use cryptocol::hash::BIG_SHAKE_768;
    use cryptocol::hash::BIG_SHAKE_1024;
    use cryptocol::hash::BIG_SHAKE_1536;
    use cryptocol::hash::BIG_cSHAKE_128;
    use cryptocol::hash::BIG_cSHAKE_256;
    use cryptocol::hash::BIG_cSHAKE_384;
    use cryptocol::hash::BIG_cSHAKE_512;
    use cryptocol::hash::BIG_cSHAKE_768;
    use cryptocol::hash::BIG_cSHAKE_1024;
    use cryptocol::hash::BIG_cSHAKE_1536;
    use cryptocol::hash::SMALL_SHA3_224;
    use cryptocol::hash::SMALL_SHA3_256;
    use cryptocol::hash::SMALL_SHA3_384;
    use cryptocol::hash::SMALL_SHAKE_128;
    use cryptocol::hash::SMALL_SHAKE_224;
    use cryptocol::hash::SMALL_SHAKE_256;
    use cryptocol::hash::SMALL_cSHAKE_128;
    use cryptocol::hash::SMALL_cSHAKE_224;
    use cryptocol::hash::SMALL_cSHAKE_256;
    use cryptocol::hash::SMALLER_SHA3_128;
    use cryptocol::hash::SMALLER_SHAKE_128;
    use cryptocol::hash::SMALLER_cSHAKE_128;
    use cryptocol::hash::TINY_SHA3_64;
    use cryptocol::hash::TINY_SHAKE_64;
    use cryptocol::hash::TINY_cSHAKE_64;
}

fn sha3_quick_start2()
{
    println!("sha3_quick_start2");

    // SHA-3 Standard family
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();

    let mut txt = "";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash_value);
    assert_eq!(hash_value, "A69F73CCA23A9AC5C8B567DC185A756E97C982164FE25859E0D1DCC1475C80A615B2123AF1F5F94C11E3E9402C3AC558F500199D95B6D3E301758586281DCD26");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "F5F0EAA9CA3FD0C4E0D72A3471E4B71EDAABE2D01C4B25E16715004ED91E663A1750707CC9F04430F19B995F4ABA21B0EC878FC5C4EB838A18DF5BF9FDC949DF");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "4D8225A3EC677F44F3489B04925989BB18A9873446C8C122AC76019527E7A2324BD07D3CE5404649050F9DA05EEE8A6F2B64FDB05EA98BB77770A668D167EE0D");

    txt = "The length of this message is forty-eight bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "D3409ADAF35CF0D99EA0742BF50F84C6000F4B8CE84C76920CDADA6A077F4D274834AFADC43480D063CFD42E71860319F8436B7EDDFB03D682222A1AE1EA7B0E");

    txt = "The unit of the message length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "1D617BEE99571A8725B1D349F005B306B34637FDCF3D672D9311CA24083161697CDFFCC959C9FAFB7D75994653D37A8A097011C3F7700A0A4173364CAD6CB65A");

    txt = "This algorithm SHA3-512 is being tested with this message the length of which is one hundred eleven bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "A78FF3E9862D7962D20A350F60D67F6DC56D4996DDEFEEF5F23962F89B55F6E26140741D1ADE9C99F1CC2828F4A2CBD8D1E3EE17B1A07E28BD764667BF09DA72");

    txt = "This algorithm SHA3-512 is being tested for this message the length of which is one hundred sixty-four long so as to check whether or not this algorithm works well.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "3DD5B04FF485CA54787706E8CFCF6DBB57DD84452520575348960639EAE467D235858E00D038A0C88F56CF04FD39EC5B889B9D54B3F5F1C31D36A2050CF7C0CB");

    txt = "This algorithm SHA3-512 is being tested with this message the length of which is two hundred ninety-one long so that whether or not this algorithm works well is checked.  The message is 'Do you see a man skilled in his work? He will serve before kings; he will not serve before obscure men.'";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0A77A1191A768A1D0D7BF280F105D6399C2B8DE2280EF4BE307F99DDFC6A8895C8262A536405680DD01CC3699D5186043E406AE3FE01287A977EA4121F85BF53");
    println!("======================");
}

fn sha3_new()
{
    println!("sha3_new");

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let hash = SHA3_512::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let hash = SHAKE_256::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let hash = cSHAKE_128::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0000000000000000000000000000000000000000000000000000000000000000");

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let hash = KECCAK_384::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // Example for BIG_SHA3_1536
    use cryptocol::hash::BIG_SHA3_1536;
    let hash = BIG_SHA3_1536::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "00000000000000000000000000000000547969F9071B9AF02278D128944DD59C6BCCB273DBC1100F794F6488CB39D8EEB7953D954C8AC24A261368E226EA56166AA0B320613AAC9FD788A774ACBA3C71500157FE72A09D4F8C8198FF48495991D3DE92E4767FAACBB34AFB7786536E07DEF4A123AA97BC1BCFE2E34CDD60D15505B6DAA4FCF38CF9C206E86C18BE03AE31B1ADB2D0996CD729A4962E8B5EA592E3BBC024F2A0C9266A2005A25E82AE87583FE906E44469BDC2FC79C8A8B881F2");

    // Example for SMALL_KECCAK_384
    use cryptocol::hash::SMALL_KECCAK_384;
    let hash = SMALL_KECCAK_384::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0000000073309BF748B9DB9AC2563DABAFA463E1B027E3AC9BF40564EA67E3C85221FD7F8565B7B6FCF438DF69A3EE9F");

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let hash = SMALLER_SHAKE_128::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0000000000000000000000000000000000002A7A5328744912AEDDC0A8734314");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let hash = TINY_cSHAKE_64::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "000000000000000000368A9548AFF393");
    println!("======================");
}

fn sha3_digest()
{
    println!("sha3_digest");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "065B40EFFE93C55937ACA0C23D7A35387E0FDCA478C49D13255A59F685A2A53C");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "231FEC5DD3B64C278D1EC8BF8BFE4FF39E759AAC0AB54A4F435FADFA65ED7F72");

    // Example for cSHAKE_128
    // This example is not fit to the standard.
    // The method digest_customized() is better than this method to use for the type cSHAKE_128.
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "231FEC5DD3B64C278D1EC8BF8BFE4FF39E759AAC0AB54A4F435FADFA65ED7F72");

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let mut hash = KECCAK_384::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F8CF3D0877DA966AB6B96D98CD722103BB3E2477CA5DC9E8805541A99AAB5ECF1A8E6A885CC7E18FAEC4ED99CD759BCE");

    // Example for BIG_SHAKE_1536
    use cryptocol::hash::BIG_SHAKE_1536;
    let mut hash = BIG_SHAKE_1536::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E43978F22B924DB053195009F3C8C66E2382F0FDC43CCE9AE39A3322BD71F03F623B7FA3B3A601F46E3919D3739E30AAD4FAC593E4F6A3BFD8639A75EE5B77387E1D1DF0C4FBB13427E76CFE27A7B0F552B6625E8618B0D502CBE5CF4D389D127AA3F1737170444590AD9386E4C177FF53A60EB95253290EDF9F2BFE7AA42D1656030B01C7BADCE2AF6AE89686A47A75C9080DEE9AC84D4A71EBFE5304F4508270BCEDC3EC187325786E318563727476DE4E892D86647C5218F804A7B80C6085858AC7E2EEEDB13E4EB4829360157E02B5616A69558EF45B583F8865F0F96BA125DB016F9B569BD85D917ECB9CB4D2774CEC15450F2F784A994A0BEAD5BD0EB65C4CE42F645CA721D88F0FB3E1658468999BBE6E3A6FAC3879239CF5931E91186177DD639CFCFDE7789CBBC0C6E98B4672BED920FD261D1FCE6680D648D0FBC5C44B2850C98F40CD601847F6847332B8CD4E18E194E5FAB67B510AF994379D848E4B423D1684335D94A4867A474DBE4BD59C38DC4C1A8DFDC5144295638A14F8");

    // Example for SMALL_SHA3_384
    use cryptocol::hash::SMALL_SHA3_384;
    let mut hash = SMALL_SHA3_384::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "780AE19CD221EA8DFEE27F5B446CC3FA75F2D689F7673EFC445F64F2F8ECB6E630FA150BA10B672D5DAA3C46ABDD3C37");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "890BDD0CC5F02273BBD3CFBEF13484C1");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7804DAFDFCDB1CE0");
    println!("======================");
}

fn sha3_digest_str()
{
    println!("sha3_digest_str");

    // Example for SHA3_224
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BA8399261A38A097A69A072A9DE74FEAB248E5E2C93E622AC7E3381A");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B2FFABCAAE6C42F5FAD92B44035260ABD40157C8A37A0C3017EBA98441031F952A2E37E29A1588AD15A37584F672E3FEE0C0689E2F8DA44F144AAA23FCCDF623");

    // Example for cSHAKE_256
    // This example is not fit to the standard.
    // The method digest_str_customized() is better than this method to use for the type cSHAKE_256.
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B2FFABCAAE6C42F5FAD92B44035260ABD40157C8A37A0C3017EBA98441031F952A2E37E29A1588AD15A37584F672E3FEE0C0689E2F8DA44F144AAA23FCCDF623");

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3D214CB0DBD422DD9CF9A21DAC50E2B6547D62BAA8546252B5A66FD3653EAF797D32A7FF804667D021AF9659F09B2AD5C983F266BE828D7BD831FD355C0FFA52");

    // Example for BIG_cSHAKE_1024
    // This example is not fit to the standard.
    // The method digest_str_customized() is better than this method to use for the type cSHAKE_256.
    use cryptocol::hash::BIG_cSHAKE_1024;
    let mut hash = BIG_cSHAKE_1024::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9F51284305AE6E02A7A03E0A4BCA3D14AC4D9A43D7CDF4ADB9483282EE02E5A4FCBA0A07F008C29351FD60283F0ECEF3CBC5EB5E6C86BB380928EB7BC7D2D8213A73A3640636088ECBEC8322E932AE9DF461B4C25CF6706EACEFEF901408A969501F5A306FDE5A5BE505A1E504F2BBF9DEDAB44E02AE86D183D259CDF8CE72803D47E19EF4B33D8CCA1DD38616EF6907AC5B8F4B3F52756CF76BF397389B5F3D872A2EFE2AF89AD1A37FDE13F1C21A30F9BDFADC45C45B66C727E9F1E329DA63C9C57152F569FCD12ABF721185001600F8D262B62CF2ADA8D804232B68AFC3B4092A4B3FD685875F029989D48C669EF0020FEF2561AED9D14B3D268ECAC33CD2");

    // Example for SMALL_KECCAK_224
    use cryptocol::hash::SMALL_KECCAK_224;
    let mut hash = SMALL_KECCAK_224::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A574F12B85064E3ACA8ABB88F4859C3C9FBDE1F93C83ECD8AE550C578D24");

    // Example for SMALLER_cSHAKE_128
    // This example is not fit to the standard.
    // The method digest_str_customized() is better than this method to use for the type SMALLER_cSHAKE_128.
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "43D99F951BAD90AD59938C03F84E3FE81FC712A0F13ECEBF8ED72DF673476EE6");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "31CC3393C108D5C4");
    println!("======================");
}

fn sha3_digest_string()
{
    println!("sha3_digest_string");
    // Example for SHA3_384
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "34721672060C3F72C8FFD207E6D7ABA63CAA7A5BFEE0A695C7A11C423E8B14A27A61A967E3BACD041C4449F127533247");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "37E99A318DD958FB0EC077D77A08733192E890A7DA8BC39FBD04F64F49A9C8C0");

    // Example for cSHAKE_128
    // This example is not fit to the standard.
    // The method digest_string_customized() is better than this method to use for the type SMALLER_cSHAKE_128.
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "37E99A318DD958FB0EC077D77A08733192E890A7DA8BC39FBD04F64F49A9C8C0");

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0FD9EB73A653FDB9693C2D9028FA29BECD5F778C17115841777BEFC2451CC765");

    // Example for BIG_KECCAK_1536
    use cryptocol::hash::BIG_KECCAK_1536;
    let mut hash = BIG_KECCAK_1536::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "8E1BD7754AFA381F9BAEF2304A0EA7B4A0BCC8CEAC31DC649725452ABF0D0017554B28CA4EB8847A6CE264650068547C40748DC7774682110D5F4896796AA37A2E5925AF74F98CF85CFB340945B04BC79B2AF8353CCC84E76A218C6F0B34AB980BE9937C56BE81225CC6FE73C1F101C2980571A228903D76930A07FB22DA2C7323AC5B6AFCDA9BC16742F04A76C420C1358E462A1FE50F2341C03EEB4E07B7EEF2A4F1AE2CEA7FE51F812885A1297EFBBA1B92F678A2C9B951DCF0FF8FDBD8A0");

    // Example for SMALL_cSHAKE_128
    // This example is not fit to the standard.
    // The method digest_string_customized() is better than this method to use for the type SMALL_cSHAKE_128.
    use cryptocol::hash::SMALL_cSHAKE_128;
    let mut hash = SMALL_cSHAKE_128::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D9D7BD5F8EE9795A678519FEB1EEA98FBC476605C9AD6D84F67FBA470C18A3BE");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F0E533FEC8331BFEC34F1360ED90A80A");

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let txt = String::from("This is an example of the method digest_string().");
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "8D9972AC977AFEB67D374022892588C8");
    println!("======================");
}

fn sha3_digest_array()
{
    println!("sha3_digest_array");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");

    // Example for cSHAKE_256
    // This example is not fit to the standard.
    // The method digest_array_customized() is better than this method to use for the type cSHAKE_256.
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "84DC4C9F2C0B38A05C973A66B63EA7AEE8BBE1334E4C756AC6660717");

    // Example for BIG_SHA3_1024
    use cryptocol::hash::BIG_SHA3_1024;
    let mut hash = BIG_SHA3_1024::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "A99105325B0816D9D872CDB328F7E6C50EF3CE1C6C6B7FE10C7AA2416973195121349A2205711B7A29BF4FEBD6F654A0DAA664CC6528D02F4EE8E810973E88342AAA12876B40E79B69F717AE4D98916A16ADD5800772B70C9DD50B87E752AD595E398F5D327794A54DF2CB2C89C37A546260D76C356DF6FEBDAB21EED62941E0");

    // Example for SMALL_SHAKE_128
    use cryptocol::hash::SMALL_SHAKE_128;
    let mut hash = SMALL_SHAKE_128::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "AC39DB3C0CFD5A01F0289EB728BF157E0B312FCEDE39C1081E7A9211D316FCA7");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "8F94C2115CEFFD6C4DFEF1CE1E036CC5");

    // Example for TINY_cSHAKE_64
    // This example is not fit to the standard.
    // The method digest_array_customized() is better than this method to use for the type TINY_cSHAKE_64.
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "30A8CD9FEB02319FF224968B7A885D15");
    println!("======================");
}

fn sha3_digest_vec()
{
    println!("sha3_digest_vec");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");

    // Example for cSHAKE_256
    // This example is not fit to the standard.
    // The method digest_vec_customized() is better than this method to use for the type cSHAKE_256.
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "84DC4C9F2C0B38A05C973A66B63EA7AEE8BBE1334E4C756AC6660717");

    // Example for BIG_SHA3_1024
    use cryptocol::hash::BIG_SHA3_1024;
    let mut hash = BIG_SHA3_1024::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "A99105325B0816D9D872CDB328F7E6C50EF3CE1C6C6B7FE10C7AA2416973195121349A2205711B7A29BF4FEBD6F654A0DAA664CC6528D02F4EE8E810973E88342AAA12876B40E79B69F717AE4D98916A16ADD5800772B70C9DD50B87E752AD595E398F5D327794A54DF2CB2C89C37A546260D76C356DF6FEBDAB21EED62941E0");

    // Example for SMALL_SHAKE_128
    use cryptocol::hash::SMALL_SHAKE_128;
    let mut hash = SMALL_SHAKE_128::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "AC39DB3C0CFD5A01F0289EB728BF157E0B312FCEDE39C1081E7A9211D316FCA7");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "8F94C2115CEFFD6C4DFEF1CE1E036CC5");

    // Example for TINY_cSHAKE_64
    // This example is not fit to the standard.
    // The method digest_vec_customized() is better than this method to use for the type TINY_cSHAKE_64.
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "30A8CD9FEB02319FF224968B7A885D15");
    println!("======================");
}

fn sha3_digest_customized()
{
    println!("sha3_digest_customized");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64,  txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "4C3793B9B1CBA98DA30F71F0ABEEB6DB7D5B35318F17E5445BAEC565FADCB003");

    // Example for BIG_cSHAKE_1536
    use cryptocol::hash::BIG_cSHAKE_1536;
    let mut hash = BIG_cSHAKE_1536::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "60AC15376CFCC7DEF13C97238126DF635425972DEAF27D3212D107C663F2327EAB83B63BC007A7A049733BDC3783A76CAF38963B08B1C697EA622F5FADBE18495930A9F0F8EFD219811156ECD4712797BBCCD0EE92168564FD3C09965D8A6411D0328DFD1A7B3E446C63CE6A7220855A447BAC6C4D7665683D23E29EC209B9F72A779A5F84F4678605D79AD5A4EA09282283EBEDF37781F6C7D428FAEC4E2F640D14F22A9204252F6DE164837E0AF540661B3FA42A1C56FD2A95FC38C4838C90695C2D90F6819B8B7AEA4AC739D270EA07504ED62FFBAF426C2386534FE95F9348D58BC7454BA4802B5984790163F2B12ED2F0AC00CAAECD352344BC08CC1487183ADA924A1064FCB4BA59D82556F322A6A33CB39921641A7232D6B852039FC2B9651FFBD13E6CBA5F74714DC06965232A1B64F1E715CEF9932070EF746A1D43A142DF9AFC75357AFDF9022BF9332C688423CC7CFCBD9E82D83C6CED8B24833294AADD37D3438735D391B0648705E094553E3194E8402FFEF4303AD0372842EB");

    // Example for SMALL_cSHAKE_256
    use cryptocol::hash::SMALL_cSHAKE_256;
    let mut hash = SMALL_cSHAKE_256::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6C3E069C821A0526D5FF2EEB44A0B04A824CE6211A4664194C982E5A9EAC2F700FB684B6D0BA0B8E7D357709164C1265736C13C67E6AB4728CA57677F6949501");

    // Example for SMALLER_KECCAK_128
    // This example is not fit to the standard.
    // The method digest() is better than this method to use for the type SMALLER_KECCAK_128.
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E11D8A5C39AF9ECD2AF3CFBEC57F46DC");

    // Example for TINY_SHA3_64
    // This example is not fit to the standard.
    // The method digest() is better than this method to use for the type TINY_SHA3_64.
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EC07E55C6AD49B81");

    // Example for SHA3_512
    // This example is not fit to the standard.
    // The method digest() is better than this method to use for the type SHA3_512.
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "183F74D0A47CE0B3B533A903AC907FEED184D4E39F212F27EE0BF6B9E4E1B7CEAF105A165A6C9CC28DA27261194667B578B4B0B7626E1554340A297B133181C1");

    // Example for SHAKE_128
    // This example is not fit to the standard.
    // The method digest() is better than this method to use for the type SHAKE_128.
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A744B963C7F2CBAD52CBDBF2090173C4593D93B854581F5B623B060CEF4E013A");

    // Example for KECCAK_224
    // This example is not fit to the standard.
    // The method digest() is better than this method to use for the type KECCAK_224.
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F9D24FB9D6F617C993B9F155457683E0D4B26F7FC646C00A7E349FFB");
    println!("======================");
}

fn sha3_digest_str_customized()
{
    println!("sha3_digest_str_customized");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized("", user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3BB260278648858A59A25EE45AEA4E17A8DD7FAF51E32AEF4D3EA11739E38D4C9D22B7AE394D79E2A88BD2EFA4385E490836D0C6ED9D9087A3229F17F5E50EC9");

    // Example for BIG_cSHAKE_768
    use cryptocol::hash::BIG_cSHAKE_768;
    let mut hash = BIG_cSHAKE_768::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A0EFC73B6175215727A67CBD4B873579DBA7B5E3E5065B394449A7C31538C738361018179A3EFFAABB7BD1E50CFB02D6AEAF809EF51775126FAC1E35EB6CE844FCE1EAFB577D153D2100AFC4FBA51A3E1C418A9A337ED1BD68D13C6AEFE362D402C7A24F159BEF0610666038DE05C630F082E80F5C62FD865B523AB205E01F2E2D5A293CCFF27000D3D54800F9541CFA402FB2F77D23F0F3FC19118A8E0D93E93C7DFA74F94F280A367C2A15FE3FC471D68A544E470B6837AF381FCF6D3AA8BA");

    // Example for SMALL_cSHAKE_128
    use cryptocol::hash::SMALL_cSHAKE_128;
    let mut hash = SMALL_cSHAKE_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "7392046EECA855497C107E6C5510D3BA9FAEA15F363CA336115846C1E2E9DBD6");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "984227B7084637C4EFEFB795790C4792BA13C3288CE000F7FD84DF804FEC8F10");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "D49E308AD8F2DA82935F48CD2E9216F0");

    // Example for SHA3_224
    // This example is not fit to the standard.
    // The method digest_str() is better than this method to use for the type SHA3_224.
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EC429BB8857A6273E108EAEFA9435867D92C442CCF4B5309795068E9");

    // Example for SHAKE_256
    // This example is not fit to the standard.
    // The method digest_str() is better than this method to use for the type SHAKE_256.
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C072CDE8B6EA6F63BD756F6BE59A7C44CB51DDBA10E96C19C79FB1286AED9DDB4D3CD5F2CC94F3D7C0505F1888805D32C0CD4FBE10E311D72436576E485DE445");

    // Example for KECCAK_256
    // This example is not fit to the standard.
    // The method digest_str() is better than this method to use for the type KECCAK_256.
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "067791E671F1493BF93A2E1EAAD460E0FDF2176EA744FC433568C013A9F299C5");
    println!("======================");
}

fn sha3_digest_string_customized()
{
    println!("sha3_digest_string_customized");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&"".to_string(), &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "411E09A6E5CA61E99546226582C89FE0D6C3A57992173476C95F8BA1089EDF6D");

    // Example for BIG_cSHAKE_512
    use cryptocol::hash::BIG_cSHAKE_512;
    let mut hash = BIG_cSHAKE_512::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "21CAADA088F7712239A4AE7089C625319C84839335C4E9E199BA6DB522DD473A57B15A721C37284CADCFD25C74E123B49BB5C67EFDF9CD4FF13E4E4F25A9EF7CDC7187DE203B559D1442444FBD7824BD6C72F8750CAFC70ECC5989446B08B8C9180B7BD4997B028F5908431A75B4B89A98F18FA365AB3B58A10009F7EB0A2A2E");

    // Example for SMALL_cSHAKE_224
    use cryptocol::hash::SMALL_cSHAKE_224;
    let mut hash = SMALL_cSHAKE_224::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BE791B9B08B2080EDEDD1F1E72A59B5EDC33E973CDC903C1EB4D967451A9A4A92EFCA80B46AB1449D5D1B8A67C0E23CD0FCCE4BFADC4F16CA086C726AA");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3AC9E1D3453345DDDB0436C3CB699538075651560710FF5C51AD6462D9FBC114");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3E027E497CE48B64B7B776124CAF7929");

    // Example for SHA3_384
    // This example is not fit to the standard.
    // The method digest_string() is better than this method to use for the type SHA3_384.
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "370207DA06E2E85AAEC6C10E15DE9F92F8954AB036A0D10C48DB6A8D2FB5238EC209B2016BDAB94CBFE53FF3ECDF1178");

    // Example for SHAKE_128
    // This example is not fit to the standard.
    // The method digest_string() is better than this method to use for the type SHAKE_128.
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BA2EF98EA6944C201267441004368743340D1B32A895C1EF5364F00E0A7A8707");

    // Example for KECCAK_512
    // This example is not fit to the standard.
    // The method digest_string() is better than this method to use for the type KECCAK_512.
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C6FCC447C8ADCB04AA7229D3884A19EC6D5C44E96AA0AB62651CD0A8D71EFA2C24317F3DFFB3ABE3CA27D8686382C7C094DF464820671C4C841E04AB3A6F2CDB");
    println!("======================");
}

fn sha3_digest_array_customized()
{
    println!("sha3_digest_array_customized");
    
    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&[0_u8; 0], &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE9");

    // Example for BIG_cSHAKE_256
    use cryptocol::hash::BIG_cSHAKE_256;
    let mut hash = BIG_cSHAKE_256::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9C0EA4C581AB1E0235F18794F34C9BB1A022CA5841A74ADE62BC416B75EF0F2E4D063572823B101D2021273D6C98AF6501037FD0BE0DB8AB463FB822AE48ABA6");

    // Example for SMALL_cSHAKE_256
    use cryptocol::hash::SMALL_cSHAKE_256;
    let mut hash = SMALL_cSHAKE_256::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "20739EB80A727D4D6E87E4DDFF0223A24831686BF535BCD1007600DD0E08ED3854AF5C5355C1076C7AA9CD464D5EB1D6BFCA881BCFAF88AABE5F851F3368AD59");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "A336A171E044424D36267946F342A36377AFE1C3EFE6A5BB3C74A7C806B7AADE");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "D407B5886144579C87CF5A4AC8706FAB");

    // Example for SHA3_256
    // This example is not fit to the standard.
    // The method digest_array() is better than this method to use for the type SHA3_256.
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "372B387ECFB8C90E97E601C1D5C7E6D21CD9A78E3B608E7C9FC3083D7F04D29B");

    // Example for SHAKE_256
    // This example is not fit to the standard.
    // The method digest_array() is better than this method to use for the type SHAKE_256.
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "2141502E2CA8EC7B5B9E1D872A7B4399403FAAF83A428E63A1716696B6777D04C4108288F5CA09ADB2D9954062BADBF9ACCD251967F475E9A757C68B9824BA8D");

    // Example for KECCAK_256
    // This example is not fit to the standard.
    // The method digest_array() is better than this method to use for the type KECCAK_256.
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let function_name = [0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "83AA5FEAA9B371B8C3CB5EA7C509951E2C586DB5B117B1AEF7F2BC8A65A13E65");
    println!("======================");
}

fn sha3_digest_vec_customized()
{
    println!("sha3_digest_vec_customized");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE9");

    // Example for BIG_cSHAKE_256
    use cryptocol::hash::BIG_cSHAKE_256;
    let mut hash = BIG_cSHAKE_256::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "9C0EA4C581AB1E0235F18794F34C9BB1A022CA5841A74ADE62BC416B75EF0F2E4D063572823B101D2021273D6C98AF6501037FD0BE0DB8AB463FB822AE48ABA6");

    // Example for SMALL_cSHAKE_256
    use cryptocol::hash::SMALL_cSHAKE_256;
    let mut hash = SMALL_cSHAKE_256::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "20739EB80A727D4D6E87E4DDFF0223A24831686BF535BCD1007600DD0E08ED3854AF5C5355C1076C7AA9CD464D5EB1D6BFCA881BCFAF88AABE5F851F3368AD59");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "A336A171E044424D36267946F342A36377AFE1C3EFE6A5BB3C74A7C806B7AADE");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "D407B5886144579C87CF5A4AC8706FAB");

    // Example for SHA3_256
    // This example is not fit to the standard.
    // The method digest_vec() is better than this method to use for the type SHA3_256.
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "372B387ECFB8C90E97E601C1D5C7E6D21CD9A78E3B608E7C9FC3083D7F04D29B");

    // Example for SHAKE_256
    // This example is not fit to the standard.
    // The method digest_vec() is better than this method to use for the type SHAKE_256.
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "2141502E2CA8EC7B5B9E1D872A7B4399403FAAF83A428E63A1716696B6777D04C4108288F5CA09ADB2D9954062BADBF9ACCD251967F475E9A757C68B9824BA8D");

    // Example for KECCAK_256
    // This example is not fit to the standard.
    // The method digest_vec() is better than this method to use for the type KECCAK_256.
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let function_name = vec![0xFFFF_u16, 0xEEEE, 0xDDDD, 0xCCCC];
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec_customized(&function_name, &user_defined, &data);
    println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "83AA5FEAA9B371B8C3CB5EA7C509951E2C586DB5B117B1AEF7F2BC8A65A13E65");
    println!("======================");
}

fn sha3_get_hash_value_in_array()
{
    println!("sha3_get_hash_value_in_array");
    use std::io::Write;
    
    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_array::<64>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "4387084DCE64950435EC8196096E6F64FBAEC92755840886F67F6FC60D18A519C02A20614DD4D6218AB6837D3CE46288A2BF1AA17ECDD63117F908161989A90D");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value: [u8; 32] = hash.get_hash_value_in_array();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D6E9DF5CC717C5AF08969E6D1CA3224953BD1F220920D9E67D6381AA537CAD92");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_value_in_array::<32>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "602EE11896CE2A9BEF0E5E18557E2DC19B8A9F5A722644560B89C19561FC8279");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value: [u8; 56] = hash.get_hash_value_in_array();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "DF8028EB204ACB170349A9EED0A739C6C6C4C598DEF5F61E0E66397E5E02E986E38D107D27F467BCA2873B0CC193628E347B8489A2085720");

    // Example for BIG_cSHAKE_1024
    use cryptocol::hash::BIG_cSHAKE_1024;
    let mut hash = BIG_cSHAKE_1024::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_value_in_array::<300>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "9BCC9E0FFF3924D2919918A3F68713F02D7B44D5F55D06DAB69E1E35F2FB7422CFA675D7E7715D38E65FAC2256EE7CC7CEF69C0AC25F6F868275584FAD95B7F21D24D95CA0CADB075D68192DCBB241F4775EBA09C72108EAD1CEDBB7BB2EC1ED0C19078DB37B1F364848580A74BF3E1F77D44403F1C63DC7197FCBE7BD05C53FFFD4374F1A48A2E8650B5E13DEABB27109CCB0E468ED71766A09D4687DEA8D4498E895E8E2B074B0E9AA1157AE72FBBC2543D78F2A3C2974AFB35D38CEF3692558837FC410493A0DC166BD99ABA040DF90CF1CD635F91E231E3251B0F1392BF5E03BA1FB99CDBD0310D25CF4A31600679FF18C96C16F487D9C9DACFF095BDCF26E5AC652DE274A723AD41A7D1AD06D5E2E9DC73E6702112E5EEC1FA155D1402AE554D92E678E3CB153C2E353");

    // Example for SMALL_SHA3_256
    use cryptocol::hash::SMALL_SHA3_256;
    let mut hash = SMALL_SHA3_256::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value: [u8; 70] = hash.get_hash_value_in_array();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "BB0CE4DE7D50B05CF5B794CB069F73CB5D4D7175C700AD108EAF2067513AB96FEBD9A2F32C5D18788D15E0FA178D3A4ABB5817E821DFD7571212BBEED72D90C15F713401B072");

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let mut hash = SMALLER_SHAKE_128::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_array::<40>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F4C9A3EC5A340C4FBF9ECC211E13930FD96EB190CD2FA7DCCDEF1CAAB0D3B2F49AD7CF5350000453");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    let hash_value: [u8; 10] = hash.get_hash_value_in_array();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3B59168F8A5A42D59208");
    println!("======================");
}

fn sha3_get_hash_value_in_vec()
{
    println!("sha3_get_hash_value_in_vec");
    use std::io::Write;
    
    // Example for SHA3_224
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "49FA84569FC01A4FD42DC7D6A892A61539AA14761FDC679A14A8A365");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "7CD7F1ABBEFDCE9ACB5D4AF5AF766F7C72141453561AD2A80D2080085F06C7FF77C50CC90E0CEAA6D357183383500FF3DEF6B9AD495A8FE5F7CA253171159277");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "54E8FF89028EBA8EFB8418687CF3867D0221B3DD3DEEE5DDA992164D4B82B939A39E898AAE726676F8BBCCB9B352BC1A9B68ABEA80D2C47F8ABA0D0762F11E5F");

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let mut hash = KECCAK_384::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "1D7E5E5E173B19CF67B7678801AA913526F808B6E0742B20688B1BD8A5B5A11C640015B23D10FDC3BABD60DF5A410545");

    // Example for BIG_SHAKE_1024
    use cryptocol::hash::BIG_SHAKE_1024;
    let mut hash = BIG_SHAKE_1024::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "19A04F56959889991B6A2573080FD8FEBFA4E7E17F5F9D95DBA85CD486DE52B9E35C1EADE0D54A1EE280A4E66D7BD55466C915BAFCD5C28349EC17A6BAE454F7358266C6D4AE81F7FC7D2C3C4AAACA221838F9355A6555234B0A4106EA1A18AF319C1721190371E23F5C8BFF14F58550AC372705587744B8A769D4DEA6BC69DD6D00642EB8D7F85A4A1FB64E8461B54C2285A9B17DF0A3ECE3D6BB3B53C38BE032277E0BA71B0EE296D541924EB069E8B8C12666E6F6ECA4E19B19B096E44F5CA0258A7E59C7D9D52C6227B4661AED780BF835246EA5963382368838AFB44BF0E89E088E28C818F7143334DD922B711E5C94E02135383C5C64EF6B8615258A90");

    // Example for SMALL_SHAKE_256
    use cryptocol::hash::SMALL_SHAKE_256;
    let mut hash = SMALL_SHAKE_256::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D907949D69AE3FD395BECD8E997749FB58992008C989F5ADC5FA3DA555EBC2A598B513C699A301D0621CF47FC433FAEB22D13132E741295548E6FDCBD37084AA");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2244DF8EDDE0F4911FAE0297685C722C");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_value_in_vec();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "CC87D4276259419C");
    println!("======================");
}

fn sha3_get_hash_code_in_vec()
{
    println!("sha3_get_hash_code_in_vec");
    use std::io::Write;
    
    // Example for SHA3_384
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<48>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "4DF9726A50546589EDED01B0D6CAF4DB022B382C6B0B6229EAD2F75B743940A0993891C6E38DB84931AAC1EB2CFAC9F8");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<16>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3B3C3672FF31BEE1E2C43CEBDA1E19F2");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_code_in_vec::<16>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "4D38EFE9C53011DE5E0A1A8236838A00");

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<64>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F1CC061FEFA62EF4E9481717B868A85852561CFDF0803C9E66EBEC4FBB8E05672E5216F697D3BF3A429AB64451259A465CD78F5913435202C42710EEB3510D2A");

    // Example for BIG_SHA3_768
    use cryptocol::hash::BIG_SHA3_768;
    let mut hash = BIG_SHA3_768::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hash_value = hash.get_hash_code_in_vec::<96>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "60F10FF289A88D8852C6E35B216522AD80E4CA51B3113AE090D9F3A776230B54AD7EE9BA9586B5BB5A36E1CF55DD9ABC92C9C34DF144EE4A75C74ADC378882C970FF0800697E72ECC22CE653A25F8A619107F7A6097DAE12431302983BEDD32A");

    // Example for SMALL_SHAKE_224
    use cryptocol::hash::SMALL_SHAKE_224;
    let mut hash = SMALL_SHAKE_224::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<28>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "DA7856F7E058BF8785D36983DAD8E0CD63F67B10B55007495CE3E6E1");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<16>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "709EE0D2B084AFED94BC774C4F0BE31B");

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let txt = "This is an example of the method get_hash_code_in_vec().";
    hash.digest_str(txt);
    let hash_value = hash.get_hash_code_in_vec::<8>();
    let mut hs = String::new();
    for h in hash_value
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "4D2CF7FAC386B2B9");
    println!("======================");
}

fn sha3_get_hash_value_in_string()
{
    println!("sha3_get_hash_value_in_string");
    
    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2075C0B4930865BA252F5BA2A7DF5AC4AF587B9E054B8BCC249CED216AFAA459");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "9268E4476035969FA6E0B3D3ECF480B712CAFC6223B3E2C4DF5223B7C84692DEBB9A3FFB7BC411D74ADC58732630CE535F6C71200056DAA49E5FE57DBFFE2E87");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "30E2B813849025D827C9983EA9EFDC3B072EA6E8EC93548C7EBCB7AF827CBB47EC7A5E3908B88A9596B18B498895906D7876F984963F1B05C67B01C7E6F8900D");

    // Example for KECCAK_256
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "9FCCFF67F1C5D6F4B01FB29E3A2CBFE343A3E1DEC23D05C5C8EE422E6CC548CC");

    // Example for BIG_KECCAK_768
    use cryptocol::hash::BIG_KECCAK_768;
    let mut hash = BIG_KECCAK_768::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2C9DFD0270B22349579A075211B8A8BD1F0564E2BB686A33C8E6649669F9084957C2581B32DC105C61CC40E074F0B718F2ED61C0F40C72457A56D0C90A77E330D58386AF1D2DF5EBC4D6B1F1E0FD966524F8F1CAD57E2689C35DB8DBC1E3309C");

    // Example for SMALL_KECCAK_256
    use cryptocol::hash::SMALL_KECCAK_256;
    let mut hash = SMALL_KECCAK_256::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "583DF8ABCD3A5A9A06978441D41C4B0C332C77341EBD2011D6277CAB5AE50AFD");

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let mut hash = SMALLER_SHAKE_128::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "C3DAB03A198E0EEE5F32D1AA1D880C1A0F9FFCB6433E21F9952579A6F47448B5");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "93C59FC524197928");
    println!("======================");
}

fn sha3_get_hash_code_in_string()
{
    println!("sha3_get_hash_code_in_string");
    
    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F9DA1EAB4A2F7204BDA5E06DACDC23D98491CB5E313F7F74594F9852F8122DAFB03A715BE6836B17F6ACD9EC6A1BA12AD8F0C8C221A9BD20D0834AB78C6FB6A7");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(16);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "263E824E0C8D04149616D4F8C5E53567");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hs = hash.get_hash_code_in_string(16);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "8FFB73ED04238F85C05FC238347A7865");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(28);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "99E3EDC19CAB6F9D10EE88B560EC84DCF9B421DA12BD515F5A3FF38A");

    // Example for BIG_SHAKE_768
    use cryptocol::hash::BIG_SHAKE_768;
    let mut hash = BIG_SHAKE_768::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str_customized("", "On my purpose", txt);
    let hs = hash.get_hash_code_in_string(96);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D2FFC59F9C30C90CB09797E7387F4A7D9307988054C0A7D433CFD8062D0D13C33DE73E59C7A052752569B5ED95144B180D8D07E5DB507861E7672E47388DC6B021D267A1410BBBB5571EAC2EC9901BB1826EED5DDC02B57865428D206C0F77A7");

    // Example for SMALL_SHAKE_224
    use cryptocol::hash::SMALL_SHAKE_224;
    let mut hash = SMALL_SHAKE_224::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(28);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F900A4457D5691B9F55E2E655FBC92CBA5CA29DE49AE61DA93A90540");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(16);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F3F0A63CBFB028C9779B2AB2CBBA84B0");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let txt = "This is an example of the method get_hash_code_in_string().";
    hash.digest_str(txt);
    let hs = hash.get_hash_code_in_string(8);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "9900A025EB6C8C1A");
    println!("======================");
}

fn sha3_push_hash_value_in_array()
{
    println!("sha3_push_hash_value_in_array");
    use std::io::Write;
    
    // Example for SHA3_224
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 28];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "84C1FA767D0DB1DF9F886333681641A55253AB934A16B51376A5403C");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 32];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2543CE197EFAE3CFFCA5BDF1FD54C165CF6D985F082605674FB54B8E637A0F02");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str_customized("", "On my purpose", txt);
    let mut hash_value = [0_u8; 32];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "279E613EA73C29E9EA1262F2F0A25013B175FEDA3BA11C2518F912E5784DF49B");

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let mut hash = KECCAK_384::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 48];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "E4B35E50230F73BA59DC699353FD9241E8B367363BEBC7E5B99E4E1B24D548039C43C5D7F44143825EFA69DD1BBE0BAA");

    // Example for BIG_SHA3_512
    use cryptocol::hash::BIG_SHA3_512;
    let mut hash = BIG_SHA3_512::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str_customized("", "On my purpose", txt);
    let mut hash_value = [0_u8; 64];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "AE12A8F8E7DAB0ED91CAF577808D47B9DD0904918C80EA4531B7B0E68A21422F0CE281913C227F033BDE13562AA0284CB5CBCFEA240902D2BA6CE20B3E4B8B69");

    // Example for SMALL_SHA3_384
    use cryptocol::hash::SMALL_SHA3_384;
    let mut hash = SMALL_SHA3_384::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 48];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "DA8C0DA96CAFD8B0EDEE2C5C42FCB7F9C84BD2CEA4DE557D85148EA24B91E0283B25E8D273D1A2A604433AC3963C99D3");

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let mut hash = SMALLER_SHAKE_128::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 16];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "9F562477441E20B3AF95A1CC0BA7AB60");

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let txt = "This is an example of the method push_hash_value_in_array().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 8];
    hash.push_hash_value_in_array(&mut hash_value);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "64426F46B9C6CBA2");
    println!("======================");
}

fn sha3_get_hash_value()
{
    println!("sha3_get_hash_value");
    use std::io::Write;
    
    // Example for SHA3_384
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 48];
    hash.get_hash_value(hash_value.as_mut_ptr(), 48);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "96298277B7B1EB85520425DD38DA75B9C1CC5D4CF34FCFAD681C17D0BD9BCEEE02C4D3ED06E9575579BBCD4A2B8614AC");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 16];
    hash.get_hash_value(hash_value.as_mut_ptr(), 16);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "21E161B692F48432835D9F47BB23215B");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str_customized("", "On my purpose", txt);
    let mut hash_value = [0_u8; 16];
    hash.get_hash_value(hash_value.as_mut_ptr(), 16);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "6F904C59782586500E8ADF61F64E6888");

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 64];
    hash.get_hash_value(hash_value.as_mut_ptr(), 64);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "980565BE887AB97CAD841C3375418AA787EA7FC99D3577E9B13796662F7AC20D3BCCF25C02941F0790CE7B2EFD65ADB4EE46CD2B82780B44F024A24C48EC4637");

    // Example for BIG_KECCAK_512
    use cryptocol::hash::BIG_KECCAK_512;
    let mut hash = BIG_KECCAK_512::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str_customized("", "On my purpose", txt);
    let mut hash_value = [0_u8; 64];
    hash.get_hash_value(hash_value.as_mut_ptr(), 64);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3DE03545158E3402B0970C337E7F064C6797C0B8E47791BEAC64B123552DFC1E96E6B210098A80E9F9073F27D726E446C0380FE226E5BAA5E1CE0B74D109F4E9");

    // Example for SMALL_KECCAK_384
    use cryptocol::hash::SMALL_KECCAK_384;
    let mut hash = SMALL_KECCAK_384::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 48];
    hash.get_hash_value(hash_value.as_mut_ptr(), 48);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "50A3A87E0EFA0FA419364C9240A0CB19DDB81167908D4AF863725243FE85C3BF02EB48C53B31D05ACE19480558CBE7E1");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 16];
    hash.get_hash_value(hash_value.as_mut_ptr(), 48);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F85CA9D70ECD11D2F2E63CEE5E7C52F4");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "This is an example of the method get_hash_value().";
    hash.digest_str(txt);
    let mut hash_value = [0_u8; 8];
    hash.get_hash_value(hash_value.as_mut_ptr(), 48);
    let mut hs = String::new();
    for h in hash_value.iter()
        { unsafe { write!(hs.as_mut_vec(), "{:02X}", h); } }
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D948C3806BC9413F");
    println!("======================");
}

fn sha3_read_hash_value_in_hexadecimal()
{
    println!("sha3_read_hash_value_in_hexadecimal");
    use cryptocol::hash::SHA3_256;
    let hash_value = [01u8, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00, 0x11, 0x22, 0x33,0x44, 0x55, 0x66, 0x77];
    let hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "0123456789ABCDEF0011223344556677");
    println!("======================");
}

fn sha3_squeeze()
{
    println!("sha3_squeeze");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D035239B93FBE534A4B1C0923EB844D689E55ACFB1E13144E6FE7D69B75290F68502392D6138336847C2D52AB1AC84BC0F66B7501ED3705FE4811B106604A2A3704C563BB2BAC52B7FAE628AE96D7E38312B5779FE5E154FAD2116C8E58F97F42C202BBA92310B5DA0758BC2F4549B1EA1C43393FC0BD4BCF392BFB0B4B59883AC6B1D702B9563BA");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "BFC8C6A5B32F55DB7FE8A060D63F1872BC8F98D2279567E92F0DC58FCB30808CB673FAB3C69AFA064D6081FB8349563AF2B1580CE458147D34C52A1CE54CA6F7C49EECB06DACCEA31926186455348AB9980F556C4764563D4AABE3C5EAF575767C7CA18DB2FD5651D567EE6D127196F424D1AC61A7B7DB60EE5E211EF290B7AF8A015E6D07C51139");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "631720A8E88A894A34177078646CA9D081C65732E0D6587EBCC5E1B902E4D5B57ADCA15B8A55173A322DD552D28D2EB244AB60C63D6BE0C72206F0CB44F74F16FAA1CE1647426892424E82F7468C00B007C79859A858AC12C92FC8068ACF14EFD0825B3E1F3200EB922B652F2B1B34D2247051F77C5B2B050A1DB5BFA92F7D2C5951D9328DBFD3D3");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "DC064A78C927A328F555A29B2654151A336D28B13F35A427DC0BB58BDA500089ED5EC7045CA3CF71B8121F3B77AC5861613E2F4EF49A23A9D737C78B7256B03AB7F9C5FF7A40814294F465EC5A0F646152FC84CF62B150B4B7CC5246E28DF429EDB483B7B34070092C27537B6AB365A6D6F77DDC2A910BD2513229C9043661F62874912EF88F7984");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3AA1BC3B1E138AAB1D33943E1A9FBB617513F77EFFD4CF4A22826939BA58E4DE3E4AFAE74F7306AC8964723E63D6AF7A6CD3A1A43C218C9BA7C38720B7685B7539B5EB0E221419985D5343E4250C74700F89D19FC35EEC1371D9CD7182EE756B6AC601B5C0AB6F5A4930C3287DB2FBEF2AAC4796E92C99997A30D8C3FC3313F415A4707484853652");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "A0D0D7002F32CA7052C8857A8A01D96F4B1F6540C05BC319A3F070078A272C30825CEFB664418D751B2A4ED15A98C40A9E5D586EA58AEF5309C9D98C1163B7C743F8120C05B368E6025A741B96285B1B9573A23D3C843EB880926266BBD1D5B809921F4B144B3F71F67BDCAB169BC333EA833B60A0EFD851E3C6CB1DFB7F8F112CB6091DA3BE4602");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "1E14598686B883B3D3BC38F9FB4F685B3A8331F6FCBD926A68870587347BAF3F466F50E56F893068FDB03F2E836FCEDD4C35E192D5243A3CE07E0A0B8267B159327FA7A56A772D99A90240602E5B6CD22B2BDBA0AFD0666AB7F3FE070245BFA7DD29E620738300014B924BE69FE73FC3DDF9EEBA2A15ACFC649CDA07C45D721CC1064FB4CDAE39BF");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "231068315F26DAADB091348F1EB272D81992617E9F1D34A482B9288F6F37A92B482452ED12B315169FA7952FD517A2EF861F2B39F472D80ADBC854A8519961E8CBA8BAEFCE1D598AEF5DFFBA7D2B91C8DEC90CB1507823CF9415F115B0EC86AC4713AE46DD0086DB67E7120A4FD846CE8F14EBAD98B90E8DFA1FF6FF2F92F226270A12B0BCC40C91");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str_customized("", "On my purpose", txt);
    let mut hash_value = hash.squeeze();
    let mut hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "80705C01F992C61DC38B4B7047B3756EAD9DCECDA9B4ED361E7D45F02996CD34C123D9172E2F0B20698D8CACDECC71E93680C433FAA5C4E55A8E2D376106C764C4ACFC48333B7D089D81F0FFF71F77EF4D283B9F732CEE8908B8B23D6CCBB0567F938CA96D4A465C3CFDB4B3FA9CE09568C115251C4984A19E0FFF11F68A674DE18E8F3392DDF274");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "F08C73DA30FEC631E608DF5B039C67122A104D5EDB80672614A8138D022570EE99079C70CC5F93676328F563240DE575B940D7E47C849C783319E486D13384FEA3C511BD3928E4A6DDCF26E43488FC70A419E9942B2C8878D4D82FDDD8FF8B6A3D86D651DF3457A6323B51D365E0E5BC6799446F0C8D454CBDB38F0002475EE733419F442BA90A66");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "C7DBD008222C5941F65036BF476D66A720F81A9E55BB73C14397E1CDFF774ECA00E0A0486D52E7810213489531AC10FE5BC7EF06654A65D6DAFE16EB59E7C8A1EA6E1ED35F2FFAECB2DC16CACE4FDFA6F7BBD6D25E4274E6BA785EEA9B4ED0946A31E4BE8EFFF39D35BDE9CF94FAE2D078561536EDB61FCBA711FA2DB3A2DA8249F64FC2343C56BF");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "3CEA8227FB50794EBD600EF13AC18996B05E3D8C8C7A33B50819B96BCE3270F72C66E1F1887847E2E1D06EDD40B66FD1108188489646E70FD2601013D5C42D7FFD44EA14848C61DB2C3FB0BE8655BC223E80A4C91D14FC390588C5B13034909AA7350B2FF7683FCFF7596F8ACE2D74235D0940D17CE486C8AC30196D336FBF25140AE3740DDED56E");

    // Example for KECCAK_256
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = KECCAK_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "49AF4310FD92BD4B913987C979AE11F5CFBF52E41C1BF6E5219F18603D632ADD282356D55F3E9A49589257357BDC5F120BA5B2A0279C247CC77286BB83AE25C134153ED5B597841C79CE6A9BF1E7944AEB7E09F53BA60BEC37814999E23E8FE962E13FDBDC61E0DCD5628C394C4E08C918B666EA3EF32BAAAB5D9B9E572E679223836E7A35D4CFA1");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "42530E9B59F224D5A852BF1DE97A9CEED57F987840B6DF0EDBF44683C4D9BD9027F242DA9572CCDAE4AE5BDF30CF37E83EB61796E5B1219F0A76F36EF93DFE80FD2C950E2D83F296C0F71F088E21DB1EBAD9E91A91ABB2D48EBA7B23C7FD36AC97329C86CF494A096ED3B8C2EF9E378B5A1C41F24CCBAD2BE30B2705FC1B3CDDBD4608FDB915F122");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "A7B3E98CFE90B4D78276D46A1C570E3C074AF22793F0416E7B53036907BB3A653B5000E26A88168B2B334114FAB499165F3637206B1A9ABC568CA8C5B18307174B7BEA597C2E6B168227454BFA63D6CCC08BB5BF39382C4C1038E73EE7EAED100342467C27C7DCDC0F79410371CF7AE82736F94E4BFFA53D127D3DDEA12CEED134CB00B6B829C578");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "0ED2EC1EBF378A431065B30C64260D51147415A3FCA60929B788FF88C2C3B95839F6ACC285BCD05A9070BFB38289A58CE073C10E4F477D486FD83514BF5419036317F7854F83B1110C0599037831611E1B9053F7743BF5644E79A807E0890A75E939CA0F98D5C74B79B3272C909A3344F3A3561208EC6FBBD0C065EA78F9A3A4C00D4281792C641E");

    // Example for BIG_SHAKE_512
    use cryptocol::hash::BIG_SHAKE_512;
    let mut hash = BIG_SHAKE_512::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = BIG_SHAKE_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "550E5C0B777E8F695D44BAC45752D642B790F4878A17FC07D8CC9EF38FFCDCBD2F5275E4534F866725D67CBE4CC0F81AC5377244FA6C56AF9EB2BC33469A99FE75C2E3C964A9D8484B6EE2FF1866F1871C259029353ABFEFE8AD0DF69C896073257EC9B8CE2DF6FFEE0598B9B27C14154A92DBE4855319061E3A7122D060CDE0923DBD6133769CEFF0660F48B1E136B95F62915EECD00F51521CB1E10B039AA2A06B77970829B4C3CEF73DDDFDFD064608D8871A9A9CE10D1050126B6933E8540E8459D52DADE3EB98ED13A9B60EB6A83F154E45092C12BEA6E2B0A486D17DBEB9618416CCC896B303144AF437044494AE43E7833D2404012CF650CBFCA45F595558E4BBC59A81E047B2750CFADA737D");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "38E8C2B7DEDF2A391ECE0B1D7C6180256257B29AFD7BC85B3ED1A66A6CAA1458BDDC595275AF7E2EA74B3043BACDCBFA8FB3626F6748291FBE092C0E690460DC674529C34111B1E64B97E9B9F6C9B0A67A3AD3B70C702E759C0CF3F8818C119A05CC24C7821AA862364BC8D99F473C262B647EE41833168B8E8A6E3D95E3AD35DB8375051D63AB7458BE668412E98A7C9D4C7C7CC9E814D39A662706CD1FCF1F83483767C44896A5EA2EDD7ED425757FE0F099FD3A4545C7DCAD047C7E88879CF8B3FAED3FC702792467955C0B88F6D958E6C9C282ABF35AD9483BC91BFEEFEE58377FF95B65AC321A0C44539415CDBBE197061BF3B3C635669D98BC22680B111D26F45D311BDF70C06279D8E9D19005");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "17664323DB7755F3F1C897BE4E899C742E8D00948BE95A9D48CA30B12305D58A88291279C379FA84D9E34992326C106805FE9F334C30BC2C6C4AD2BCF8B2D66D4696DE27B977D57D35643788B47DAB9AEB3EE8B4034B76CBC8D300C8F03FEDFCC8BDDB5A1B7D16087BF203206C6B19F206301E494A47593818FD76029D6D94DD45CAE145AC0B19FAC27C94CF5E3ECF2FFBA79DEDF529DD6D10E29CBF2C55E58D1100451BF800703562101CC0EB69864B260A5FEE564DF87937A63C11670AC51B760B6AC749DAD7C3B29EC66227F0D0CDC773438DD7622E884519A25CCA8AF6F1B9A374F86E627AECFB6E8E1CDFB14656E14F53BA124C60DB9CB4BE07EBC5C8863581AF144D0BB07721FD0319429B1D85");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "504E2F94EF87D7DF7132E083B3AB67ED92A91FF50DE823ADF39B09184C257EF33BF166954F32601AA54196BC98BA63C2C73D1A75EB6CE096C66364F15B7AF6D11574AD9B5E6D35706FEED1B6869943CF8A6D4ED7FA4EF82E729E450111F8EDBFC6D299DD690307B7A3D1EB6099F41511A5951A9149EDF220DEA5EE462A03E7AE2635D48B6007BC75A6193D8C073190F1CB87268AA6786D65230E6AE8B47FEED25A954E6628D4DDB5F714529A852ABB0F099ED9199985883CDDB99862E5BC164C16676547457F4131608D87A9E217CFD31E65F64A10AA77EAAB244CED274381529503C6B0747C0A37CCBBAFAD087E877151C0EA943F66C6048A8A4582299F675D47060A780D9A0119410D922FE6B88F11");

    // Example for SMALL_SHAKE_128
    use cryptocol::hash::SMALL_SHAKE_128;
    let mut hash = SMALL_SHAKE_128::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = SMALL_SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "865C6B34C80FAB343F72D100C39F9CCA9661FD9FF32118F503CC59AF79151566420226C3FE01DC7D1445427EAC97D3EF9C65252F747AC74847696134BE9D3F353D782DD6");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "0AE283837C8FBA419D898E57D7F5EFCFDF5632934F7B12DE179A6935370DD9B7D93965A942984BE8CC76415E039D7EA1FA0C9A6E51D59AE4639797687AC2A03D34AE8740");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "D117A7204045520F27AF8C0305E6D8585178BFCA9D97C8574150D1B99658AD3DDC716BE6E60C05F9E1AF355E18848D37EFD8DEC7CC2045A194480A7622F41A8E95C7E580");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "1B24D5D759B1F144D48834450D54A1EDC2B4A378292CD68E76B14E140AB4399555EBE4135C542F74F0A360FCC2F58FB9DF9B8680300527DF95F7ED49BF04B95F766B5092");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let mut hash = SMALLER_SHAKE_128::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = SMALLER_SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "AA6EAEF276BCBC7370DD18024EBFAE7BF07A");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "C8C273F8DE03BA37BD139F3014C35A696CB0");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "058E96B72AD3CAF5E29FC5C04F914CE8BB85");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "7AE343EE3036B1F14741845F30B97B06AE4B");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let txt = "This is an example of the method squeeze().";
    hash.absorb_str(txt);
    let mut hash_value = hash.squeeze();
    let mut hs = TINY_SHA3_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3EF98C0E7A7E7C1C73");

    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("First random number =\t{}", hs);
    assert_eq!(hs, "AEF4FCA8D9E603377D");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Second random number =\t{}", hs);
    assert_eq!(hs, "D66789C3DB84618921");
    hash_value = hash.squeeze();
    hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Third random number =\t{}", hs);
    assert_eq!(hs, "DB28732A04F8942EF0");
    println!("======================");
}

fn sha3_absorb()
{
    println!("sha3_absorb");

    // Example for SHA3_384
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SHA3_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "50739E69E4848F1AAB0192BD08899F5FDCF194EA91CFB51FCFA53BBF01749E81515503A363D961300434B7DA1BC3E2BA2A56E39A33D7D797AE3694D42027F52F594FEB9EA93684CAE961EC23B7BD8E586D7A79A9A80BE24175F5C169CBC4B0FC0C9E73C8D08EF06D");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "CCBED74DB9FEE068F65011D013357D86123A15A0F0156A199E3FD82539766CCB2FB3F3703A5FEE02CB19BDFDDD10BEA95A0F1BBBCD02E44B5073C18E6C38159A2CBA32266FF3C92A8030CCA2AA648094DFCE67B502A1E2342D1E155FE8E56F8FAE935E8E69A0DFB8CBDA908E7AA590E5A9F4BCF0DCF93D5714CC02FEAF2F5C9DC1C27F7DFD6C39790234A9AAB03DCBF54E5FEB308A2D458E5341A84AF282934948F3A62D7FEE27C1");

    // Example for cSHAKE_128
    // This example is not fit to the standard.
    // The method absorb_customized() is better than this method to use for the type cSHAKE_128.
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "CCBED74DB9FEE068F65011D013357D86123A15A0F0156A199E3FD82539766CCB2FB3F3703A5FEE02CB19BDFDDD10BEA95A0F1BBBCD02E44B5073C18E6C38159A2CBA32266FF3C92A8030CCA2AA648094DFCE67B502A1E2342D1E155FE8E56F8FAE935E8E69A0DFB8CBDA908E7AA590E5A9F4BCF0DCF93D5714CC02FEAF2F5C9DC1C27F7DFD6C39790234A9AAB03DCBF54E5FEB308A2D458E5341A84AF282934948F3A62D7FEE27C1");

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = KECCAK_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F3205511D8887C99656F452F9622FE890029E7C4CC873B1DAF5325B46DE0AA0933E132C3C561B5C25910B70C6E356CFD9E2223797733A870C2ABF9EF63D0EF486AD2FECC8E31E27A");

    // Example for BIG_SHAKE_384
    use cryptocol::hash::BIG_SHAKE_384;
    let mut hash = BIG_SHAKE_384::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = BIG_SHAKE_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "FAE3EEA011A7A4AC631CA66633B8380F4601A7490816DB7EB1CECF6EDFA9E4D8AAE8F798C93FBA4DE8CC20CC6B3C78EAEFC0327D46B553F5CF1AF5C229569112105FDD82E57B604C8D2A27B12BECE1CA5D8CDE11D4CBD2D91C61216F88AFE1C661D6D92BA5FB8BDC79CD363DCE017F8D4B3AC0A8287ECC45F1B6E19825589D3ADFC723E2A3D6BF76F82BDD266FB0D2A329CF9564A066C65CAF2D4DFEFADE8304CBB8AFDC600091A6931979A85387D029BD7D5D41519BA8B4BD6F2C795C223B2AB5C912BD60D809583E1BD312C728DF0D8468AF8B2093A87914AC25735755E1EB52BF6F373C18C28CC899D42E37CCC6C3EB98D964B808492C41688CE9DD9ED62DAB2C7A63B2222D954042B088EB43E92D469F4DD8C74677CDCA3D1CDDE8618AF6DE7D16C34C6CBA406002CDEB248364B9");

    // Example for SMALL_SHAKE_256
    use cryptocol::hash::SMALL_SHAKE_256;
    let mut hash = SMALL_SHAKE_256::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SMALL_SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "7F4E0516B8EFF60CEAE1202D51396B90367D1BFDC291F0FE0D25D33496EDC1B47955C7AB");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SMALLER_SHA3_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "60F3AA0ABC339756344CD628F1DA3C8A7D6B");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let txt = "This is an example of the method absorb().";
    hash.absorb(txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = TINY_SHA3_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A5AD0F3795AC9E3427");
    println!("======================");
}

fn sha3_absorb_str()
{
    println!("sha3_absorb_str");

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = SHA3_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "625AFE67C854BAB869A7C18798402BDB136E2A320DF953F0AF0F6AD7CA10C3A4020BB0951AD8CAFD36249266B23D6AC41F2D8A65DCEA8FF0B643A17E16F01CFCE51A73747941EECD");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3F2C75AEAAE0DF11793C86A8668C458555171C2ADCA2A6E8446094698476B976D293E1A9D27E15F2F292C202B8933A22CDAAFCD5F9E036A1DB88CDD4E42F6E0607D6B9EEEAE15BDE07323E3954D1D2F77640E04571D65F64D2A2A2F470EC192DF2D83838373FC446345129078111EB67217FD12423CDEDC8EC1A00B873FE07127FC0CD2E27E6C785D9206FA6A181D5B82258FF6E552238B2264692EFEE60E2A80DCB6100DBD64B6B");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3F2C75AEAAE0DF11793C86A8668C458555171C2ADCA2A6E8446094698476B976D293E1A9D27E15F2F292C202B8933A22CDAAFCD5F9E036A1DB88CDD4E42F6E0607D6B9EEEAE15BDE07323E3954D1D2F77640E04571D65F64D2A2A2F470EC192DF2D83838373FC446345129078111EB67217FD12423CDEDC8EC1A00B873FE07127FC0CD2E27E6C785D9206FA6A181D5B82258FF6E552238B2264692EFEE60E2A80DCB6100DBD64B6B");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = KECCAK_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "CD18849F0EC2D7CDBAC6E9C019196A091EB945891F50EF1472D3094B7DC02C67C2B286674345D71768796C79A41EEABBC9351EAC6EBA6733D6153960769357A98CFFCE16F2F040BC8278C27C3FB96D276B297A97E1545D93F1BBD31432FC809715A20ACEDBBC08F49AAAE90719F6C1E7C14DF4CF0EC8550A7C358524C408CA7A4E2A04258312A886F0C319416003B7E9");

    // Example for BIG_SHAKE_384
    use cryptocol::hash::BIG_SHAKE_384;
    let mut hash = BIG_SHAKE_384::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = BIG_SHAKE_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "5481F64E69E5B6369A8F02E1A0D91008DDF2A64DBE339D31AF4586D55D023A836AAA836594C814E252732C9319B1C3728EB2D713F3462833C9FD41DEE02A93BFAA91C2D033E3E45D218D11AE93C00E45EDFB815F80F3C0872DAD7774FFEDE9A629C065DCCAC50D7F558F65B883523238EC605322D4A4147398AD4276D39CC627B21557E2840304AF83F31918459E99472CB0E7BCED3C433751015FC3382F2E7C560124EC17BC947BC079A72BFB8ECAE1D9183495FB42B5E0729F3CC98DB61872DFC1E9BD0362DB1EC284B71F45DFDFEE49C3FC3DAEB3DBB2B76947B4336883E9B0A305BD95E72894A4816AB8328F5F5103A328D1444B53BDD52AEDA71DB5F8B236026B271596755E0297D1C0703127B5C95775712A8C1D2DE35EE8FAD7D6EDA12F9FBE412135497001C706C86361747D");

    // Example for SMALL_SHA3_256
    use cryptocol::hash::SMALL_SHA3_256;
    let mut hash = SMALL_SHA3_256::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = SMALL_SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "93DA2AFF408F35982F065FD8C4D947D7BBBE550615903B0B579135C77D3EAB1908582C42");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = SMALLER_SHA3_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "FED3B5606817A11065BBB80DC5842839901A");

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let txt = "This is an example of the method absorb_str().";
    hash.absorb_str(txt);
    let hash_value = hash.squeeze();
    let hs = TINY_SHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "C817912623378B1587");
    println!("======================");
}

fn sha3_absorb_string()
{
    println!("sha3_absorb_string");

    // Example for SHA3_224
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = SHA3_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "D3B139242B2B6EF8B399CF0E6A4F6D993AC83EDE2CEA3894D4EDE8AAB69ADA5DB232289E62D752349FC3F621B2DA118400142D5DDBA6897633F712A1B320FC4F1F7F3D5BE186FE75F8E3602D344FDA1A5AAB343E8FC68B918CC3C2DC01DA3DDF7AE0CBF0C855D6463EBDCAD81F9CABED5580EF6C652E786DED50EBC72DD827E4129A368A823E8B6F7FBCFA6F9FBB72F9");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "1110C31605133B975503D885F1A8EBB37AB6E48251279A412C0A0AB066CA337CA3B0BE6AD33F1BE18E40094FDE8270D61578CD069273330BA4E84933AE6E328B63C7C0670853323718580D0AD3814046E8470A6338D32F8D2E5E11C278B0CCDE1D12DA82297E48F9B25608DC44AF5EEFF88DE200A150D1A40347BB9287994AFC034F49D5E595A458");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "1110C31605133B975503D885F1A8EBB37AB6E48251279A412C0A0AB066CA337CA3B0BE6AD33F1BE18E40094FDE8270D61578CD069273330BA4E84933AE6E328B63C7C0670853323718580D0AD3814046E8470A6338D32F8D2E5E11C278B0CCDE1D12DA82297E48F9B25608DC44AF5EEFF88DE200A150D1A40347BB9287994AFC034F49D5E595A458");

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let mut hash = KECCAK_384::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = KECCAK_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "324EF5EDFAB21E0896507C6BAB6CAFB60D0522C86AEBEF91FA7AAD3FB2537559F1A2977888525739463CA2A58E55604CD229A1613962B99CF9AA5D30CE767BF52A3355943E0BD9F88C6397F8FB2482C7BFA691E6960B89E9B4EEF83F7EAA68B717F14C498683EBB6");

    // Example for BIG_KECCAK_384
    use cryptocol::hash::BIG_KECCAK_384;
    let mut hash = BIG_KECCAK_384::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = BIG_KECCAK_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2652C0FA38FFAAC2828C51D616070B9731BF0099245CC416728CAAC8A2076AEF4D84F96E8C9B132A33443784F64B6BD81DABAE84BA758841AAD0113A876AB1D1738AA523A8B050517CEA512B8C1689E3F5D3552F0A66E574EF8B68B54ACBE9F9FF1036294B895AEB9690441255B2C88DCBA170D9DBAA46B31D470F2ED2F458153A3E40AF4705B34423C0BFAE5B06A51851B16F0D3AFF77B00FF15C3A82AC3D52EE80E91D9B126F1EFC5FF2868971E267457F2CDE57022A64C66E1C60A6CFBB5EDDD46D7C68C7B5605528635EB41C25F889D81690E72C7160A077447F0B112E2203A1FD27D43B363D8484BD094B9D777147BC68AE7F3023F0C5CA53C7C1928A51F614D8ED8D21C2097A8BDEE0FD9BE5BCEA22D852AA31D2BE2EA8FFF6AEE48B5665803C8A6CE9EF4B34DB0CC650336FE1");

    // Example for SMALL_KECCAK_224
    use cryptocol::hash::SMALL_KECCAK_224;
    let mut hash = SMALL_KECCAK_224::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = SMALL_KECCAK_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "605EA28CFFFCC112FA9642B596B537B0EABB6810747FF58C548FB0149FC4057137EE275D9D4D34");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = SMALLER_SHA3_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "E5ACFBA7DFF25B176152CC4B8BB635D14D89");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "This is an example of the method absorb_string().".to_string();
    hash.absorb_string(&txt);
    let hash_value = hash.squeeze();
    let hs = TINY_KECCAK_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "C4580F144848AA72AB");
    println!("======================");
}

fn sha3_absorb_array()
{
    println!("sha3_absorb_array");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B149F41975DC994F8BD506DA5460FE855267CBD0C02D4DB595C78697D2D80FCE659B1F7ED187BEC70EDC00666A545DDBC8836B11D9410F544FD3A0A0288DAEBB92F86C654AC20336A9A7343180DBB8FC342E024DA4627C80697D78BD365AEEBC58353F684ACEAE89ED4");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8DB7083B7D83471BF3390D83F714B9E5DF3D5604786BDFAC1974D77F4DEA4DF0EA0DA02C3B8A25990B5F623225C1B5B46F7415F5158397251E1D00E77235E920D2CC78CF1986CA3391");

    // Example for cSHAKE_256
    // This example is not fit to the standard.
    // The method digest_array_customized() is better than this method to use for the type cSHAKE_256.
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8DB7083B7D83471BF3390D83F714B9E5DF3D5604786BDFAC1974D77F4DEA4DF0EA0DA02C3B8A25990B5F623225C1B5B46F7415F5158397251E1D00E77235E920D2CC78CF1986CA3391");

    // Example for KECCAK_256
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = KECCAK_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "DBC011A9579CE2645298902B926DD2A22DB6AEDA5B47DA3EA0C9FE167C314E931567B74881A9A9D5615937AED56685DB9838E71AEC3F2238595C5D5D9BDC3BF7A4949FAF2FBA8C460318F396A50A161F2F1EC44F1281839B12E9B6852F72E75FF11CA794360059871D7C9054F136FFBA173275078E2350AE0198F0B2567E8FB325CB4DA809ACA26B");

    // Example for BIG_cSHAKE_224
    // This example is not fit to the standard.
    // The method absorb_array_customized() is better than this method to use for the type BIG_cSHAKE_224.
    use cryptocol::hash::BIG_cSHAKE_224;
    let mut hash = BIG_cSHAKE_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "C62AD0B10644D3F6C1BACB15F7E8DDEEAF42DE964784907DC47A6A227620AE24B7239AFE4DE4544CFEB5B875E454DAC63DCED38C725B27E1C06122ABFB06BAAC153EF379A769ADACB4C9A6BDF1479A541DBE463090ED89133EAE3205F34F8C9009B9CBD1BF654F1395B76214923C0DDD19440A00B4181B5C011AF9E4B4816DEF89FBFBD2898D21249C3C1B4A08C0CF6EF6C1D4649EC28DD6967D8BDDC3CE21EA761A20F31F1FE8D79B055308143A150530ECE044E48FBBD9B100280B8C6726B5314A0EF84B9C3BB01594839F1FF2B4FB7E02E0A872A6EF4A89F5BC7AEB9EEA412D1A75718F6F0DE1FC0A671D9B357F7EA0A81D05D8242C55DB17835BC697B7090F4A8F8766DD4A3502F18DB2FB1D6770695DE2919C1D7AEA8B0DE3A137E8E33DB558E3FC1C32B2D106ED06233797C892A083727D8BF75F05ED35C8D5CE83076F7CA64AF8CF0C6F10209BDA9B36B1523D66C7792466A6A7D7");

    // Example for SMALL_cSHAKE_224
    // This example is not fit to the standard.
    // The method absorb_array_customized() is better than this method to use for the type SMALL_cSHAKE_224.
    use cryptocol::hash::SMALL_cSHAKE_224;
    let mut hash = SMALL_cSHAKE_224::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "96F9E1E685079869C22ABE1148BD9930D82F20F699BD34E23A74D2B08242D0DC879B5A8E169C60");

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let mut hash = SMALLER_SHA3_128::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = SMALLER_SHA3_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "55B0DB1B383233F05518C4FFE43DF8B1C7DB");

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let mut hash = TINY_SHAKE_64::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array(&data);
    let hash_value = hash.squeeze();
    let hs = TINY_SHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "30A8CD9FEB02319FF2");
    println!("======================");
}

fn sha3_absorb_vec()
{
    println!("sha3_absorb_vec");

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = SHA3_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "8714760A0421335BF23FE3C6F436686983A04B055729161041E717E1E964AD2B4E0DB3EEBBE35BC1BAE2999EF1E0DF48F4A60FBAFA1F5B96E3A20A24D44A02989646C598D1857681");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "D1FA3A4B2052C609200E765D22BD55DEE83AC254E8CC8376278C096AF9C930827AC527728CA7A1B081EF4E9ECEC88F07F3D57F1F25385710F6921A3FC841BCD61E74D6A0DBEBC3910AE77A58616344DFBD8FFF3794CB318009E55A0B698CA023BF4835AD56A2D5B7BDBA88284757A8552A975E821EDE520A31DEEB7C579690CFAB0AC75E7844D6124C95584B66E10E8B0DB16E5368AC3A29BB74898DA55CAD2D60ABB1AE70273C9C");

    // Example for cSHAKE_128
    // This example is not fit to the standard.
    // The method digest_array_customized() is better than this method to use for the type cSHAKE_128.
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "D1FA3A4B2052C609200E765D22BD55DEE83AC254E8CC8376278C096AF9C930827AC527728CA7A1B081EF4E9ECEC88F07F3D57F1F25385710F6921A3FC841BCD61E74D6A0DBEBC3910AE77A58616344DFBD8FFF3794CB318009E55A0B698CA023BF4835AD56A2D5B7BDBA88284757A8552A975E821EDE520A31DEEB7C579690CFAB0AC75E7844D6124C95584B66E10E8B0DB16E5368AC3A29BB74898DA55CAD2D60ABB1AE70273C9C");

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = KECCAK_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "84DC4C9F2C0B38A05C973A66B63EA7AEE8BBE1334E4C756AC6660717D133DEB151EA217F7D3B69326C42DA675D7B2DC60CF1CD8F7EBEEFD2302BDC71F068F1098E55CF3EEDCDFFB3301E039A983DFC5D2567DD4D5594E6814BE12465851AB189F1F29567D8378D395C259BC35216B99AFA47814F71490F1CE9AE829810CAA75E03E388D000203FDEF711E0E973C64364");

    // Example for BIG_SHA3_256
    use cryptocol::hash::BIG_SHA3_256;
    let mut hash = BIG_SHA3_256::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = BIG_SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "ED0D42916B3C3AAD0D3773563EFA3E0A2F7BAE185D2AF692B243D21E5B5201098027516E2C6AED10E612799D36FE0EE32C39402D447E60233C8CBBB90CEB981F6577A71B571EF81C387C64E1F9DF59B8EDD1E1066EB8B3B240A318F2EBE27AC02FA98A4F970AB55A4BA09997FB3374C5F965B971BC6A24F18E24AAFEBDF12ECA0E74378D2C1B4AF1E8F218FEA5A72CFB9817F3CC9B445BB88F9017300CC208F932B17646CFB33D169536C01E5262822125462CFC03A80056FDBA99A487B415A12FE8B038FDDB0AF0877DCAC77071ECED431CAB2465BAF8B0A00CB81D5A9B570E934D777EBABDE0502AD857307D8751D3D1728EDCF7795616609D451DE6A2392E146894E41B60E606A22B5609DA6080096F719D7C3430E93AE1EFF2D89B53CD572EA7F3C14D0EA18791C9F0E49AD6CAAC0F669983E0E171AE40E7D9A701791C5BC9328FE447033E28FB6CD10C0CB72523");

    // Example for SMALL_SHA3_224
    use cryptocol::hash::SMALL_SHA3_224;
    let mut hash = SMALL_SHA3_224::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = SMALL_SHA3_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "3F807B4BB0ED65212CB44339A310797A3B855C5CE2693CB9820EE0858473641749D3D93D7EEB1A");

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let mut hash = SMALLER_SHAKE_128::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = SMALLER_SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "8943A7B10EF95A6298F3A7EC9DC4E66E85EB");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec(&data);
    let hash_value = hash.squeeze();
    let hs = TINY_SHA3_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "1ECF158DCF9006C6AC");
    println!("======================");
}

fn sha3_absorb_customized()
{
    println!("sha3_absorb_customized");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "C72A5C005749C1EEFC7776965FE81E519E769A026D5FA9321928D7881B77A46A1C44A949E1636846D29771076EE02E40C1A684F0575D3F618DA25C6025153AE2F4E416844E1676F885116B879B9E53356A5C18D7E5216D32BEAF07EBD652477A2AA03A6A31408C5153730084218FFFF9B5CD5F1AC1F1949A4732231CA7364F223516B3871384048695ECC60079CB18B25963643CA8468EA3BC1DD020E66B75C6E13AF900C61DD9E4");

    // Example for SHA3_384
    // This example is not fit to the standard.
    // The method absorb() is better than this method to use for the type SHA3_384.
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SHA3_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "1812660A433E0FCDA63903AA17071413973124836F79A3D6BB9E5506F80D973076BBAA6B95DB38303F535E598E41BA4D04DABE2E7A911737EE7BBC99121C0DFBF0BD0805793D7E7BA5E1F91C353EE83B1A26C63F410757DD00EFD7DCBA23822F039D582ECB3546A1");

    // Example for SHAKE_128
    // This example is not fit to the standard.
    // The method absorb() is better than this method to use for the type SHA3_384.
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "63227E7917D6435340ABD35666A38BAC8C8E628631FF437433670E38A19FAB2E68EDC4FD0ED36036B81508E4F4BBE2FBEC26C365CBC64B5640E2A024452782E3226190EB16B3AA20B97D9FC466D61E28EABDBB43DE92B4A7140DFD8BDCB563EE109DF89165F978D021D3E15E0CEA2005F5A19BA5B0650CF359566E3AF8D6499FC8879E36EFE16AA36C950C4A687BDCFFA6C518320D5CA244FC03CA19E73754D5791CAC43FB72504A");

    // Example for KECCAK_512
    // This example is not fit to the standard.
    // The method absorb() is better than this method to use for the type SHA3_384.
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = KECCAK_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A1AD537C85CE8D196D812DC393D3AECABF82D6BA976EA4EE7420E253E7A6EFC73B28A7CAC3ECCBA187BAA93947BC41ABA2E700F1A9CCB798E3BE1D691E5AEDFC572EEACB221FC188");

    // Example for BIG_cSHAKE_128
    use cryptocol::hash::BIG_cSHAKE_128;
    let mut hash = BIG_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "B84E4ABA41BC82EFA2416516CADE91E37C804332B4726914E9240806A155D4C66421A9DEE3E3B13D5D98969CC34751DB609CC9DA6AF9E3CCD6262EC4492CD3B527A94467966EF7B71B313BADB8A14F43C96942A94DC0EE2AA85963CC59EBE0FE01BE3CFADE6A4E9A4CF14DD12790FAAA8620C1F39B20F0FF0624FA3FFE967E51F6EABEE6FDF771C3B364D0A6B89971E2293367F3100AC8883E1B6A824CBA2B7D6363C84E220DC0182714E24AB0E85311C7E0593828D67E4A0ADCD1112B9EE50B6B189259C876CEA481998A4038897E80B0F42D19532BD649FD8AB381EA50FC8F3638FCBFBB3EC43BA8913A9F628F038D78D0D17E3F807EB47F3AC430608008150840BF02271439497DC16C9FBF3086D439F2A497258F0CC41E242A6F8B54502E6FDA2957B5E23BCCF6DF8CBD7E867F4B5AEC121C1168EF0B84D4D9667A160C2251107969D28833D80D1859554A03DE9FA9ED62980BF7C0E789766C0691ED6B1DE149190D22735FAFBD153F0BFFEDD084");

    // Example for SMALL_cSHAKE_128
    use cryptocol::hash::SMALL_cSHAKE_128;
    let mut hash = SMALL_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "AF9E668540BC874F8C462DC9CC6CB9468C63D8C85669DB3F1062DC40BAB7983230E58BF82FA02B8C3809DECAA15782572DDB817D0E20F49C2B602294FB2880DE446CEF56");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = SMALLER_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A4464D863E6E285F8226A775C1EE6E50CF0D");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let txt = "This is an example of the method absorb_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    let hash_value = hash.squeeze();
    let hs = TINY_cSHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A0598F7B69F3E7E33B");
    println!("======================");
}

fn sha3_absorb_str_customized()
{
    println!("sha3_absorb_str_customized");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "8C4C069400F3D84F48DCF19F3C38136BA5C7ACADE522359FB44085A39C3909A41E2D953B9454BC3BF5794DEE14B65F0B801FC72EE593850F63552441E08E328D7EDFDD0F7C84A48C699A1C01C331002D938422DAFDEC9BA210E0F0CA8828C0EF89038FE0995BEBFC21D96AB4EE1599E847427B465F3911BA6631A4FF85585663F255ECA5459E090B4FE3E9B199086BF33E4F1221A8128BBE9487DE178EE497B088B69F80F0C024AE");

    // Example for SHA3_384
    // This example is not fit to the standard.
    // The method absorb_str() is better than this method to use for the type SHA3_384.
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = SHA3_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "2F037BE246A580D7B7515D614D37541E3BF07D57D3A3E000671254211E8C05A781BCCCA322D851E8D6B918B5F5337F7F91E95C82D266D3D9653EF84661F7ECA943E89382170FBAA9FA79F2BCBC850F89C584C433B1785D330F11A40ED4AB3AA0C5A7520355229CEB");

    // Example for SHAKE_128
    // This example is not fit to the standard.
    // The method absorb_str() is better than this method to use for the type SHAKE_128.
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A840EAC4B16B658E5DC19DBCFB83A808928575629043021CDA5EA8F859C53ADB5612D99347248F22FBD93EC0AD816CE8D5D2AC2D7D6235E2C04850BC490270DFA516BED9274B73F1A063DF4ED231EB110A2F68C26391A0302CFC91943D66E14AAC5D7B952826448761B6A026453DF01D04CA34AB66AFCA59E8ED4F73B9E2CA646984DF74DEDC7BCD9F6F0CAFD95CC50DB73E592A9F906F446ECC18A48D59BECD4F4B7078197A6CC0");

    // Example for KECCAK_512
    // This example is not fit to the standard.
    // The method absorb_str() is better than this method to use for the type KECCAK_512.
    use cryptocol::hash::KECCAK_512;
    let mut hash = KECCAK_512::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = KECCAK_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "609C1CDEF68C23C77FC8A3B95E15A5CA6A44D91CBA8EE6A98D9793C152E478CEC9187271ED8D5F0673D9952AF7857CF3AB6F63AF86BA01C15CB3DD8930E636D8D44A75A849253EEC");

    // Example for BIG_cSHAKE_128
    use cryptocol::hash::BIG_cSHAKE_128;
    let mut hash = BIG_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "55710F83BFC76A4E084AEFF200A9E45E058E666F71F69002FDF93320E346E7C6DD109DC08F5EF739D41FF25B26D9A70571A4F5A233EB48DABAEDC10369D98968D39E4C939F7C7B3CE80E47D9A0E3F267FC7D77460F60688D7A41B5DD993898EDB4B8E5E929002A4425813A86387AE10D363EB4EEB0A548D40233EC1EEDEF672EC1C28DF7CA073BFAA1935BBE5DEE213A3AA6E48B276F08F8E31B3654CC6D73F2094136C0BB2431A6F0B6D2BB133A7CE6B9EC99FDE0C93E341E40EE4A5B62F4B6BE8CB6E2F0B47D67D68B17CAA58C58D9F565FC48B5050241AE2C146696F8E5F8FA26D09FB0BA20EA7C0952F5FD231ADF49770A2D9D9588E6C1A456DC0CBAC0E2EEDE7EE12302DC24DFCA676F8ED7CF62C3508145DD6B46971FD6622329EDF114EE8A3BD5BD430951154F06B88E72F92AF4BC8A88E1F39B62F6A90528860CE70141C83BC7B48F8FC28DC216191F26CB9339A04525D0E9B526437F801F8E5B2C007227432D120E7765F638E7AF2713AD4B");

    // Example for SMALL_cSHAKE_128
    use cryptocol::hash::SMALL_cSHAKE_128;
    let mut hash = SMALL_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "B67B8EECEB57ABFE5586FB0FCDC7F60D2B049E6E3E580A30ED0F9733EB0325C84B76975A7FF06838E8355ECD0EFB3AF5A0DCB9D5835198D1B221A86E75DB939005B823AE");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = SMALLER_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "71E97196E17B1D316C2F80DD895543F43DBF");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let txt = "This is an example of the method absorb_str_customized().";
    let user_defined = "On my own purpose";
    hash.absorb_str_customized("", user_defined, txt);
    let hash_value = hash.squeeze();
    let hs = TINY_cSHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3CEC68AB82D86223A4");
    println!("======================");
}

fn sha3_absorb_string_customized()
{
    println!("sha3_absorb_string_customized");
    
    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "E9D77C6EEF37DEB01E8B8A8F250A4F5791A3B2BBF17D9C5A729DF38ADBF80742F14C44E7CE17CAFEF6A847C5403D0D420BD47E38E24E6B8D3946B825D96165BD5D81373195218031E915DCC744F7CA244F600D1D7E2B6195890BD2F0B5CE05BB2D877958675D0E0395150CF7E94A0036200A620049595530D123C077CAC1749E4ACE0098F92DA6BB");

    // Example for SHA3_256
    // This example is not fit to the standard.
    // The method absorb_string() is better than this method to use for the type SHA3_256.
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = SHA3_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "A7E1F85E875B8B392E77DBF3C3F3B7256CF087A48808ECCF3D544F1FD3AFD2321CC7DF57804EBC1C4A91DE2A342C2479B609BDBA123874B64C06829C8A3B6B5BAF213DBCF5CDC7F4F33EDA59CCA0C73BDD0DDAB55896B5BDBDC80FE69F646301D3F2686729CA8EA2EEDBDBD7F86EDF4DD0D63F7F4C208183E12EBF28C681A3EA769F1DB7C9621298");

    // Example for SHAKE_256
    // This example is not fit to the standard.
    // The method absorb_string() is better than this method to use for the type SHAKE_256.
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "3EF7AEF691B32C09D05FAB0DFE54879CE9B71410B9BD90DBC470C1D8C40929E394350C298F8ED8DE0AC5CB94900852555A2CAAF0C7B1CCB6C6578042C5BE74B10309635BBE3477FD1A457571BA7F7480548A9DAE0C0EF357ABCCEB7B8448DF8EEC8B8F2016AAB0ACE778E4569549D38FBB35F6C3C66F3AC83E0B07AC1367C7C79729316C22AD5F9A");

    // Example for KECCAK_256
    // This example is not fit to the standard.
    // The method absorb_string() is better than this method to use for the type KECCAK_256.
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = KECCAK_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "5AD55131FC9187E26FCE1BC64FA43596543FA7DE4B771E589DA84F648C6BCC4E57E4FD163860CC4136D737B60809F0E7DC2D3697F640F4168E5CF47BC7F1D40A6E2BA44037B04BB048E4DAE67028EBC313629E6DA76FF870D4702775FFEEEB9C78C80A90D548E7FF15FC7DF4A1C04587D32D8433AB4526274E8CE052F55A50AB1404883C01121B65");

    // Example for BIG_cSHAKE_224
    use cryptocol::hash::BIG_cSHAKE_224;
    let mut hash = BIG_cSHAKE_224::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "316520AA20F7CA25A84E0A7691213645C2AE15CE7926CB882FA6B8857232DE125E2C5208824AB668E4827BFFAF2F788BC5A72AD6FB87E041036DE1473E9841861F89293E0C141966404D340DC1139F06584FF48D20A81F612569A812AECB389041582B9287BCA48EA1CB52D438EA96C674E61638FBAF0BA47BFD10EC3E84CA0F81F12147AAC4679899A424DAC32E82292C5ADC908EE9460F083183235FBF230F5464154894654736E2387C3F9EB7ABB60293F7123490A268E39BE059A9E8E3DE048F21BBAE084A9E9E8CF7AEFD2DCDFB698F24952E8C12BE9B97E47D27148C1A81D61CAE6FBF923C09A9089C5267C557316103735DDBDD662E7D2367BE031C1331B98AA9CCC1D233DA0BA62492A38D8F45160F801E10A27BC6930407CE3E8D6B4E22551A53162114BE93679D66E22359300127EECA277B1A6FAC7F0C9B02C4DEA2877137169DC9377268915D6FDF35EB29142579F36B03A3");

    // Example for SMALL_cSHAKE_224
    use cryptocol::hash::SMALL_cSHAKE_224;
    let mut hash = SMALL_cSHAKE_224::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "1D4323F415C446379E43502C58F0673A91267DF06A4E7C40599DE643552B4A13F5683468E9AC6B");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = SMALLER_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "FBBFD0BA546A9F064FAE54FB402AC214FA1F");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let txt = "This is an example of the method absorb_string_customized().".to_string();
    let user_defined = "On my own purpose".to_string();
    hash.absorb_string_customized(&"".to_string(), &user_defined, &txt);
    let hash_value = hash.squeeze();
    let hs = TINY_cSHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "C50FB96542CCB2988F");
    println!("======================");
}

fn sha3_absorb_array_customized()
{
    println!("sha3_absorb_array_customized");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "FE5EBE3F5007EDCE865F7DC425531F0C493E18168835722CCC0F11CE66F1423061C202BE0B1D0528DE0D763A3097A090B62115392769305D1FF32588A78CCEE990286ABF615452E4DBFF3915D64673E725C5BADF965FC06A80F90C865C65EAEA7A107228B80AC49C9C89F98F1AF4C3563BE243207D6970219264ACB9420E97BF802E862D32D18F31");

    // Example for SHA3_224
    // This example is not fit to the standard.
    // The method absorb_array() is better than this method to use for the type SHA3_224.
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SHA3_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "EA16847C324CE60EA9EB98F87743D9EAD2CD2D5DA80C667B348864103E5DEEBA1522994495F546DF6D938B3BF57371408F1E917645B800C03DBB1AEBBE9CFF1AB15178905CA481779C2F42D35CE2FE73F6CF85EFD12F52A27A3913140E11074BC4A96AC09D54A8769A6445F7AC7D3FE5A35CC65877FE2EDEFF87D94E83F06D9948BD57F85A8D55835E7F3C4672D507AD");

    // Example for SHAKE_256
    // This example is not fit to the standard.
    // The method absorb_array() is better than this method to use for the type SHAKE_256.
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SHAKE_256::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "AF96786F00A9A965EBECF2FC38558BE143967CFE1E6E48352B4115BD6F567620F56AEDFB7B5C66838E2135BCE776151F7286E34DE924FF91701D4F10DF3868AD32696D5EACE1988D9C07FFF0D872F12BA21DD76F728CE47BE236904780FE1CE64062A42421D4659A7F5B9C1674ACB62F48802A1055BE9EF7F61EAB0392B5400BF62D93EBD07771DC");

    // Example for KECCAK_384
    // This example is not fit to the standard.
    // The method absorb_array() is better than this method to use for the type KECCAK_384.
    use cryptocol::hash::KECCAK_384;
    let mut hash = KECCAK_384::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = KECCAK_384::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "B5EFD551D15540FAFCC9B4E2A97706C1778CAEFC9EBFF95A0145BB66E93E62835D358E6F95814EE23840314BA531A63BCCCF023754F6DCB28C48341AF998D283BAE6BC06C358852FC92DCD6EAB5BF6E97F24F8738599DCA9420363F44F3DA72D060CC32539FE17C2");

    // Example for BIG_cSHAKE_768
    use cryptocol::hash::BIG_cSHAKE_768;
    let mut hash = BIG_cSHAKE_768::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_768::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "973F18EA445C6515E4A1EF6B38800A8F3EC6E6CD9855ECDCE706FDB629025CFBB93D567D545384BE3758505FF0050023769B23F194DF8FEF5E46872E06BCC9A2EDF4B720D37C4DCB5ED01A305A447C5EF7856BF4A31E9943290D180F17EF26CB45728DC55F8E6DE4D8857F3C84541AC4F836BEA78A534DA5F784BF6247A3619A07725E5EBB04B429187093153336E9A6FD96E9A70221CFA1D501DD67F3A313E93ADF6732B0B688B63B9DD5C54511724FC26AA80E487F5EE34B7259636369E601059F3BCD17BF8B7A9EE64FA402CF0C59");

    // Example for SMALL_cSHAKE_128
    use cryptocol::hash::SMALL_cSHAKE_128;
    let mut hash = SMALL_cSHAKE_128::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "7C50B8DE9218CBD47030B9AF035A6A9B817347CDABDAA2E50555BB6852283E4C15FA557D1EEB9F5BC2BA4D3AB2DF344A2A800781C801C2C3E0BAFE6FD5E958B87C5492AB");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SMALLER_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "B7C9B22DF873C1355E45EDE39211C8226E32");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let user_defined = [1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_array_customized(&[0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = TINY_cSHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "74BE9675CAB68B809A");
    println!("======================");
}

fn sha3_absorb_vec_customized()
{
    println!("sha3_absorb_vec_customized");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "1307CF49A76814AC1D48ADE39AEBC3FADC746604B8C0376B0F76AA01BF4395C097C316B00278F63B080CA5662B4588D38D25A8599414064D91661F2F5B850266C4436230F8557E5CA45A5E295205B99B00EA3756700E85A5C4C3EAFD7948E7C9AFF5015930DFCEF8DCC22E36A1CA896AB4AA501E849047DACF8702644AE746011D63ADB8A3CC1F1446B8028182485C081B87523FBE0BA5E70F3AD020829E9293767E34CE275D30CC");

    // Example for SHA3_512
    // This example is not fit to the standard.
    // The method absorb_vec() is better than this method to use for the type SHA3_512.
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SHA3_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "2BF945A4D1AAC6E3E9931BC1E31D60FE781270E0849877DCDA90F1C9F7D7A8861ECDEFF4DB4881CF74DF465D03E9B481AE3F58F57B342FD0D624E3C82340D22692D6AEBE57D00AA1");

    // Example for SHAKE_128
    // This example is not fit to the standard.
    // The method absorb_vec() is better than this method to use for the type SHAKE_128.
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "C5B2817F6D76AB55AD9DE10BE38DFCCBBC2A8BF1253C20C91BF55AC261A3B230076E562E03DF7D1A20BA2EA819FC317981FAE5688C0B9CFAAC40DE539C5FCEF6791E57941F8CB5C55FD6B7ED59DF25B54C384C12B0682753491C0227FCDC209C30054F5AED56CCE60B5607F7C80F855565D3B5E98EF4D711754F071C1D09EEB8E9C433E98247A074574F9AF75CF00D587C841FFB74A16A8C01E617CDB6DE11C352F164F0617BC607");

    // Example for KECCAK_224
    // This example is not fit to the standard.
    // The method absorb_vec() is better than this method to use for the type KECCAK_224.
    use cryptocol::hash::KECCAK_224;
    let mut hash = KECCAK_224::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = KECCAK_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "16F4344DD7D648676673C29CB09EF4E5ED538CDFA9F648D652C92FC70A0C240971ACBB896E73E471D431D560225A92B99E92DCFAFD3CC7A703C86EE97DAF8F5AA1697369E801ECF9FBCC3134B9801844717187B0652B3FB041B7C59E94DE6473C241767F0A6747F621BB460A6EBA2ABBFB4321C846A3B2B100B3C606413AD7DADCCA60B64C8D61C800354931D9E30B99");

    // Example for BIG_cSHAKE_512
    use cryptocol::hash::BIG_cSHAKE_512;
    let mut hash = BIG_cSHAKE_512::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = BIG_cSHAKE_512::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "45B8C5DCBE132626EDD5623B0E74395F067F009F8C7F7764CC749E53EEB592B1E1983E281B84BA8405744B59312ECDE5A4B0355A2B002FA83C065A2BCB3E6FF55047307D44CEE6B5FAE5731480DE3CCAFBC1B67CE6B158520ABE3067BDBBE90B07C5D68821543C7EEC62852CD870D19F4F08C33E880782595D3E1AA27FAC30F4D5C1AA0482675AB1CF94FFF33E7FED8D7CB415DFC5BCA8FC9BE2FDBF13FA49103740A9BF617A91E6502FF1C44116C97A70717514BBAAA2B3336DB6BA8270DC80B0CF1112BAF5204C5911B59512F96A92CEB9A84BA70C1FAC280F94728DB7F685ED09DEF7BF4073E1C29E172189BCA3C802F4A5DBE24F24D358FD3D5E565494A2814BE317CBCA48950730F3BCC5347F2E");

    // Example for SMALL_cSHAKE_224
    use cryptocol::hash::SMALL_cSHAKE_224;
    let mut hash = SMALL_cSHAKE_224::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SMALL_cSHAKE_224::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "ECCCB09A7E9EB86CDC150433DAAEC792E537C25E558F559668E7E9BEB799C49F1C4AB1CC048215");

    // Example for SMALLER_cSHAKE_128
    use cryptocol::hash::SMALLER_cSHAKE_128;
    let mut hash = SMALLER_cSHAKE_128::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = SMALLER_cSHAKE_128::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "B7C9B22DF873C1355E45EDE39211C8226E32");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let mut hash = TINY_cSHAKE_64::new();
    let user_defined = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.absorb_vec_customized(&vec![0_u8; 0], &user_defined, &data);
    let hash_value = hash.squeeze();
    let hs = TINY_cSHAKE_64::read_hash_value_in_hexadecimal(&hash_value);
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "74BE9675CAB68B809A");
    println!("======================");
}

fn sha3_tangle()
{
    println!("sha3_tangle");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let mut hash = SHA3_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "FA126A570281297F1B8F4075BECD6CD4263AB45A746D37CE2323560E876021A3");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "762C52E75427AD19FD29FF6606CFC1E09DD9038C2B23489591A6288AA69F0182");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "1959308B6378EB3613A1F9A6FFBB77534549A5FDD0BE06A9D3A7988D92F8B0CB");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F799A071FE356696916EC29802116885DE01DAC154CB3FADD4243F050751414E6787D6E64D7F43B1AD053FF67A2166D0F225963C8BEFAEFA45300478B83B6923");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "35D22F6F3915E82335804584C3B6EC6B106DD14A62F4C85060E783561F61C29EE6491E6FF8647CDFEB95861DF6DA0FC5496F335E9B7C998AB0E9BD34A5CBEA6F");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "ED2660FF383BBA663A340BE7E8F2FA7E620E7B2B23085ACE360BBCC310EB3F07F7B2CF7DF27BB75B2A55675F7A26D6C3BA96C5ABA00DC2A9DAF407BCE1365328");

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let mut hash = cSHAKE_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "F799A071FE356696916EC29802116885DE01DAC154CB3FADD4243F050751414E6787D6E64D7F43B1AD053FF67A2166D0F225963C8BEFAEFA45300478B83B6923");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "35D22F6F3915E82335804584C3B6EC6B106DD14A62F4C85060E783561F61C29EE6491E6FF8647CDFEB95861DF6DA0FC5496F335E9B7C998AB0E9BD34A5CBEA6F");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "ED2660FF383BBA663A340BE7E8F2FA7E620E7B2B23085ACE360BBCC310EB3F07F7B2CF7DF27BB75B2A55675F7A26D6C3BA96C5ABA00DC2A9DAF407BCE1365328");

    // Example for KECCAK_256
    use cryptocol::hash::KECCAK_256;
    let mut hash = KECCAK_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "266F0D8A65072603CC4A0711E1888835D3D14A97D852F446E78B3232AB1C5C0B");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "43039CA4C36C5AC7279E4BEFAD43E4F8EF14537386D67275986498E8837210C8");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "466B0F464DF7603281941CBDD0D6B5DF58A1EE2EACF81AF5439FC04A69A66586");

    // Example for BIG_SHAKE_256
    use cryptocol::hash::BIG_SHAKE_256;
    let mut hash = BIG_SHAKE_256::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "77CDCC4B4B833ED6A93FD5591935CC13C84722A72BAE82CF163ED6FAE32B5901A4B734FBB106F330780255F3BF6B7345433DF1F31CE3F13033E96844A776D31F");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "1D9036F0570BFA61E49D6ED32B53735895773251D9DE2B6A65653D129421CA94BEFA3215F2F18ED953A7F34C78B2DCBC4446A292EF486B38639AB28E7B8DA869");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "A4E478172B1BAF50FF077DC7C393C448943AE44FD0829E103654BF89C5707F9CA4EE065A3C7B0B3914C4E2E1B3E9CB895A11FA065EDD88167B3FC06314CB1E9D");

    // Example for SMALL_SHAKE_224
    use cryptocol::hash::SMALL_SHAKE_224;
    let mut hash = SMALL_SHAKE_224::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "8BC5574BB7AEE312EB34A82C21D382727436F2E1524F159B9871EEC4C0F801D4358434AFF15B440BC9518EE64E3F3C555D95C26D7CF4B96E3C13D7AA79");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "ABCB5B39E44E8187967EE5C15B18D54BA55A68389025BB641F9739BC4E8FD8860F07936F92B3C2109AB18B69902356A588BB96EDA5D38711F8972FEEFE");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "720A96B939BFC4FC0EBB4BF481D6B376E9D184753C53A9F7855B5B0EF11377610EE732DA3895FC0220F512D830D1240153F2E204DBD1CC8C04281B608F");

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "51DB9BA6BB23E23D8DB9B5121C9F07AA");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "6F4E68AD77FA883BE35E31AA511F9931");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "366A55B63EB696EFE8F95C8B7272C3C4");

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let mut hash = TINY_KECCAK_64::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    let hs = hash.get_hash_value_in_string();
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hs);
    assert_eq!(hs, "891D22A1BCE2A372");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "D78A4D8AB8042629");
    hash.tangle(1);
    let hs = hash.get_hash_value_in_string();
    println!("Hash =\t{}", hs);
    assert_eq!(hs, "0A08F97DEE3AF7F6");
    println!("======================");
}

fn sha3_get_desirable_l()
{
    println!("sha3_get_desirable_l");

    // Example for SHA3_224
    use cryptocol::hash::SHA3_224;
    let L = SHA3_224::get_desirable_l();
    println!("The desirable L of SHA3-244 is {}", L);
    assert_eq!(L, 6);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let L = SHAKE_256::get_desirable_l();
    println!("The desirable L of SHAKE-256 is {}", L);
    assert_eq!(L, 6);

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let L = cSHAKE_256::get_desirable_l();
    println!("The desirable L of cSHAKE-256 is {}", L);
    assert_eq!(L, 6);

    // Example for KECCAK_384
    use cryptocol::hash::KECCAK_384;
    let L = KECCAK_384::get_desirable_l();
    println!("The desirable L of KECCAK-384 is {}", L);
    assert_eq!(L, 6);

    // Example for BIG_SHA3_224
    use cryptocol::hash::BIG_SHA3_224;
    let L = BIG_SHA3_224::get_desirable_l();
    println!("The desirable L of BIG_SHA3_224 is {}", L);
    assert_eq!(L, 7);

    // Example for SMALL_SHA3_384
    use cryptocol::hash::SMALL_SHA3_384;
    let L = SMALL_SHA3_384::get_desirable_l();
    println!("The desirable L of SMALL_SHA3_384 is {}", L);
    assert_eq!(L, 5);

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let L = SMALLER_SHAKE_128::get_desirable_l();
    println!("The desirable L of SMALLER_SHAKE_128 is {}", L);
    assert_eq!(L, 4);

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let L = TINY_SHA3_64::get_desirable_l();
    println!("The desirable L of TINY_SHA3_64 is {}", L);
    assert_eq!(L, 3);
    println!("======================");
}

fn sha3_get_desirable_rounds()
{
    println!("sha3_get_desirable_rounds");

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let ROUND = SHA3_512::get_desirable_rounds();
    println!("The desirable ROUND of SHA3-512 is {}", ROUND);
    assert_eq!(ROUND, 24);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let ROUND = SHAKE_128::get_desirable_rounds();
    println!("The desirable ROUND of SHAKE-128 is {}", ROUND);
    assert_eq!(ROUND, 24);

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let ROUND = cSHAKE_128::get_desirable_rounds();
    println!("The desirable ROUND of cSHAKE-128 is {}", ROUND);
    assert_eq!(ROUND, 24);

    // Example for KECCAK_224
    use cryptocol::hash::KECCAK_224;
    let ROUND = KECCAK_224::get_desirable_rounds();
    println!("The desirable ROUND of KECCAK-224 is {}", ROUND);
    assert_eq!(ROUND, 24);

    // Example for BIG_KECCAK_256
    use cryptocol::hash::BIG_KECCAK_256;
    let ROUND = BIG_KECCAK_256::get_desirable_rounds();
    println!("The desirable ROUND of BIG_KECCAK_256 is {}", ROUND);
    assert_eq!(ROUND, 26);

    // Example for SMALL_KECCAK_384
    use cryptocol::hash::SMALL_KECCAK_384;
    let ROUND = SMALL_KECCAK_384::get_desirable_rounds();
    println!("The desirable ROUND of SMALL_KECCAK_384 is {}", ROUND);
    assert_eq!(ROUND, 22);

    // Example for SMALLER_SHA3_128
    use cryptocol::hash::SMALLER_SHA3_128;
    let ROUND = SMALLER_SHA3_128::get_desirable_rounds();
    println!("The desirable ROUND of SMALLER_SHA3_128 is {}", ROUND);
    assert_eq!(ROUND, 20);

    // Example for TINY_SHAKE_64
    use cryptocol::hash::TINY_SHAKE_64;
    let ROUND = TINY_SHAKE_64::get_desirable_rounds();
    println!("The desirable ROUND of TINY_SHAKE_64 is {}", ROUND);
    assert_eq!(ROUND, 18);
    println!("======================");
}

fn sha3_get_desirable_b()
{
    println!("sha3_get_desirable_b");

    // Example for SHA3_384
    use cryptocol::hash::SHA3_384;
    let B = SHA3_384::get_desirable_b();
    println!("The desirable B of SHA3-384 is {}", B);
    assert_eq!(B, 200);

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let B = SHAKE_128::get_desirable_b();
    println!("The desirable B of SHAKE-128 is {}", B);
    assert_eq!(B, 200);

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let B = cSHAKE_128::get_desirable_b();
    println!("The desirable B of cSHAKE-128 is {}", B);
    assert_eq!(B, 200);

    // Example for KECCAK_512
    use cryptocol::hash::KECCAK_512;
    let B = KECCAK_512::get_desirable_b();
    println!("The desirable B of KECCAK-512 is {}", B);
    assert_eq!(B, 200);

    // Example for BIG_SHAKE_224
    use cryptocol::hash::BIG_SHAKE_224;
    let B = BIG_SHAKE_224::get_desirable_b();
    println!("The desirable B of BIG_SHAKE_224 is {}", B);
    assert_eq!(B, 400);

    // Example for SMALL_SHAKE_128
    use cryptocol::hash::SMALL_SHAKE_128;
    let B = SMALL_SHAKE_128::get_desirable_b();
    println!("The desirable B of SMALL_SHAKE_128 is {}", B);
    assert_eq!(B, 100);

    // Example for SMALLER_KECCAK_128
    use cryptocol::hash::SMALLER_KECCAK_128;
    let B = SMALLER_KECCAK_128::get_desirable_b();
    println!("The desirable B of SMALLER_KECCAK_128 is {}", B);
    assert_eq!(B, 50);

    // Example for TINY_KECCAK_64
    use cryptocol::hash::TINY_KECCAK_64;
    let B = TINY_KECCAK_64::get_desirable_b();
    println!("The desirable B of TINY_KECCAK_64 is {}", B);
    assert_eq!(B, 25);
    println!("======================");
}

fn sha3_get_desirable_output_length()
{
    println!("sha3_get_desirable_output_length");

    // Example for SHA3_256
    use cryptocol::hash::SHA3_256;
    let LENGTH = SHA3_256::get_desirable_output_length();
    println!("The desirable output length of SHA3-256 is {}", LENGTH);
    assert_eq!(LENGTH, 32);

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let LENGTH = SHAKE_256::get_desirable_output_length();
    println!("The desirable output length of SHAKE-256 is {}", LENGTH);
    assert_eq!(LENGTH, 64);

    // Example for cSHAKE_256
    use cryptocol::hash::cSHAKE_256;
    let LENGTH = cSHAKE_256::get_desirable_output_length();
    println!("The desirable output length of cSHAKE-256 is {}", LENGTH);
    assert_eq!(LENGTH, 64);

    // Example for KECCAK_256
    use cryptocol::hash::KECCAK_256;
    let LENGTH = KECCAK_256::get_desirable_output_length();
    println!("The desirable output length of KECCAK-256 is {}", LENGTH);
    assert_eq!(LENGTH, 32);

    // Example for BIG_SHA3_1536
    use cryptocol::hash::BIG_SHA3_1536;
    let LENGTH = BIG_SHA3_1536::get_desirable_output_length();
    println!("The desirable output length of BIG_SHA3_1536 is {}", LENGTH);
    assert_eq!(LENGTH, 192);

    // Example for SMALL_SHA3_256
    use cryptocol::hash::SMALL_SHA3_256;
    let LENGTH = SMALL_SHA3_256::get_desirable_output_length();
    println!("The desirable output length of SMALL_SHA3_256 is {}", LENGTH);
    assert_eq!(LENGTH, 32);

    // Example for SMALLER_SHAKE_128
    use cryptocol::hash::SMALLER_SHAKE_128;
    let LENGTH = SMALLER_SHAKE_128::get_desirable_output_length();
    println!("The desirable output length of SMALLER_SHAKE_128 is {}", LENGTH);
    assert_eq!(LENGTH, 32);

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let LENGTH = TINY_SHA3_64::get_desirable_output_length();
    println!("The desirable output length of TINY_SHA3_64 is {}", LENGTH);
    assert_eq!(LENGTH, 8);
    println!("======================");
}

fn sha3_224()
{
    println!("sha3_224");
    use cryptocol::hash::SHA3_224;
    
    let mut sha3 = SHA3_224::new();
    let data = [0xA3_u8; 8 * 25];
    sha3.absorb_array(&data);
    let txt = sha3.get_hash_value_in_string();
    println!("sha3 = {}", txt);
    assert_eq!(txt, "9376816ABA503F72F96CE7EB65AC095DEEE3BE4BF9BBC2A1CB7E11E0");
    let mut sha3 = SHA3_224::new();
    sha3.absorb_array(&data);
    let v = sha3.get_hash_value_in_vec();
    print!("sha3 = ");
    for vv in v
    {
        print!("{:02X}", vv);
    }
    println!("\n");
    println!("======================");
}

fn sha3_256()
{
    println!("sha3_256");
    use cryptocol::hash::SHA3_256;
    
    let mut sha3 = SHA3_256::new();
    sha3.absorb_str("");
    let txt = sha3.get_hash_value_in_string();
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
    let txt = sha3.get_hash_value_in_string();
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
    let txt = sha3.get_hash_value_in_string();
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
    let txt = shake.get_hash_value_in_string();
    println!("shake = {}", txt);
    assert_eq!(txt, "7F9C2BA4E88F827D616045507605853ED73B8093F6EFBC88EB1A6EACFA66EF26");
    println!("======================");
}

fn shake_256()
{
    println!("shake_256");
    use cryptocol::hash::SHAKE_256;
    
    let mut shake = SHAKE_256::new();
    shake.absorb_str("");
    let txt = shake.get_hash_value_in_string();
    println!("shake = {}", txt);
    assert_eq!(txt, "46B9DD2B0BA88D13233B3FEB743EEB243FCD52EA62B81B82B50C27646ED5762FD75DC4DDD8C0F200CB05019D67B592F6FC821C49479AB48640292EACB3B7C4BE");
    println!("======================");
}

fn keccak_224()
{
    println!("keccak_224");
    use cryptocol::hash::KECCAK_224;

    let mut keccak = KECCAK_224::new();
    keccak.absorb_str("");
    let txt = keccak.get_hash_value_in_string();
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
    let txt = keccak.get_hash_value_in_string();
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
    let txt = keccak.get_hash_value_in_string();
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
    let txt = keccak.get_hash_value_in_string();
    println!("keccak = {}", txt);
    assert_eq!(txt, "0EAB42DE4C3CEB9235FC91ACFFE746B29C29A8C366B7C60E4E67C466F36A4304C00FA9CAF9D87976BA469BCBE06713B435F091EF2769FB160CDAB33D3670680E");
    println!("======================");
}

fn cshake_128()
{
    println!("cshake_128");
    use cryptocol::hash::cSHAKE_128;
    
    let mut shake = cSHAKE_128::new();
    shake.absorb_vec_customized(&"".to_string().into_bytes(), &"Email Signature".to_string().into_bytes(), &vec![0_u8, 1, 2, 3]);
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "C1C36925B6409A04F1B504FCBCA9D82B4017277CB5ED2B2065FC1D3814D5AAF5");
    
    let mut data = Vec::<u8>::new();
    for i in 0..200
        { data.push(i as u8); }
    shake.absorb_vec_customized(&"".to_string().into_bytes(), &"Email Signature".to_string().into_bytes(), &data);
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "C5221D50E4F822D96A2E8881A961420F294B7B24FE3D2094BAED2C6524CC166B");

    shake.absorb_str_customized("PARK", "Youngho", "In the beginning God created the heavens and the earth.");
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "25685E3E59672612F86AB24E418EB610B6F5F7D299E1B315FD9B59BD698A4AC9");
    println!("======================");
}

fn cshake_256()
{
    println!("cshake_256");
    use cryptocol::hash::cSHAKE_256;
    
    let mut shake = cSHAKE_256::new();
    shake.absorb_str_customized("PARK", "Youngho", "In the beginning God created the heavens and the earth.");
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "490561E775E7CCF90084522B6D8F9DDFAF087C198ABC788737DAC198795242A29B520951D09E8096C441EF88EB53B48AC43B2E7FF9416CF9F32A897BA2EE99FC");
    println!("======================");
}
