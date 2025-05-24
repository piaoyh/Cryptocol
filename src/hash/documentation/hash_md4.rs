// Copyright 2024 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]


use std::fmt::{ self, Debug, Display, Formatter };

use crate::number::{ SmallUInt, IntUnion };

/// md4.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to hash_md4.rs.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct MD4_Generic<const N: usize = 4,
        const H0: u32 = 0x67452301, const H1: u32 = 0xefcdab89,
        const H2: u32 = 0x98badcfe, const H3: u32 = 0x10325476, const ROUND: usize = 48,
        const K0: u32 = 0x00000000, const K1: u32 = 0x5A827999, const K2: u32 = 0x6ED9EBA1,
        const R00: u32 = 3, const R01: u32 = 7, const R02: u32 = 11, const R03: u32 = 19,
        const R10: u32 = 3, const R11: u32 = 5, const R12: u32 = 9, const R13: u32 = 13, 
        const R20: u32 = 3, const R21: u32 = 9, const R22: u32 = 11, const R23: u32 = 15>
{
    // Dummy struct for documentation
    hash_code: [IntUnion; 4],
}

impl<const N: usize, const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32>
MD4_Generic<N, H0, H1, H2, H3, ROUND, K0, K1, K2, 
            R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23>
{
    // pub fn new() -> Self
    /// Constructs a new `MD4` object or a new MD4-based hash object.
    /// 
    /// # Output
    /// A new object of `MD4` or a new MD4-based hash object.
    /// 
    /// # Initialization
    /// All the attributes of the constructed object, which is initial hash
    /// value, will be initialized with `0123456789ABCDEFFEDCBA9876543210` for
    /// MD4. However, if you use your own MD4-expanded version, it will be
    /// initialized with your special values H0 ~ H3.
    /// 
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let hash = MD4::new();
    /// println!("Hash =\t{}", hash);
    /// assert_eq!(hash.to_string(), "0123456789ABCDEFFEDCBA9876543210");
    /// ```
    /// 
    /// # Exmaple 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let my_hash = MyMD4::new();
    /// println!("Hash =\t{}", my_hash);
    /// assert_eq!(my_hash.to_string(), "111111114444444488888888FFFFFFFF");
    /// ```
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
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@MD4_Generic#method.digest_vec)
    ///   rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method digest().";
    /// hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A18836F660C3C66B8CBEE4BD24FEFFA9");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method digest().";
    /// my_hash.digest(txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "B2F465006DCBA147BCE76D7EB8B564E1");
    /// ```
    #[allow(unused_variables)]
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
    ///   [digest_string()](struct@MD4_Generic#method.digest_string)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@MD4_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@MD4_Generic#method.digest_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@MD4_Generic#method.digest) rather than this method.
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
    #[allow(unused_variables)]
    pub fn digest_str(&mut self, message: &str)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_string(&mut self, message: &String)
    /// Computes hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of String object.
    /// 
    /// # Argument
    /// - message is `&String`.
    /// 
    /// # Counterpart Methods
    /// - If you want to compute of the hash value of a string slice,
    ///   you are highly recommended to use the method
    ///   [digest_str()](struct@MD4_Generic#method.digest_str)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Array
    ///   object, you are highly recommended to use the method
    ///   [digest_array()](struct@MD4_Generic#method.digest_array)
    ///   rather than this method.
    /// - If you want to compute of the hash value of the content of Vec
    ///   object, you are highly recommended to use the method
    ///   [digest_vec()](struct@MD4_Generic#method.digest_vec)
    ///   rather than this method.
    /// - If you want to use this method from other programming languages such
    ///   as C/C++, you are highly recommended to use the method
    ///   [digest()](struct@MD4_Generic#method.digest) rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "DF23C7808B2B158C5E2D8C9FE1FF2ECC");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method digest_string().".to_string();
    /// my_hash.digest_string(&txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "FD42F7479ED133619D877BB1E6C8A084");
    /// ```
    #[allow(unused_variables)]
    pub fn digest_string(&mut self, message: &String)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_array<T, const M: usize>(&mut self, message: &[T; M])
    /// Computes hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of Array object.
    /// 
    /// # Argument
    /// - message is `&[T; M]`.
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
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_array(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    /// ```
    #[allow(unused_variables)]
    pub fn digest_array<T, const M: usize>(&mut self, message: &[T; M])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    /// Computes hash value.
    /// 
    /// # Features
    /// This function is a wrapping function of `digest()`.
    /// This function computes hash value of the content of Vec object.
    /// 
    /// # Argument
    /// - message is `&Vec<T>`.
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
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "31489CF63B7FC170E9046F0176A60B39");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.digest_vec(&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "3011AFFDE0C322C2CCEE632FE39AF16D");
    /// ```
    #[allow(unused_variables)]
    pub fn digest_vec<T>(&mut self, message: &Vec<T>)
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ruminate(&mut self, n: usize, message: *const u8, length_in_bytes: u64)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is pointer to const u8.
    /// - `length_in_bytes` is the size of message in the unit of bytes, and
    /// data type is `u64`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*const u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "23EAC3CEE64E4266EEDFE2D6AB255B9F");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut hash = MyMD4::new();
    /// let txt = "This is an example of the method ruminate().";
    /// hash.ruminate(2, txt.as_ptr(), txt.len() as u64);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "A1608F7E4052E267B3233862FD5C1C41");
    /// ```
    #[allow(unused_variables)]
    pub fn ruminate(&mut self, n: usize, message: *const u8, length_in_bytes: u64)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ruminate_str(&mut self, n: usize, message: &str)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&str`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of string slice.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "B19769E514631D59FD257C4AD667BD9D");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method ruminate_str().";
    /// my_hash.ruminate_str(3, txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "534F1EC44D4B2CEF12B7A9A81941D9A8");
    /// ```
    #[allow(unused_variables)]
    pub fn ruminate_str(&mut self, n: usize, message: &str)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ruminate_string(&mut self, n: usize, message: &String)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&String`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of String object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "71D3AB5636348DB24A7AE302E7E6C05A");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method ruminate_string().".to_string();
    /// my_hash.ruminate_string(2, &txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "EFB3B63FC1DBF3852F469D4EA0E8D517");
    /// ```
    #[allow(unused_variables)]
    pub fn ruminate_string(&mut self, n: usize, message: &String)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ruminate_array<T, const M: usize>(&mut self, n: usize, message: &[T; M])
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&[T; M]`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of Array object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "810F75A7BD28179BA2D4604A3092FBC8");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let data = [ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_array(5,&data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "27F598D17E6DFBA0A0713F3262D34FFC");
    /// ```
    #[allow(unused_variables)]
    pub fn ruminate_array<T, const M: usize>(&mut self, n: usize, message: &[T; M])
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ruminate_vec<T>(&mut self, n: usize, message: &Vec<T>)
    /// Computes a hash value of `message`, and then computes a new hash value
    /// of the hash value of the message, and then computes a hash value of the
    /// previous hash value, and then ... `n` times repeatedly.
    /// 
    /// # Arguments
    /// - `n` is the number of repetition of digestion
    /// - `message` is `&Vec<T>`.
    /// 
    /// # Origin
    /// Double hashing is invented by Ferguson and Schneier in their book
    /// Practical Cryptography to countermeasure against length extension
    /// attacks. Plus, Bitcoin uses double hashing.
    /// This is generalized version of it.
    /// 
    /// # Features
    /// This function is a wrapping function of `ruminate()`.
    /// This function computes hash value of the content of Vec object.
    /// 
    /// # Security Issue
    /// The author doubts that the double hashing is securer than normal
    /// hashing. The double hashing will be as secure as the normal hashing
    /// at most because birthday paradox applies twice for the double hashing
    /// though the size of the domain is the same size of the codomain for
    /// second hashing of the double hashing, while the birthday paradox
    /// applies only once for the normal hashing.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, hash);
    /// assert_eq!(hash.to_string(), "B3E296760B88B44613DB03D72CE59917");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let data = vec![ 0x67452301_u32.to_le(), 0xefcdab89_u32.to_le(), 0x98badcfe_u32.to_le(), 0x10325476_u32.to_le() ];
    /// my_hash.ruminate_vec(2, &data);
    /// println!("Msg =\t{:?}\nHash =\t{}", data, my_hash);
    /// assert_eq!(my_hash.to_string(), "AFC96A14952E9FB9D6D7C7A1FD3D4C2E");
    /// ```
    #[allow(unused_variables)]
    pub fn ruminate_vec<T>(&mut self, n: usize, message: &Vec<T>)
    where T: SmallUInt + Copy + Clone
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    /// Gives a hash value to the place where `hash_value` points to.
    /// 
    /// # Features
    /// This function has the generalized interface (pointer, `*mut u8`)
    /// so as to enable other functions to wrap this function with any
    /// convenient interface for uses. So, this function is usually not called
    /// directly in Rust. This function is provided to be called from other
    /// programming languages such as C/C++.
    /// 
    /// # Arguments
    /// - `hash_value` is the pointer to the place to hold the result hash value.
    /// - `length` is the size of the place that `hash_value` points to. 
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@MD4_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD4_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD4_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 16];
    /// hash.digest_str(txt);
    /// hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[A7, AD, DF, 36, A2, 43, 97, D1, 6D, 3C, 99, 78, A6, D5, 6E, 74]");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method get_hash_value().";
    /// let hash_value = [0_u8; 16];
    /// my_hash.digest_str(txt);
    /// my_hash.get_hash_value(hash_value.as_ptr() as *mut u8, hash_value.len());
    /// println!("Msg =\t\"{}\"\nHash =\t{:02X?}", txt, hash_value);
    /// assert_eq!(format!("{:02X?}", hash_value), "[02, 43, 79, C6, 08, F1, CA, 30, C0, 75, 5C, 6C, 07, AD, 76, 72]");
    /// ```
    #[allow(unused_variables)]
    pub fn get_hash_value(&self, hash_value: *mut u8, length: usize)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_hash_value_in_string(&self) -> String
    /// Returns a hash value in the form of String object.
    /// 
    /// # Output
    /// It returns String object.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD4_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD4_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD4_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.get_hash_value_in_string());
    /// assert_eq!(hash.get_hash_value_in_string(), "FA48527AD8257A371E70AA9473D425D6");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.get_hash_value_in_string());
    /// assert_eq!(my_hash.get_hash_value_in_string(), "626192ACD80D62D8966ACE89AE439E76");
    /// ```
    #[allow(unused_variables)]
    pub fn get_hash_value_in_string(&self) -> String
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_hash_value_in_array(&self) -> [u32; N]
    /// Returns a hash value in the form of array object.
    /// 
    /// # Output
    /// It returns [u32; N].
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in any form of Array object,
    /// you are highly recommended to use the method
    /// [put_hash_value_in_array()](struct@MD4_Generic#method.put_hash_value_in_array)
    /// rather than this method.
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_string()](struct@MD4_Generic#method.get_hash_value_in_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of Vec object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_vec()](struct@MD4_Generic#method.get_hash_value_in_vec)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD4_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[832C724B, 4A73A717, 5EA679B8, E991D13B]");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_array().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[2CFB0798, 77AA2A27, 602B457E, AD3B964C]");
    /// ```
    #[allow(unused_variables)]
    pub fn get_hash_value_in_array(&self) -> [u32; N]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_hash_value_in_vec(&self) -> Vec<u32>
    /// Returns a hash value in the form of Vec object.
    /// 
    /// # Output
    /// It returns `Vec<u32>`.
    /// 
    /// # Counterpart Methods
    /// - If you want to get the hash value in the form of String object,
    /// you are highly recommended to use the method
    /// [get_hash_value_string()](struct@MD4_Generic#method.get_hash_value_string)
    /// rather than this method.
    /// - If you want to get the hash value in the form of array object,
    /// you are highly recommended to use the method
    /// [get_hash_value_in_array()](struct@MD4_Generic#method.get_hash_value_in_array)
    /// rather than this method.
    /// - If you want to use this method from other programming languages such
    /// as C/C++, you are highly recommended to use the method
    /// [get_hash_value()](struct@MD4_Generic#method.get_hash_value)
    /// rather than this method.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_vec()), "[EE74475E, ECA09C8F, 038A89A3, 9B2A6C4F]");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method get_hash_value_in_vec().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_vec());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_vec()), "[440664DA, 49687C74, C0536C83, 192830D8]");
    /// ```
    #[allow(unused_variables)]
    pub fn get_hash_value_in_vec(&self) -> Vec<u32>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn put_hash_value_in_array<T, const M: usize>(&self, out: &mut [T; M])
    /// Puts a hash value in the form of array object.
    /// 
    /// # Argument
    /// `out` is the array [T; M] which is the place to put the hash value.
    /// 
    /// # Features
    /// If `M * mem::size_of::<T>()` > `mem::size_of::<u32>() * N`,
    /// it pass the output as the amount of `mem::size_of::<u32>() * N`.
    ///
    /// # Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 4];
    /// hash.digest_str(txt);
    /// hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[147DD795, C34F9C9D, 80B94C86, FB922262]");
    /// ```
    /// 
    /// # Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "This is an example of the method put_hash_value_in_array().";
    /// let mut hash_code = [0_u32; 4];
    /// my_hash.digest_str(txt);
    /// my_hash.put_hash_value_in_array(&mut hash_code);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash_code);
    /// assert_eq!(format!("{:08X?}", hash_code), "[1411D15D, 37BBE0DF, 1EAF8DA5, AC822C42]");
    /// ```
    #[allow(unused_variables)]
    pub fn put_hash_value_in_array<T, const M: usize>(&self, out: &mut [T; M])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn tangle(&mut self, tangling: u64)
    /// Tangles the hash value
    /// 
    /// # Argument
    /// u32 constant to tangle the hash value
    /// 
    /// # Features
    /// It is for using this struct as random number generator.
    /// 
    /// Example 1 for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "TANGLING";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[BC65D6E1, F0F37B4E, 2F404331, A8F25E2A]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[CE1E07A3, F3373D70, 95A8F098, 9BC7894E]");
    /// hash.tangle(1);
    /// println!("Hash =\t{:08X?}", hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", hash.get_hash_value_in_array()), "[5B9A2D14, 64888002, 15282E23, E5B2F4BD]");
    /// ```
    /// 
    /// Example 2 for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "TANGLING";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{:08X?}", txt, my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[C4377D49, 05FD9A1F, 3DA4E254, ACF22116]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[8CB0AF83, F75E073C, 77C5BF6C, EDFE1D51]");
    /// my_hash.tangle(1);
    /// println!("Hash =\t{:08X?}", my_hash.get_hash_value_in_array());
    /// assert_eq!(format!("{:08X?}", my_hash.get_hash_value_in_array()), "[A5C900D1, 388193FA, B2C0ED53, 4DE71DDE]");
    /// ```
    #[allow(unused_variables)]
    pub fn tangle(&mut self, tangling: u64)
    {
        unimplemented!(); // Dummy code for documentation
    }
}


impl<const N: usize,
    const H0: u32, const H1: u32, const H2: u32, const H3: u32,
    const ROUND: usize, const K0: u32, const K1: u32, const K2: u32,
    const R00: u32, const R01: u32, const R02: u32, const R03: u32,
    const R10: u32, const R11: u32, const R12: u32, const R13: u32,
    const R20: u32, const R21: u32, const R22: u32, const R23: u32>
Display for MD4_Generic<N, H0, H1, H2, H3, ROUND, K0, K1, K2, 
                        R00, R01, R02, R03, R10, R11, R12, R13, R20, R21, R22, R23>
{
    /// Formats the value using the given formatter.
    /// You will hardly use this method directly.
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()`, and you can also print the MD4
    /// object in the macro `println!()` directly for example.
    /// `f` is a buffer, this method must write the formatted string into it.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Example 1 for the method to_string() for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash.to_string());
    /// assert_eq!(hash.to_string(), "E2244B71E17D5BD7E1CCEB58C8F8C82E");
    /// ```
    /// 
    /// # Example 2 for the method to_string() for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "Display::fmt() automagically implement to_string().";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash.to_string());
    /// assert_eq!(my_hash.to_string(), "6B0D8F0CE90782E5FF88EE57B5DC5AF1");
    /// ```
    /// 
    /// # Example 3 for the use in the macro println!() for MD4
    /// ```
    /// use cryptocol::hash::MD4;
    /// let mut hash = MD4::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, hash);
    /// assert_eq!(hash.to_string(), "3C607B5548C155DCF4E84C7A6C21D349");
    /// ```
    /// 
    /// # Example 4 for the use in the macro println!() for MD4_Expanded
    /// ```
    /// use cryptocol::hash::MD4_Expanded;
    /// type MyMD4 = MD4_Expanded<4, 0x1111_1111, 0x4444_4444, 0x8888_8888, 0xffff_ffff, 96>;
    /// let mut my_hash = MyMD4::new();
    /// let txt = "Display::fmt() enables the object to be printed in the macro println!() directly for example.";
    /// my_hash.digest_str(txt);
    /// println!("Msg =\t\"{}\"\nHash =\t{}", txt, my_hash);
    /// assert_eq!(my_hash.to_string(), "745B42127EC2479032923F2EE368FD92");
    /// ```
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        unimplemented!(); // Dummy code for documentation
    }
}
