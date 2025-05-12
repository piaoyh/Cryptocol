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

#[allow(unused_imports)]
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
    use cryptocol::hash::SMALLER_KECCAK_128;
    let mut hash = SMALLER_KECCAK_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E11D8A5C39AF9ECD2AF3CFBEC57F46DC");

    // Example for TINY_SHA3_64
    use cryptocol::hash::TINY_SHA3_64;
    let mut hash = TINY_SHA3_64::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EC07E55C6AD49B81");

    // Example for SHA3_512
    use cryptocol::hash::SHA3_512;
    let mut hash = SHA3_512::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "183F74D0A47CE0B3B533A903AC907FEED184D4E39F212F27EE0BF6B9E4E1B7CEAF105A165A6C9CC28DA27261194667B578B4B0B7626E1554340A297B133181C1");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_customized().";
    hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A744B963C7F2CBAD52CBDBF2090173C4593D93B854581F5B623B060CEF4E013A");

    // Example for KECCAK_224
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
    use cryptocol::hash::SHA3_224;
    let mut hash = SHA3_224::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "EC429BB8857A6273E108EAEFA9435867D92C442CCF4B5309795068E9");

    // Example for SHAKE_256
    use cryptocol::hash::SHAKE_256;
    let mut hash = SHAKE_256::new();
    let function_name = "Reserved for NIST";
    let user_defined = "on my own purpose";
    let txt = "This is an example of the method digest_str_customized().";
    hash.digest_str_customized(function_name, user_defined, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C072CDE8B6EA6F63BD756F6BE59A7C44CB51DDBA10E96C19C79FB1286AED9DDB4D3CD5F2CC94F3D7C0505F1888805D32C0CD4FBE10E311D72436576E485DE445");

    // Example for KECCAK_256
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
    use cryptocol::hash::SHA3_384;
    let mut hash = SHA3_384::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "370207DA06E2E85AAEC6C10E15DE9F92F8954AB036A0D10C48DB6A8D2FB5238EC209B2016BDAB94CBFE53FF3ECDF1178");

    // Example for SHAKE_128
    use cryptocol::hash::SHAKE_128;
    let mut hash = SHAKE_128::new();
    let function_name = "Reserved for NIST".to_string();
    let user_defined = "on my own purpose".to_string();
    let txt = String::from("This is an example of the method digest_string_customized().");
    hash.digest_string_customized(&function_name, &user_defined, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BA2EF98EA6944C201267441004368743340D1B32A895C1EF5364F00E0A7A8707");

    // Example for KECCAK_512
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
