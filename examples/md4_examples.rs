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
    md4_quick_start1();
    md4_quick_start2();
    md4_new();
    md4_digest();
    md4_digest_str();
    md4_digest_string();
    md4_digest_array();
    md4_digest_vec();
    md4_ruminate();
    md4_ruminate_str();
    md4_ruminate_string();
    md4_ruminate_array();
    md4_ruminate_vec();
    md4_get_hash_value();
    md4_get_hash_value_in_string();
    md4_get_hash_value_in_array();
    md4_get_hash_value_in_vec();
    md4_put_hash_value_in_array();
    md4_tangle();
    md4_fmt_for_to_string();
    md4_fmt_for_println();
}

fn md4_quick_start1()
{
    println!("md4_quick_start1");
    use std::string::*;
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "31D6CFE0D16AE931B73C59D7E0C089C0");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "D5EF20EEB3F75679F86CF57F93ED0FFE");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6407C0E728DA762A04924ADFE630974C");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "4F4A24D124B996BEA395344419F9A06B");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "9DE35D8FCF68E74867FFB63F28625ABE");

    txt = "I am testing MD4 for the data whose length is sixty-two bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "3A9F1487472B3A4315E0C90DC5CB3A2E");

    txt = "I am testing MD4 for the message which is sixty-four bytes long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "6CDB5B2BFF823A4A7B23675180EB7BEF");

    txt = "I am testing MD4 for the case data whose length is more than sixty-four bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "56771653687981390B0EB2A7D0A40DBB");
    println!("-------------------------------");
}

fn md4_quick_start2()
{
    println!("md4_quick_start2");
    use std::string::*;
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut hash = MyMD4::new();

    let mut txt = "";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "13892AE087B903E5EC030A51E1BC720A");

    let txt_stirng = String::from("A");
    hash.digest_string(&txt_stirng);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt_stirng, hash);
    assert_eq!(hash.to_string(), "0C0BE1B8893E47C005D95C69234141E9");

    let txt_array = ['W' as u8, 'o' as u8, 'w' as u8];
    hash.digest_array(&txt_array);
    println!("Msg =\t\"{:?}\"\nHash =\t{}\n", txt_array, hash);
    assert_eq!(hash.get_hash_value_in_string(), "17545CEB681C5B848234A557C5957AA7");

    txt = "This data is 26-byte long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "70F3EA1DCDE46C65868DC0937E374433");

    txt = "The unit of data length is not byte but bit.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "640B4635ED76F6574FC30AB233B74712");
    
    txt = "I am testing MD4_Expanded for the data of the length 62 bytes.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.to_string(), "B0D18D969B99F4BF48365449AF82EAFB");
    
    txt = "I am testing MD4_Expanded for the message which is 64 byte-long.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}\n", txt, hash);
    assert_eq!(hash.get_hash_value_in_string(), "2D4ADABC3504B4A1B98FCCBFC48145AE");

    txt = "I am testing MD4_Expanded for the case data whose length is more than 64 bytes is given.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "26E5336E4D863BBAD6347918CE6DBAF5");
    println!("-------------------------------");
}

fn md4_new()
{
    println!("md4_new");
    // Example for MD4
    use cryptocol::hash::MD4;
    let hash = MD4::new();
    println!("Hash =\t{}", hash);
    assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let my_hash = MyMD4::new();
    println!("Hash =\t{}", my_hash);
    assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    println!("-------------------------------");
}

fn md4_digest()
{
    println!("md4_digest");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest().";
    hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest().";
    my_hash.digest(txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "B2F465006DCBA147BCE76D7EB8B564E1");
    println!("-------------------------------");
}

fn md4_digest_str()
{
    println!("md4_digest_str");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest_str().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest_str().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "719A1EB0F5077837BB408434B7AAD81E");
    println!("-------------------------------");
}

fn md4_digest_string()
{
    println!("md4_digest_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method digest_string().".to_string();
    hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method digest_string().".to_string();
    my_hash.digest_string(&txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "FD42F7479ED133619D877BB1E6C8A084");
    println!("-------------------------------");
}

fn md4_digest_array()
{
    println!("md4_digest_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_array(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn md4_digest_vec()
{
    println!("md4_digest_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.digest_vec(&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    println!("-------------------------------");
}

fn md4_ruminate()
{
    println!("md4_ruminate");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "23EAC3CEE64E4266EEDFE2D6AB255B9F");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut hash = MyMD4::new();
    let txt = "This is an example of the method ruminate().";
    hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "A1608F7E4052E267B3233862FD5C1C41");
    println!("-------------------------------");
}

fn md4_ruminate_str()
{
    println!("md4_ruminate_str");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate_str().";
    hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "B19769E514631D59FD257C4AD667BD9D");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method ruminate_str().";
    my_hash.ruminate_str(3, txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "534F1EC44D4B2CEF12B7A9A81941D9A8");
    println!("-------------------------------");
}

fn md4_ruminate_string()
{
    println!("md4_ruminate_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "71D3AB5636348DB24A7AE302E7E6C05A");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method ruminate_string().".to_string();
    my_hash.ruminate_string(2, &txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "EFB3B63FC1DBF3852F469D4EA0E8D517");
    println!("-------------------------------");
}

fn md4_ruminate_array()
{
    println!("md4_ruminate_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "810F75A7BD28179BA2D4604A3092FBC8");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_array(5,&data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "27F598D17E6DFBA0A0713F3262D34FFC");
    println!("-------------------------------");
}

fn md4_ruminate_vec()
{
    println!("md4_ruminate_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    assert_eq!(hash.to_string(), "B3E296760B88B44613DB03D72CE59917");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    my_hash.ruminate_vec(2, &data);
    println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    assert_eq!(my_hash.to_string(), "AFC96A14952E9FB9D6D7C7A1FD3D4C2E");
    println!("-------------------------------");
}

fn md4_get_hash_value()
{
    println!("md4_get_hash_value");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    hash.digest_str(txt);
    hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[A7, AD, DF, 36, A2, 43, 97, D1, 6D, 3C, 99, 78, A6, D5, 6E, 74]");
 
    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value().";
    let hash_value = [0_u8; 16];
    my_hash.digest_str(txt);
    my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    assert_eq!(format!("{:02X?}", hash_value), "[02, 43, 79, C6, 08, F1, CA, 30, C0, 75, 5C, 6C, 07, AD, 76, 72]");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_string()
{
    println!("md4_get_hash_value_in_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    assert_eq!(hash.get_hash_value_in_string(), "FA48527AD8257A371E70AA9473D425D6");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    assert_eq!(my_hash.get_hash_value_in_string(), "626192ACD80D62D8966ACE89AE439E76");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_array()
{
    println!("md4_get_hash_value_in_array");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[832C724B, 4A73A717, 5EA679B8, E991D13B]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_array().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2CFB0798, 77AA2A27, 602B457E, AD3B964C]");
    println!("-------------------------------");
}

fn md4_get_hash_value_in_vec()
{
    println!("md4_get_hash_value_in_vec");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[EE74475E, ECA09C8F, 038A89A3, 9B2A6C4F]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method get_hash_value_in_vec().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[440664DA, 49687C74, C0536C83, 192830D8]");
    println!("-------------------------------");
}

fn md4_put_hash_value_in_array()
{
    println!("md4_put_hash_value_in_array");
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    hash.digest_str(txt);
    hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[147DD795, C34F9C9D, 80B94C86, FB922262]");

    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "This is an example of the method put_hash_value_in_array().";
    let mut hash_code = [0_u32; 4];
    my_hash.digest_str(txt);
    my_hash.put_hash_value_in_array(&mut hash_code);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    assert_eq!(format!("{:08X?}", hash_code), "[1411D15D, 37BBE0DF, 1EAF8DA5, AC822C42]");
    println!("-------------------------------");
}

fn md4_tangle()
{
    println!("md4_tangle");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "TANGLING";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[BC65D6E1, F0F37B4E, 2F404331, A8F25E2A]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[CE1E07A3, F3373D70, 95A8F098, 9BC7894E]");
    hash.tangle(1);
    println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B9A2D14, 64888002, 15282E23, E5B2F4BD]");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "TANGLING";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C4377D49, 05FD9A1F, 3DA4E254, ACF22116]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[8CB0AF83, F75E073C, 77C5BF6C, EDFE1D51]");
    my_hash.tangle(1);
    println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A5C900D1, 388193FA, B2C0ED53, 4DE71DDE]");
    println!("-------------------------------");
}

fn md4_fmt_for_to_string()
{
    println!("md4_fmt_for_to_string");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "Display::fmt() automagically implement to_string().";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    assert_eq!(my_hash.to_string(), "6B0D8F0CE90782E5FF88EE57B5DC5AF1");
    println!("-------------------------------");
}

fn md4_fmt_for_println()
{
    println!("md4_fmt_for_println");
    // Example for MD4
    use cryptocol::hash::MD4;
    let mut hash = MD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");

    // Example for MD4_Expanded
    use cryptocol::hash::MD4_Expanded;
    type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    let mut my_hash = MyMD4::new();
    let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    my_hash.digest_str(txt);
    println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    assert_eq!(my_hash.to_string(), "745B42127EC2479032923F2EE368FD92");
    println!("-------------------------------");
}