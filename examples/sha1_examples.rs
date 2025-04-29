// Copyright 2023, 2024 PARK Youngho.
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
    sha1_quick_start1();
    sha1_quick_start2();
    sha1_quick_start3();
    sha1_quick_start4();
    sha1_new();
    sha1_digest();
    sha1_digest_str();
    sha1_digest_string();
    sha1_digest_array();
    sha1_digest_vec();
    sha1_ruminate();
    sha1_ruminate_str();
    sha1_ruminate_string();
    sha1_ruminate_array();
    sha1_ruminate_vec();
    sha1_get_hash_value();
    sha1_get_hash_value_in_string();
    sha1_get_hash_value_in_array();
    sha1_get_hash_value_in_vec();
    sha1_put_hash_value_in_array();
    sha1_tangle();
    sha1_fmt_for_to_string();
    sha1_fmt_for_println();
}


fn sha1_quick_start1()
{
    println!("sha1_quick_start1");
    use std::string::*;
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "6DCD4CE23D88E2EE9568BA546C007C63D9131C1B");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "0BBCDBD1616A1D2230100F629649DCF5B7A28B7F");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B82A61505779F6B3ACA4F5E0D54DA44C17375B49");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6DC54281357FC16D357E1D730BFC313C585DAEC");

    txt = "I am testing SHA1 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "36CD36337097D764797091E5796B6FF45A9FA79F");

    txt = "I am testing SHA-1 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E408F6B82DCDDB5EE6613A759AC1B13D0FA1CEF1");

    txt = "I am testing SHA1 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BB2C79F551B95963ECE49D40F8A92349BF66CAE7");
    println!("-------------------------------");
}

fn sha1_quick_start2()
{
    println!("sha1_quick_start2");
    use std::string::*;
    use cryptocol::hash::SHA0;
    let mut hash = SHA0::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "F96CEA198AD1DD5617AC084A3D92C6107708C0EF");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "E4DA6A8FBD813C90E6FA040D5F15398ECA200339");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "72CFDDBCDCCCC0847DA8AA7FDBA901A2FC431068");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B56263EB76AE1ABA8E7E4A4CA104BC78F1BC8D7A");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "613FEB0029DF4FE0D16CBA8AAFA596D9BC309D18");

    txt = "I am testing SHA0 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "E0351ED0E4FDD2F5731A2E7472B08038B10AFB0D");

    txt = "I am testing SHA-0 for the data whose length is sixty-four bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "270CCCFD32361F7C01427D9F64B2248C6C88D080");

    txt = "I am testing SHA0 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "0E71D76AC85D342DB566EDCFC971B6E06C5D7CBC");
    println!("-------------------------------");
}

fn sha1_quick_start3()
{
    println!("sha1_quick_start3");
    use std::string::*;
    use cryptocol::hash::SHA1_Expanded;

    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA1::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "6C64DE0C4E997B2EE6DD562DBC43D2A1CB53F186");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "DA0918B8FDF524572C293C4971CCE4071E14CE30");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6FC72A4D3A72DC40FF8B601DA4F1A626210EBC4");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "58CEBE40E0391A38724EA06F327946C70C5585B2");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "756049AC909ACA9BC1A9213E0148402156B0DC7F");
    
    txt = "I am testing SHA1_Expanded for the data of sixty-two byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "18AD939F2027D8FEF249E533ECEC4BC551558317");
    
    txt = "I am testing SHA1_Expanded for the data whose length is 64 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "89DE08E440FEDA32C10C704B0741E7EBBA8F74F4");

    txt = "I am testing SHA1_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E131AE2668538116730817D639801D639AD9B883");
    println!("-------------------------------");
}

fn sha1_quick_start4()
{
    println!("sha1_quick_start4");
    use std::string::*;
    use cryptocol::hash::SHA0_Expanded;

    type MySHA0 = SHA0_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA0::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "DC1EF9093D309849ABA5EDF152DB32C695421438");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "244743C28AE0223A6A0661813221864CC2EDBA35");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2BBE776DC9E577B444CDA082EE31A87DB3C4EF57");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "170E3BCFC4DDE939C7BF62C2F28A3DED07041407");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "E3EDDB4717F72DC7E703139D60964860A6AB316B");
    
    txt = "I am testing SHA0_Expanded for the data of sixty-two byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "ADF444B96A666E409E10452F634E0830B704C4EA");
    
    txt = "I am testing SHA0_Expanded for the data whose length is 64 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "F0148F75FEF02934F69B18870FEC85DFE215B2AF");

    txt = "I am testing SHA0_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "C9A0A2597CAB6E3A6C0009FA6552F46814DBCB54");
    println!("-------------------------------");
}

fn sha1_new()
{
    println!("sha1_new");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let hash = SHA1::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "67452301EFCDAB8998BADCFE10325476C3D2E1F0");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let my_hash = MySHA1::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888CCCCCCCCFFFFFFFF");
    println!("-------------------------------");
}

fn sha1_digest()
{
    println!("sha1_digest");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9631162DFDAEAB89821256D4585D66D35CD61FD6");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FAF77A15CDEDFC6A33CE2C4003B119F225CBE414");
    println!("-------------------------------");
}

fn sha1_digest_str()
{
    println!("sha1_digest_str");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9FDE56BBB5028966CC2E7BDCD0758FE3121407E6");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "A6BE8FEA7E3F61508DC0A8BA85A0AEC77D0C0784");
    println!("-------------------------------");
}

fn sha1_digest_string()
{
    println!("sha1_digest_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "FDCDC0EBC9181B881BE1F15FECEBB9D70E4DDAAB");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "F4FE5C5A4D2A4BD414DDDF1FD32B185F3ED8AA32");
    println!("-------------------------------");
}

fn sha1_digest_array()
{
    println!("sha1_digest_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
    println!("-------------------------------");
}

fn sha1_digest_vec()
{
    println!("sha1_digest_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "76BC87BAECA7725C948FD1C53766454FDA0867AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A6E00DB72776DEBB7C6DB235024BB3E237E24D18");
    println!("-------------------------------");
}

fn sha1_ruminate()
{
    println!("sha1_ruminate");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "1E91427CF3BBB256A2BD44DA9F89D7406ED5D5FE");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut hash = MySHA1::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "509038D0447A5D05F4AD62C25AD6F9E130E694F4");
    println!("-------------------------------");
}

fn sha1_ruminate_str()
{
    println!("sha1_ruminate_str");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "778B3FF529024A46A3CC06F01CBE9078F6447BC0");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "0CFDD49B87B844C4C329C997C1FB650EBEEA4909");
    println!("-------------------------------");
}

fn sha1_ruminate_string()
{
    println!("sha1_ruminate_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F4CE0B5A8D93BEB1C0A99F6290B26661C212A8B3");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "C4B55C59A15FCDFF6FFD39D3867665F67E89C8FC");
    println!("-------------------------------");
}

fn sha1_ruminate_array()
{
    println!("sha1_ruminate_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "35BC04C66EBA9751C482FD98BCD1CBDC2C5E56AF");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "898835EC92B5F7818A25C6645673DED30DA5F78D");
    println!("-------------------------------");
}

fn sha1_ruminate_vec()
{
    println!("sha1_ruminate_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "042811212E91F341473A43BF71BD8DA035D23032");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "99B8709ACB93051C4CB238CE9CD9031BD40F2A2B");
    println!("-------------------------------");
}

fn sha1_get_hash_value()
{
    println!("sha1_get_hash_value");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 20];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[82, 62, 1B, E6, A6, 74, 88, 18, 12, 60, 5F, 27, C7, EF, 19, 38, 65, 39, 00, 8A]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 20];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[F5, DD, 99, 0C, 9B, 5A, 4C, A3, 84, DF, B1, 3D, 73, 5A, CE, CF, 19, BB, 52, B4]");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_string()
{
    println!("sha1_get_hash_value_in_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "826621B45597FA1B58C855DFCDE111E7500BCC96");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "72CEC05D49E2FA7206E2BF5A6C9D38F0404E7956");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_array()
{
    println!("sha1_get_hash_value_in_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_array()), "[7647F56F, 1508A320, 2303B1A8, D3BB7325, FC4497F8]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C7DD61D1, 4E88AC6C, FFFC2A7E, C8E2DA66, 01BD283D]");
    println!("-------------------------------");
}

fn sha1_get_hash_value_in_vec()
{
    println!("sha1_get_hash_value_in_vec");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:02X?}", hash.get_hash_value_in_vec()), "[58271E8F, 7E54E508, CF099E8F, 4D3B597B, D3BE3F42]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[DA959A5F, A8B581AD, FC006FB0, 9CCB3BCF, 7F4732F3]");
    println!("-------------------------------");
}

fn sha1_put_hash_value_in_array()
{
    println!("sha1_put_hash_value_in_array");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[BC02B27F, 99A5A1FB, A820CEC4, 19516BC8, E4D2A0D6]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MySHA1::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 5];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[91EF4936, CFCF8F2D, C581EF30, 450E4E05, 0FBD39A7]");
    println!("-------------------------------");
}

fn sha1_tangle()
{
    println!("sha1_tangle");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let txt = "TANGLING";
    let mut hash = SHA1::new();
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B296514, 79D48A17, 1ADABF55, 09CC69B9, 83477776]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[6D00CD91, 2A9BAD37, 210A8909, B6A83E2F, 5D986325]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E41C001F, 476FDC14, 1166767C, 3C09AE4D, 447B9B2F]");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let txt = "TANGLING";
    let mut my_hash = MySHA1::new();
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[570C0960, 44388BBA, 0DD84AC9, 2F78A2F8, E514D1FD]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[AE8C42A9, 4CFC9130, FF606528, E4876633, 27FC359F]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2E33CBCF, 800599AD, 98827D7A, 41AA8BCB, D2D011FD]");
    println!("-------------------------------");
}

fn sha1_fmt_for_to_string()
{
    println!("sha1_fmt_for_to_string");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "8D0A6284BBFF4DE8D68962A924842C80959B0404");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "54F0234F7188202D98EDDC643F71D95BEDE77ED7");
    println!("-------------------------------");
}

fn sha1_fmt_for_println()
{
    println!("sha1_fmt_for_println");
    // Example for SHA1
    use cryptocol::hash::SHA1;
    let mut hash = SHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "835CEFA297628E4DADBDA011C5FDEA68D88A8EE8");

    // Example for SHA1_Expanded
    use cryptocol::hash::SHA1_Expanded;
    type MySHA1 = SHA1_Expanded<5, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xcccc_cccc, 0xffff_ffff, 160>;
    let mut my_hash = MySHA1::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "78083F4E573928D6C4E9F869036F8A4D4D549E9F");
    println!("-------------------------------");
}
