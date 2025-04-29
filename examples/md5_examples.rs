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
    md5_quick_start1();
    md5_quick_start2();
    md5_new();
    md5_digest();
    md5_digest_str();
    md5_digest_string();
    md5_digest_array();
    md5_digest_vec();
    md5_ruminate();
    md5_ruminate_str();
    md5_ruminate_string();
    md5_ruminate_array();
    md5_ruminate_vec();
    md5_get_hash_value();
    md5_get_hash_value_in_string();
    md5_get_hash_value_in_array();
    md5_get_hash_value_in_vec();
    md5_put_hash_value_in_array();
    md5_tangle();
    md5_fmt_for_to_string();
    md5_fmt_for_println();
}

fn md5_quick_start1()
{
    println!("md5_quick_start1");
    use std::string::*;
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "D41D8CD98F00B204E9800998ECF8427E");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "7FC56270E7A70FA81A5935B72EACBE29");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "49DC5E45FBEC1433E2C612E5AA809C10");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "17ED1DB5CD96184041659D84BB36D76B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C3EB6D4A1071E1A9C5E08FEF6E8F3FBF");

    txt = "I am testing MD5 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "6C33614E6317DC4641573E0EBC287F98");

    let mut txt = "I am testing MD5 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "584D41C6837AC714275196E4FF14B2EF");

    txt = "I am testing MD5 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "9831162AB272AE1D85245B75726D215E");
    println!("-------------------------------");
}

fn md5_quick_start2()
{
    println!("md5_quick_start2");
    use std::string::*;
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut hash = MyMD5::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2793C0925118EEA53C288640AA7D9C81");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "78DBB9B3C63B704745BFF37E3254B350");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "298B65824C2F415446A6210AB0191B8B");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "80C1F5A7858DEB5A136CE57DC60FCFBE");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "F3B540538E750E62A3F9417C308DD018");

    txt = "I am testing MD5_Expanded for the data of the length 62 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "A136326C1DA54B63CC2743647219BA60");

    txt = "I am testing MD5_Expanded for the message which is 64 byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "C6AC6907DDE0E8429CF44BC21941F64A");

    txt = "I am testing MD5_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "BD4E0962262EBBAE4C4A89FBB2F7A2D4");
    println!("-------------------------------");
}

fn md5_new()
{
    println!("md5_new");
    // Example for MD5
    use cryptocol::hash::MD5;
    let hash = MD5::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let my_hash = MyMD5::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    println!("-------------------------------");
}

fn md5_digest()
{
    println!("md5_digest");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "336EA91DD3216BD0FC841E86F9E722D8");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "51F4248FBEFBE0A00196F9F04DD07FF0");
    println!("-------------------------------");
}

fn md5_digest_str()
{
    println!("md5_digest_str");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "F2E455CEB5FB993A980E67D3FA8A3961");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "21EE03C8185BD65CDB8116D0E2714F09");
    println!("-------------------------------");
}

fn md5_digest_string()
{
    println!("md5_digest_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "40929E789D2F5880B85456E289F704C0");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "02BDBC510B949045A131C0C3302027BA");
    println!("-------------------------------");
}

fn md5_digest_array()
{
    println!("md5_digest_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
    println!("-------------------------------");
}

fn md5_digest_vec()
{
    println!("md5_digest_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "054DE9CF5F9EA623BBB8DC4781685A58");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "A5DC1291539528723C6C3E6F7EFDAE94");
    println!("-------------------------------");
}

fn md5_ruminate()
{
    println!("md5_ruminate");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "71F09FB7840FA1EB78A88ED071627C0D");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut hash = MyMD5::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "CC179809A9DC1475EEF5E4810C272882");
    println!("-------------------------------");
}

fn md5_ruminate_str()
{
    println!("md5_ruminate_str");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "68B3B09AE0EED0D15E744671E29824D4");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "7A460BDA766C6A7D4F9A23DCBDB71A4C");
    println!("-------------------------------");
}

fn md5_ruminate_string()
{
    println!("md5_ruminate_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E06B1A664322C1296D1FCD3F28428493");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "5018AF48C7606F748073FC5255448BAB");
    println!("-------------------------------");
}

fn md5_ruminate_array()
{
    println!("md5_ruminate_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "4914D161AE665750248DF91B6E57C7BE");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "1FBF755293909670FE66B8CA482BCF66");
    println!("-------------------------------");
}

fn md5_ruminate_vec()
{
    println!("md5_ruminate_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "BDEE5A3C5B2DB7B6F18B170C2E865FE0");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 128>;
    let mut my_hash = MyMD5::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "DBFD74659889B373D90477B59A193CBD");
    println!("-------------------------------");
}

fn md5_get_hash_value()
{
    println!("md5_get_hash_value");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[91, 57, 43, 58, C7, F9, 04, 83, 60, 63, 15, CD, 1B, 77, 2E, DD]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[A4, 5C, 46, 58, 29, BB, 83, 06, 32, 4D, 20, 20, 23, 9D, 41, AE]");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_string()
{
    println!("md5_get_hash_value_in_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "5E9D7F0006214CB49D09FC846FBE2927");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "A8BA6619878AE3A8135B7FD2A6ECAE6D");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_array()
{
    println!("md5_get_hash_value_in_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[1FC84032, 1DFA906E, 911B468C, 66EDE0CE]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[06813E1E, DA7BA0BF, 4B48D110, 6B111859]");
    println!("-------------------------------");
}

fn md5_get_hash_value_in_vec()
{
    println!("md5_get_hash_value_in_vec");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[D9A44F09, 27F51F07, 4517E390, 4CF17D73]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[5D9AB684, 090F7AEB, 31FD214E, F03D3032]");
    println!("-------------------------------");
}

fn md5_put_hash_value_in_array()
{
    println!("md5_put_hash_value_in_array");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[512E75DE, 4525528D, 41E8D192, 5606EE3B]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[F8634B80, 96E02659, E26EA89D, EDA8E0C4]");
    println!("-------------------------------");
}

fn md5_tangle()
{
    println!("md5_tangle");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E60545F6, 6DCF2B02, 8245048B, AE2A98C6]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[E0B5F1C0, 5C62629F, 68D44BC1, D384AB34]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[C75EEA9C, 9D5CF62B, 0ABFA634, CD29C2D4]");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[9CCE671A, 5366F625, 68056532, D6B0DA5C]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A12380BC, DE74206D, C145732C, 4CAAD502]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[D9EB87F4, 00C2D299, A492A483, 1C24FCDD]");
    println!("-------------------------------");
}

fn md5_fmt_for_to_string()
{
    println!("md5_fmt_for_to_string");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "ED085603C2CDE77DD0C6FED3EC1A8ADB");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "3FDFF3827C89F3C770A0863F069FE766");
}

fn md5_fmt_for_println()
{
    println!("md5_fmt_for_println");
    // Example for MD5
    use cryptocol::hash::MD5;
    let mut hash = MD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "6C9494A4A5C313001695262D72571F74");

    // Example for MD5_Expanded
    use cryptocol::hash::MD5_Expanded;
    type MyMD5 = MD5_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD5::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "45BA0E4FEA1FACF829D19544A77105B8");
    println!("-------------------------------");
}
