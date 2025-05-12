// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]

use std::ptr::{ copy_nonoverlapping, write_bytes };
use std::fmt::{ self, Debug, Display, Formatter };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign,
                BitXor, BitXorAssign, Not, Shl };

use crate::number::{ SmallUInt, LongUnion };

pub struct Keccak_Generic<const RATE: usize = 72, const PADDING: usize = 0,
        const ROUNDS: usize = 24, T = u64, const LFSR: u8 = 0b_0111_0001,
        const THETA_SUB: usize = 1, const THETA_ADD: usize = 1, const THETA_ROT: u32 = 1,
        const RHO_MUL_X: usize = 2, const RHO_MUL_Y: usize = 3, const RHO_T: u32 = 24,
        const PI_MUL_X: usize = 1, const PI_MUL_Y: usize = 3,
        const CHI_ADD_1: usize = 1, const CHI_ADD_2: usize = 2>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    // Dummy struct for documentation
    state: [[T; 5]; 5],
}

impl<const RATE: usize, const PADDING: usize, const ROUNDS: usize, T, const LFSR: u8,
    const THETA_SUB: usize, const THETA_ADD: usize, const THETA_ROT: u32,
    const RHO_MUL_X: usize, const RHO_MUL_Y: usize, const RHO_T: u32,
    const PI_MUL_X: usize, const PI_MUL_Y: usize,
    const CHI_ADD_1: usize, const CHI_ADD_2: usize>
Keccak_Generic<RATE, PADDING, ROUNDS, T, LFSR,
                THETA_SUB, THETA_ADD, THETA_ROT,
                RHO_MUL_X, RHO_MUL_Y, RHO_T,
                PI_MUL_X, PI_MUL_Y, CHI_ADD_1, CHI_ADD_2>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + Shl<Output = T>
{
    
    // pub fn new() -> Self
    /// Creates the new object of `Self`.
    /// 
    /// # Output
    /// A new object of `Self`.
    /// 
    /// # Example 1 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let hash = SHA3_512::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    /// 
    /// # Example 2 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let hash = SHAKE_256::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let hash = cSHAKE_128::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "00000000000000000000000000000000");
    /// ```
    /// 
    /// # Example 4 for KECCAK_384
    /// ```
    /// use cryptocol::hash::KECCAK_384;
    /// let hash = KECCAK_384::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    /// ```
    /// 
    /// # Example 5 for BIG_SHA3_1536
    /// ```
    /// use cryptocol::hash::BIG_SHA3_1536;
    /// let hash = BIG_SHA3_1536::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "00000000000000000000000000000000547969F9071B9AF02278D128944DD59C6BCCB273DBC1100F794F6488CB39D8EEB7953D954C8AC24A261368E226EA56166AA0B320613AAC9FD788A774ACBA3C71500157FE72A09D4F8C8198FF48495991D3DE92E4767FAACBB34AFB7786536E07DEF4A123AA97BC1BCFE2E34CDD60D15505B6DAA4FCF38CF9C206E86C18BE03AE31B1ADB2D0996CD729A4962E8B5EA592E3BBC024F2A0C9266A2005A25E82AE87583FE906E44469BDC2FC79C8A8B881F2");
    /// ```
    /// 
    /// # Example 6 for SMALL_KECCAK_384
    /// ```
    /// use cryptocol::hash::SMALL_KECCAK_384;
    /// let hash = SMALL_KECCAK_384::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0000000073309BF748B9DB9AC2563DABAFA463E1B027E3AC9BF40564EA67E3C85221FD7F8565B7B6FCF438DF69A3EE9F");
    /// ```
    /// 
    /// # Example 7 for SMALLER_SHAKE_128
    /// ```
    /// use cryptocol::hash::SMALLER_SHAKE_128;
    /// let hash = SMALLER_SHAKE_128::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "00000000000000000000000000000000");
    /// ```
    /// 
    /// # Example 8 for TINY_cSHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_cSHAKE_64;
    /// let hash = TINY_cSHAKE_64::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0000000000000000");
    /// ```
    #[inline]
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    /// Computes hash value.
    /// 
    /// # Features
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// - This method is the wrapper of the method `absorb()` for consistancy
    ///   with other hash functions.
    /// 
    /// # Arguments
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "065B40EFFE93C55937ACA0C23D7A35387E0FDCA478C49D13255A59F685A2A53C");
    /// ```
    /// 
    /// # Example 2 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// let mut hash = SHAKE_128::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "231FEC5DD3B64C278D1EC8BF8BFE4FF3");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "786CECCA21143CC45A532E526819F734");
    /// ```
    /// 
    /// # Example 4 for KECCAK_384
    /// ```
    /// use cryptocol::hash::KECCAK_384;
    /// let mut hash = KECCAK_384::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F8CF3D0877DA966AB6B96D98CD722103BB3E2477CA5DC9E8805541A99AAB5ECF1A8E6A885CC7E18FAEC4ED99CD759BCE");
    /// ```
    /// 
    /// # Example 5 for BIG_SHAKE_1536
    /// ```
    /// use cryptocol::hash::BIG_SHAKE_1536;
    /// let mut hash = BIG_SHAKE_1536::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// ```
    /// 
    /// # Example 6 for SMALL_SHA3_384
    /// ```
    /// use cryptocol::hash::SMALL_SHA3_384;
    /// let mut hash = SMALL_SHA3_384::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "780AE19CD221EA8DFEE27F5B446CC3FA75F2D689F7673EFC445F64F2F8ECB6E630FA150BA10B672D5DAA3C46ABDD3C37");
    /// ```
    /// 
    /// # Example 7 for SMALLER_KECCAK_128
    /// ```
    /// use cryptocol::hash::SMALLER_KECCAK_128;
    /// let mut hash = SMALLER_KECCAK_128::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "890BDD0CC5F02273BBD3CFBEF13484C1");
    /// ```
    /// 
    /// # Example 8 for TINY_SHA3_64
    /// ```
    /// use cryptocol::hash::TINY_SHA3_64;
    /// let mut hash = TINY_SHA3_64::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7804DAFDFCDB1CE0");
    /// ```
    #[inline]
    pub fn digest(&mut self, message: *const u8, length_in_bytes: u64)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_str(&mut self, message: &str)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str()`
    /// for consistancy with other hash functions.
    /// 
    /// # Argument
    /// - `message` is of type `&str`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@Keccak_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_224
    /// ```
    /// use cryptocol::hash::SHA3_224;
    /// let mut hash = SHA3_224::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BA8399261A38A097A69A072A9DE74FEAB248E5E2C93E622AC7E3381A");
    /// ```
    /// 
    /// # Example 2 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let mut hash = SHAKE_256::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B2FFABCAAE6C42F5FAD92B44035260ABD40157C8A37A0C3017EBA98441031F952A2E37E29A1588AD15A37584F672E3FEE0C0689E2F8DA44F144AAA23FCCDF623");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B2FFABCAAE6C42F5FAD92B44035260ABD40157C8A37A0C3017EBA98441031F952A2E37E29A1588AD15A37584F672E3FEE0C0689E2F8DA44F144AAA23FCCDF623");
    /// ```
    /// 
    /// # Example 4 for KECCAK_512
    /// ```
    /// use cryptocol::hash::KECCAK_512;
    /// let mut hash = KECCAK_512::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3D214CB0DBD422DD9CF9A21DAC50E2B6547D62BAA8546252B5A66FD3653EAF797D32A7FF804667D021AF9659F09B2AD5C983F266BE828D7BD831FD355C0FFA52");
    /// ```
    /// 
    /// # Example 5 for BIG_cSHAKE_1024
    /// ```
    /// use cryptocol::hash::BIG_cSHAKE_1024;
    /// let mut hash = BIG_cSHAKE_1024::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "9F51284305AE6E02A7A03E0A4BCA3D14AC4D9A43D7CDF4ADB9483282EE02E5A4FCBA0A07F008C29351FD60283F0ECEF3CBC5EB5E6C86BB380928EB7BC7D2D8213A73A3640636088ECBEC8322E932AE9DF461B4C25CF6706EACEFEF901408A969501F5A306FDE5A5BE505A1E504F2BBF9DEDAB44E02AE86D183D259CDF8CE72803D47E19EF4B33D8CCA1DD38616EF6907AC5B8F4B3F52756CF76BF397389B5F3D872A2EFE2AF89AD1A37FDE13F1C21A30F9BDFADC45C45B66C727E9F1E329DA63C9C57152F569FCD12ABF721185001600F8D262B62CF2ADA8D804232B68AFC3B4092A4B3FD685875F029989D48C669EF0020FEF2561AED9D14B3D268ECAC33CD2");
    /// ```
    /// 
    /// # Example 6 for SMALL_KECCAK_224
    /// ```
    /// use cryptocol::hash::SMALL_KECCAK_224;
    /// let mut hash = SMALL_KECCAK_224::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A574F12B85064E3ACA8ABB88F4859C3C9FBDE1F93C83ECD8AE550C578D24");
    /// ```
    /// 
    /// # Example 7 for SMALLER_cSHAKE_128
    /// ```
    /// use cryptocol::hash::SMALLER_cSHAKE_128;
    /// let mut hash = SMALLER_cSHAKE_128::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "43D99F951BAD90AD59938C03F84E3FE81FC712A0F13ECEBF8ED72DF673476EE6");
    /// ```
    /// 
    /// # Example 8 for TINY_KECCAK_64
    /// ```
    /// use cryptocol::hash::TINY_KECCAK_64;
    /// let mut hash = TINY_KECCAK_64::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "31CC3393C108D5C4");
    /// ```
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_string(&mut self, message: &String)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_string()`
    /// for consistancy with other hash functions.
    /// 
    /// # Argument
    /// - `message` is of the type `&String`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@Keccak_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@Keccak_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_384
    /// ```
    /// use cryptocol::hash::SHA3_384;
    /// let mut hash = SHA3_384::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "34721672060C3F72C8FFD207E6D7ABA63CAA7A5BFEE0A695C7A11C423E8B14A27A61A967E3BACD041C4449F127533247");
    /// ```
    /// 
    /// # Example 2 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// let mut hash = SHAKE_128::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "37E99A318DD958FB0EC077D77A08733192E890A7DA8BC39FBD04F64F49A9C8C0");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "37E99A318DD958FB0EC077D77A08733192E890A7DA8BC39FBD04F64F49A9C8C0");
    /// ```
    /// 
    /// # Example 4 for KECCAK_512
    /// ```
    /// use cryptocol::hash::KECCAK_256;
    /// let mut hash = KECCAK_256::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "0FD9EB73A653FDB9693C2D9028FA29BECD5F778C17115841777BEFC2451CC765");
    /// ```
    /// 
    /// # Example 5 for BIG_KECCAK_1536
    /// ```
    /// use cryptocol::hash::BIG_KECCAK_1536;
    /// let mut hash = BIG_KECCAK_1536::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "8E1BD7754AFA381F9BAEF2304A0EA7B4A0BCC8CEAC31DC649725452ABF0D0017554B28CA4EB8847A6CE264650068547C40748DC7774682110D5F4896796AA37A2E5925AF74F98CF85CFB340945B04BC79B2AF8353CCC84E76A218C6F0B34AB980BE9937C56BE81225CC6FE73C1F101C2980571A228903D76930A07FB22DA2C7323AC5B6AFCDA9BC16742F04A76C420C1358E462A1FE50F2341C03EEB4E07B7EEF2A4F1AE2CEA7FE51F812885A1297EFBBA1B92F678A2C9B951DCF0FF8FDBD8A0");
    /// ```
    /// 
    /// # Example 6 for SMALL_cSHAKE_128
    /// ```
    /// use cryptocol::hash::SMALL_cSHAKE_128;
    /// let mut hash = SMALL_cSHAKE_128::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "D9D7BD5F8EE9795A678519FEB1EEA98FBC476605C9AD6D84F67FBA470C18A3BE");
    /// ```
    /// 
    /// # Example 7 for SMALLER_SHA3_128
    /// ```
    /// use cryptocol::hash::SMALLER_SHA3_128;
    /// let mut hash = SMALLER_SHA3_128::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F0E533FEC8331BFEC34F1360ED90A80A");
    /// ```
    /// 
    /// # Example 8 for TINY_SHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_SHAKE_64;
    /// let mut hash = TINY_SHAKE_64::new();
    /// let txt = String::from("This is an example of the method digest_string().");
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "8D9972AC977AFEB67D374022892588C8");
    /// ```
    #[inline]
    pub fn digest_string(&mut self, message: &String)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_array<U, const N: usize>(&mut self, message: &[U; N])
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_array()`
    /// for consistancy with other hash functions.
    /// 
    /// # Argument
    /// - `message` is of the type `&[U; N]`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@MD4_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@MD4_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@MD4_Generic#method.digest_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@MD4_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");
    /// ```
    /// 
    /// # Example 2 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let mut hash = SHAKE_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");
    /// ```
    /// 
    /// # Example 4 for KECCAK_224
    /// ```
    /// use cryptocol::hash::KECCAK_224;
    /// let mut hash = KECCAK_224::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "84DC4C9F2C0B38A05C973A66B63EA7AEE8BBE1334E4C756AC6660717");
    /// ```
    /// 
    /// # Example 5 for BIG_SHA3_1024
    /// ```
    /// use cryptocol::hash::BIG_SHA3_1024;
    /// let mut hash = BIG_SHA3_1024::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "A99105325B0816D9D872CDB328F7E6C50EF3CE1C6C6B7FE10C7AA2416973195121349A2205711B7A29BF4FEBD6F654A0DAA664CC6528D02F4EE8E810973E88342AAA12876B40E79B69F717AE4D98916A16ADD5800772B70C9DD50B87E752AD595E398F5D327794A54DF2CB2C89C37A546260D76C356DF6FEBDAB21EED62941E0");
    /// ```
    /// 
    /// # Example 6 for SMALL_SHAKE_128
    /// ```
    /// use cryptocol::hash::SMALL_SHAKE_128;
    /// let mut hash = SMALL_SHAKE_128::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "AC39DB3C0CFD5A01F0289EB728BF157E0B312FCEDE39C1081E7A9211D316FCA7");
    /// ```
    /// 
    /// # Example 7 for SMALLER_KECCAK_128
    /// ```
    /// use cryptocol::hash::SMALLER_KECCAK_128;
    /// let mut hash = SMALLER_KECCAK_128::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "8F94C2115CEFFD6C4DFEF1CE1E036CC5");
    /// ```
    /// 
    /// # Example 8 for TINY_cSHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_cSHAKE_64;
    /// let mut hash = TINY_cSHAKE_64::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "30A8CD9FEB02319FF224968B7A885D15");
    /// ```
    #[inline]
    pub fn digest_array<U, const N: usize>(&mut self, message: &[U; N])
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_vec<U>(&mut self, message: &Vec<U>)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_array()`
    /// for consistancy with other hash functions.
    /// 
    /// # Argument
    /// - `message` is of the type `&Vec<U>`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@MD4_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string()](struct@MD4_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@MD4_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@MD4_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let mut hash = SHA3_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "2FA65DD00903E850AD14E00D13ACBE9C2CA2E7B140419B8C7EA2742900586B14");
    /// ```
    /// 
    /// # Example 2 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let mut hash = SHAKE_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");
    /// ```
    /// 
    /// # Example 3 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "BD401EE4EA04D047D732FE73F274AF0334185E3ADC82F6C761CF1722F6502F10EC5B0A58C861D503237BBFD99A1F6ECCAF1A2FC4A6C7CE4DC81563270BB10D8D");
    /// ```
    /// 
    /// # Example 4 for KECCAK_224
    /// ```
    /// use cryptocol::hash::KECCAK_224;
    /// let mut hash = KECCAK_224::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "84DC4C9F2C0B38A05C973A66B63EA7AEE8BBE1334E4C756AC6660717");
    /// ```
    /// 
    /// # Example 5 for BIG_SHA3_1024
    /// ```
    /// use cryptocol::hash::BIG_SHA3_1024;
    /// let mut hash = BIG_SHA3_1024::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "A99105325B0816D9D872CDB328F7E6C50EF3CE1C6C6B7FE10C7AA2416973195121349A2205711B7A29BF4FEBD6F654A0DAA664CC6528D02F4EE8E810973E88342AAA12876B40E79B69F717AE4D98916A16ADD5800772B70C9DD50B87E752AD595E398F5D327794A54DF2CB2C89C37A546260D76C356DF6FEBDAB21EED62941E0");
    /// ```
    /// 
    /// # Example 6 for SMALL_SHAKE_128
    /// ```
    /// use cryptocol::hash::SMALL_SHAKE_128;
    /// let mut hash = SMALL_SHAKE_128::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "AC39DB3C0CFD5A01F0289EB728BF157E0B312FCEDE39C1081E7A9211D316FCA7");
    /// ```
    /// 
    /// # Example 7 for SMALLER_KECCAK_128
    /// ```
    /// use cryptocol::hash::SMALLER_KECCAK_128;
    /// let mut hash = SMALLER_KECCAK_128::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "8F94C2115CEFFD6C4DFEF1CE1E036CC5");
    /// ```
    /// 
    /// # Example 8 for TINY_cSHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_cSHAKE_64;
    /// let mut hash = TINY_cSHAKE_64::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t\"{:?}\"\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "30A8CD9FEB02319FF224968B7A885D15");
    /// ```
    #[inline]
    pub fn digest_vec<U>(&mut self, message: &Vec<U>)
    where U: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_customized(&mut self, function_name: *const u8, function_name_length_in_bytes: u64, user_defined: *const u8, user_defined_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    /// Computes hash value.
    /// 
    /// # Features
    /// - This function has the generalized interface (pointer, `*const u8`)
    ///   so as to enable other functions to wrap this function with any
    ///   convenient interface for uses. So, this function is usually not called
    ///   directly in Rust. This function is provided to be called from other
    ///   programming languages such as C/C++.
    /// - This method is the wrapper of the method `absorb_customized()`.
    /// 
    /// # Arguments
    /// - `function_name` is pointer to const u8, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can you
    ///   `function_name` because NIST reserved `function_name` and
    ///   `function_name_length_in_bytes` only for cSHAKE. You can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `function_name_length_in_bytes` is the size of `function_name` in the
    ///   unit of bytes, and its data type is `u64`, but it is reserved
    ///   for NIST use. You are supposed to give `0` for
    ///   `function_name_length_in_bytes`. You may want to use
    ///   `user_defined_length_in_bytes` instead. However, for all the types other
    ///   than cSHAKE, you can you `function_name_length_in_bytes` because NIST
    ///   reserved `function_name` and `function_name_length_in_bytes` only for
    ///   cSHAKE. You can freely use this method for expanded versions of struct
    ///   `Keccak_Generic`.
    /// - `user_defined` is pointer to const u8, which contains a string for
    ///   your special use.
    /// - `user_defined_length_in_bytes` is the size of `user_defined` in the
    ///   unit of bytes, and its data type is `u64`.
    /// - `message` is pointer to const u8, which contains the actual date to
    ///   hash.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    ///   its data type is `u64`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    ///
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized("".as_ptr(), 0, user_defined.as_ptr(), user_defined.len() as u64,  txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "4C3793B9B1CBA98DA30F71F0ABEEB6DB7D5B35318F17E5445BAEC565FADCB003");
    /// ```
    ///
    /// # Example 2 for BIG_cSHAKE_1536
    /// ```
    /// use cryptocol::hash::BIG_cSHAKE_1536;
    /// let mut hash = BIG_cSHAKE_1536::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "60AC15376CFCC7DEF13C97238126DF635425972DEAF27D3212D107C663F2327EAB83B63BC007A7A049733BDC3783A76CAF38963B08B1C697EA622F5FADBE18495930A9F0F8EFD219811156ECD4712797BBCCD0EE92168564FD3C09965D8A6411D0328DFD1A7B3E446C63CE6A7220855A447BAC6C4D7665683D23E29EC209B9F72A779A5F84F4678605D79AD5A4EA09282283EBEDF37781F6C7D428FAEC4E2F640D14F22A9204252F6DE164837E0AF540661B3FA42A1C56FD2A95FC38C4838C90695C2D90F6819B8B7AEA4AC739D270EA07504ED62FFBAF426C2386534FE95F9348D58BC7454BA4802B5984790163F2B12ED2F0AC00CAAECD352344BC08CC1487183ADA924A1064FCB4BA59D82556F322A6A33CB39921641A7232D6B852039FC2B9651FFBD13E6CBA5F74714DC06965232A1B64F1E715CEF9932070EF746A1D43A142DF9AFC75357AFDF9022BF9332C688423CC7CFCBD9E82D83C6CED8B24833294AADD37D3438735D391B0648705E094553E3194E8402FFEF4303AD0372842EB");
    /// ```
    ///
    /// # Example 3 for SMALL_cSHAKE_256
    /// ```
    /// use cryptocol::hash::SMALL_cSHAKE_256;
    /// let mut hash = SMALL_cSHAKE_256::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "6C3E069C821A0526D5FF2EEB44A0B04A824CE6211A4664194C982E5A9EAC2F700FB684B6D0BA0B8E7D357709164C1265736C13C67E6AB4728CA57677F6949501");
    /// ```
    ///
    /// # Example 4 for SMALLER_KECCAK_128
    /// ```
    /// use cryptocol::hash::SMALLER_KECCAK_128;
    /// let mut hash = SMALLER_KECCAK_128::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E11D8A5C39AF9ECD2AF3CFBEC57F46DC");
    /// ```
    ///
    /// # Example 5 for TINY_SHA3_64
    /// ```
    /// use cryptocol::hash::TINY_SHA3_64;
    /// let mut hash = TINY_SHA3_64::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "EC07E55C6AD49B81");
    /// ```
    ///
    /// # Example 6 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let mut hash = SHA3_512::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "183F74D0A47CE0B3B533A903AC907FEED184D4E39F212F27EE0BF6B9E4E1B7CEAF105A165A6C9CC28DA27261194667B578B4B0B7626E1554340A297B133181C1");
    /// ```
    ///
    /// # Example 7 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// let mut hash = SHAKE_128::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A744B963C7F2CBAD52CBDBF2090173C4593D93B854581F5B623B060CEF4E013A");
    /// ```
    ///
    /// # Example 8 for KECCAK_224
    /// ```
    /// use cryptocol::hash::KECCAK_224;
    /// let mut hash = KECCAK_224::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_customized().";
    /// hash.digest_customized(function_name.as_ptr(), function_name.len() as u64, user_defined.as_ptr(), user_defined.len() as u64, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "F9D24FB9D6F617C993B9F155457683E0D4B26F7FC646C00A7E349FFB");
    /// ```
    #[inline]
    pub fn digest_customized(&mut self, function_name: *const u8, function_name_length_in_bytes: u64, user_defined: *const u8, user_defined_length_in_bytes: u64, message: *const u8, length_in_bytes: u64)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str_customized()`
    /// .
    /// # Argument
    /// - `function_name` is of type `&str`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can you
    ///   `function_name` because NIST reserved `function_name` and
    ///   `function_name_length_in_bytes` only for cSHAKE. You can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&str`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&str`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of the content of String
    ///   object, you are highly recommended to use the method
    ///   [digest_string_customized()](struct@Keccak_Generic#method.digest_string_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for cSHAKE_256
    /// ```
    /// use cryptocol::hash::cSHAKE_256;
    /// let mut hash = cSHAKE_256::new();
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized("", user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3BB260278648858A59A25EE45AEA4E17A8DD7FAF51E32AEF4D3EA11739E38D4C9D22B7AE394D79E2A88BD2EFA4385E490836D0C6ED9D9087A3229F17F5E50EC9");
    /// ```
    /// 
    /// # Example 2 for BIG_cSHAKE_768
    /// ```
    /// use cryptocol::hash::BIG_cSHAKE_768;
    /// let mut hash = BIG_cSHAKE_768::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A0EFC73B6175215727A67CBD4B873579DBA7B5E3E5065B394449A7C31538C738361018179A3EFFAABB7BD1E50CFB02D6AEAF809EF51775126FAC1E35EB6CE844FCE1EAFB577D153D2100AFC4FBA51A3E1C418A9A337ED1BD68D13C6AEFE362D402C7A24F159BEF0610666038DE05C630F082E80F5C62FD865B523AB205E01F2E2D5A293CCFF27000D3D54800F9541CFA402FB2F77D23F0F3FC19118A8E0D93E93C7DFA74F94F280A367C2A15FE3FC471D68A544E470B6837AF381FCF6D3AA8BA");
    /// ```
    /// 
    /// # Example 3 for SMALL_cSHAKE_128
    /// ```
    /// use cryptocol::hash::SMALL_cSHAKE_128;
    /// let mut hash = SMALL_cSHAKE_128::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "7392046EECA855497C107E6C5510D3BA9FAEA15F363CA336115846C1E2E9DBD6");
    /// ```
    /// 
    /// # Example 4 for SMALLER_cSHAKE_128
    /// ```
    /// use cryptocol::hash::SMALLER_cSHAKE_128;
    /// let mut hash = SMALLER_cSHAKE_128::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "984227B7084637C4EFEFB795790C4792BA13C3288CE000F7FD84DF804FEC8F10");
    /// ```
    /// 
    /// # Example 5 for TINY_cSHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_cSHAKE_64;
    /// let mut hash = TINY_cSHAKE_64::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "D49E308AD8F2DA82935F48CD2E9216F0");
    /// ```
    /// 
    /// # Example 6 for SHA3_224
    /// ```
    /// use cryptocol::hash::SHA3_224;
    /// let mut hash = SHA3_224::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "EC429BB8857A6273E108EAEFA9435867D92C442CCF4B5309795068E9");
    /// ```
    /// 
    /// # Example 7 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let mut hash = SHAKE_256::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "C072CDE8B6EA6F63BD756F6BE59A7C44CB51DDBA10E96C19C79FB1286AED9DDB4D3CD5F2CC94F3D7C0505F1888805D32C0CD4FBE10E311D72436576E485DE445");
    /// ```
    /// 
    /// # Example 8 for KECCAK_256
    /// ```
    /// use cryptocol::hash::KECCAK_256;
    /// let mut hash = KECCAK_256::new();
    /// let function_name = "Reserved for NIST";
    /// let user_defined = "on my own purpose";
    /// let txt = "This is an example of the method digest_str_customized().";
    /// hash.digest_str_customized(function_name, user_defined, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "067791E671F1493BF93A2E1EAAD460E0FDF2176EA744FC433568C013A9F299C5");
    /// ```
    #[inline]
    pub fn digest_str_customized(&mut self, function_name: &str, user_defined: &str, message: &str)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    /// Computes hash value.
    /// 
    /// # Features
    /// This method is the wrapper of the method `absorb_str_customized()`
    /// .
    /// # Argument
    /// - `function_name` is of type `&String`, which contains function name,
    ///   but it is reserved for NIST use. You are supposed to give null string8
    ///   for `function name`. You may want to use `user_defined` instead.
    ///   However, for all the types other than cSHAKE, you can you
    ///   `function_name` because NIST reserved `function_name` and
    ///   `function_name_length_in_bytes` only for cSHAKE. You can freely use
    ///   this method for expanded versions of struct `Keccak_Generic`.
    /// - `user_defined` is of type `&String`, which contains a string for
    ///   your special use.
    /// - `message` is of type `&String`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str_customized()](struct@Keccak_Generic#method.digest_str_customized)
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array_customized()](struct@Keccak_Generic#method.digest_array_customized)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec_customized()](struct@Keccak_Generic#method.digest_vec_customized)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@Keccak_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for cSHAKE_128
    /// ```
    /// use cryptocol::hash::cSHAKE_128;
    /// let mut hash = cSHAKE_128::new();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&"".to_string(), &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "411E09A6E5CA61E99546226582C89FE0D6C3A57992173476C95F8BA1089EDF6D");
    /// ```
    ///
    /// # Example 2 for BIG_cSHAKE_512
    /// ```
    /// use cryptocol::hash::BIG_cSHAKE_512;
    /// let mut hash = BIG_cSHAKE_512::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "21CAADA088F7712239A4AE7089C625319C84839335C4E9E199BA6DB522DD473A57B15A721C37284CADCFD25C74E123B49BB5C67EFDF9CD4FF13E4E4F25A9EF7CDC7187DE203B559D1442444FBD7824BD6C72F8750CAFC70ECC5989446B08B8C9180B7BD4997B028F5908431A75B4B89A98F18FA365AB3B58A10009F7EB0A2A2E");
    /// ```
    ///
    /// # Example 3 for SMALL_cSHAKE_224
    /// ```
    /// use cryptocol::hash::SMALL_cSHAKE_224;
    /// let mut hash = SMALL_cSHAKE_224::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BE791B9B08B2080EDEDD1F1E72A59B5EDC33E973CDC903C1EB4D967451A9A4A92EFCA80B46AB1449D5D1B8A67C0E23CD0FCCE4BFADC4F16CA086C726AA");
    /// ```
    ///
    /// # Example 4 for SMALLER_cSHAKE_128
    /// ```
    /// use cryptocol::hash::SMALLER_cSHAKE_128;
    /// let mut hash = SMALLER_cSHAKE_128::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3AC9E1D3453345DDDB0436C3CB699538075651560710FF5C51AD6462D9FBC114");
    /// ```
    ///
    /// # Example 5 for TINY_cSHAKE_64
    /// ```
    /// use cryptocol::hash::TINY_cSHAKE_64;
    /// let mut hash = TINY_cSHAKE_64::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3E027E497CE48B64B7B776124CAF7929");
    /// ```
    ///
    /// # Example 6 for SHA3_384
    /// ```
    /// use cryptocol::hash::SHA3_384;
    /// let mut hash = SHA3_384::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "370207DA06E2E85AAEC6C10E15DE9F92F8954AB036A0D10C48DB6A8D2FB5238EC209B2016BDAB94CBFE53FF3ECDF1178");
    /// ```
    ///
    /// # Example 7 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// let mut hash = SHAKE_128::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "BA2EF98EA6944C201267441004368743340D1B32A895C1EF5364F00E0A7A8707");
    /// ```
    ///
    /// # Example 8 for KECCAK_512
    /// ```
    /// use cryptocol::hash::KECCAK_512;
    /// let mut hash = KECCAK_512::new();
    /// let function_name = "Reserved for NIST".to_string();
    /// let user_defined = "on my own purpose".to_string();
    /// let txt = String::from("This is an example of the method digest_string_customized().");
    /// hash.digest_string_customized(&function_name, &user_defined, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "C6FCC447C8ADCB04AA7229D3884A19EC6D5C44E96AA0AB62651CD0A8D71EFA2C24317F3DFFB3ABE3CA27D8686382C7C094DF464820671C4C841E04AB3A6F2CDB");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/hash_sha3/struct.Keccak_Generic.html#method.digest_string_customized)
    #[inline]
    pub fn digest_string_customized(&mut self, function_name: &String, user_defined: &String, message: &String)
    {
        unimplemented!(); // Dummy code for documentation
    }
}