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
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
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
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of string slice.
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
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method digest_str().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "E396CE68E2BE1001BCBFD62B49E226C0");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method digest_str().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "719A1EB0F5077837BB408434B7AAD81E");
    /// ```
    #[inline]
    pub fn digest_str(&mut self, message: &str)
    {
        unimplemented!(); // Dummy code for documentation
    }
}