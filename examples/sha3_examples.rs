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

fn sha3_rc()
{
    println!("sha3_rc");
    fn register(reg: u8, rule: u8) -> (u8, u8)
    {
        ((reg >> 1) | (((reg & rule).count_ones() as u8) << 7), reg & 1)
    }

    #[allow(non_snake_case)]
    fn make_rc(L: usize) -> [u128; 26]
    {
        let ROUNDS: usize = 12 + 2 * L;
        let WIDTH: usize = 1 << L; // = 2_usize.pow(L as u32);
        let mut RC = [0_u128; 26];
        let mut bit = [0_usize; 7];
        for j in 0..7_usize
            { bit[j] = ((1_usize << j) - 1) % WIDTH; }
        let mut state = 1_u8;
        let mut output;
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
//     println!("======================");panic!("TEST");
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
    assert_eq!(hash.to_string(), "0000000000000000000000000000000000000000000000000000000000000000");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let hash = cSHAKE_128::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "00000000000000000000000000000000");

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
    assert_eq!(hash.to_string(), "00000000000000000000000000000000");

    // Example for TINY_cSHAKE_64
    use cryptocol::hash::TINY_cSHAKE_64;
    let hash = TINY_cSHAKE_64::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0000000000000000");
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
    assert_eq!(hash.to_string(), "231FEC5DD3B64C278D1EC8BF8BFE4FF3");

    // Example for cSHAKE_128
    use cryptocol::hash::cSHAKE_128;
    let mut hash = cSHAKE_128::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "786CECCA21143CC45A532E526819F734");

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
    assert_eq!(hash.to_string(), "E43978F22B924DB053195009F3C8C66E2382F0FDC43CCE9AE39A3322BD71F03F623B7FA3B3A601F46E3919D3739E30AAD4FAC593E4F6A3BFD8639A75EE5B77387E1D1DF0C4FBB13427E76CFE27A7B0F552B6625E8618B0D502CBE5CF4D389D127AA3F1737170444590AD9386E4C177FF53A60EB95253290EDF9F2BFE7AA42D1656030B01C7BADCE2AF6AE89686A47A75C9080DEE9AC84D4A71EBFE5304F4508270BCEDC3EC187325786E318563727476DE4E892D86647C5218F804A7B80C6085");

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

    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");
    
    println!("======================");
}

fn sha3_digest_string()
{
    println!("sha3_digest_string");

    println!("======================");
}

fn sha3_digest_array()
{
    println!("sha3_digest_array");

    println!("======================");
}

fn sha3_digest_vec()
{
    println!("sha3_digest_vec");

    println!("======================");
}

fn sha3_digest_customized()
{
    println!("sha3_digest_customized");

    println!("======================");
}

fn sha3_digest_str_customized()
{
    println!("sha3_digest_str_customized");

    println!("======================");
}

fn sha3_digest_string_customized()
{
    println!("sha3_digest_string_customized");

    println!("======================");
}

fn sha3_digest_array_customized()
{
    println!("sha3_digest_array_customized");

    println!("======================");
}

fn sha3_digest_vec_customized()
{
    println!("sha3_digest_vec_customized");

    println!("======================");
}

fn sha3_get_hash_value_in_array()
{
    println!("sha3_get_hash_value_in_array");

    println!("======================");
}

fn sha3_get_hash_value_in_vec()
{
    println!("sha3_get_hash_value_in_vec");

    println!("======================");
}

fn sha3_get_hash_code_in_vec()
{
    println!("sha3_get_hash_code_in_vec");

    println!("======================");
}

fn sha3_get_hash_value_in_string()
{
    println!("sha3_get_hash_value_in_string");

    println!("======================");
}

fn sha3_get_hash_code_in_string()
{
    println!("sha3_get_hash_code_in_string");

    println!("======================");
}

fn sha3_push_hash_value_in_array()
{
    println!("sha3_push_hash_value_in_array");

    println!("======================");
}

fn sha3_get_hash_value()
{
    println!("sha3_get_hash_value");

    println!("======================");
}

fn sha3_read_hash_value_in_hexadecimal()
{
    println!("sha3_read_hash_value_in_hexadecimal");

    println!("======================");
}

fn sha3_squeeze()
{
    println!("sha3_squeeze");

    println!("======================");
}

fn sha3_absorb_string()
{
    println!("sha3_absorb_string");

    println!("======================");
}

fn sha3_absorb_array()
{
    println!("sha3_absorb_array");

    println!("======================");
}

fn sha3_absorb_vec()
{
    println!("sha3_absorb_vec");

    println!("======================");
}

fn sha3_absorb_customized()
{
    println!("sha3_absorb_customized");

    println!("======================");
}

fn sha3_absorb_str_customized()
{
    println!("sha3_absorb_str_customized");

    println!("======================");
}

fn sha3_absorb_string_customized()
{
    println!("sha3_absorb_string_customized");

    println!("======================");
}

fn sha3_absorb_array_customized()
{
    println!("sha3_absorb_array_customized");

    println!("======================");
}

fn sha3_absorb_vec_customized()
{
    println!("sha3_absorb_vec_customized");

    println!("======================");
}

fn sha3_tangle()
{
    println!("sha3_tangle");

    println!("======================");
}

fn sha3_get_desirable_l()
{
    println!("sha3_get_desirable_l");

    println!("======================");
}

fn sha3_get_desirable_rounds()
{
    println!("sha3_get_desirable_rounds");

    println!("======================");
}

fn sha3_get_desirable_b()
{
    println!("sha3_get_desirable_b");

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
    println!("\n@@@@@");
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
    assert_eq!(txt, "7F9C2BA4E88F827D616045507605853E");
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
    assert_eq!(txt, "46B9DD2B0BA88D13233B3FEB743EEB243FCD52EA62B81B82B50C27646ED5762F");
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
    assert_eq!(txt, "C1C36925B6409A04F1B504FCBCA9D82B");
    
    let mut data = Vec::<u8>::new();
    for i in 0..200
        { data.push(i as u8); }
    shake.absorb_vec_customized(&"".to_string().into_bytes(), &"Email Signature".to_string().into_bytes(), &data);
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "C5221D50E4F822D96A2E8881A961420F");

    shake.absorb_str_customized("PARK", "Youngho", "In the beginning God created the heavens and the earth.");
    let txt = shake.get_hash_value_in_string();
    println!("cSHAKE = {}", txt);
    assert_eq!(txt, "25685E3E59672612F86AB24E418EB610");
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
    assert_eq!(txt, "490561E775E7CCF90084522B6D8F9DDFAF087C198ABC788737DAC198795242A2");
    println!("======================");
}
