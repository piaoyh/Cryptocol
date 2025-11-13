// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(missing_docs)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// #![warn(rustdoc::missing_doc_code_examples)]


use crate::number::TraitsBigUInt;


/// Random.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to random_ranodm.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
#[allow(non_camel_case_types)]
pub struct Random_Generic
{
    // Dummy struct for documentation
}

/// Random.rs may be too big
/// because of documentation and plenty of examples.
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to random_ranodm.rs. And, most of generic parameters
/// are omitted. It is not actual code but dummy code for compilation!!!
impl Random_Generic
{
    // pub fn random_u8(&mut self) -> u8
    /// Generates random numbers of type `u8`.
    /// 
    /// # Output
    /// It returns a random number of type `u8`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_u8()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u8()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u8()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u8()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_256) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u8()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u8()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u8()); }
    /// ```
    pub fn random_u8(&mut self) -> u8
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_u16(&mut self) -> u16
    /// Generates random numbers of type `u16`.
    /// 
    /// # Output
    /// It returns a random number of type `u16`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_u16()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u16()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u16()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u16()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u16()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u16()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u16()); }
    /// ```
    pub fn random_u16(&mut self) -> u16
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_u32(&mut self) -> u32
    /// Generates random numbers of type `u32`.
    /// 
    /// # Output
    /// It returns a random number of type `u32`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_u32()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u32()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u32()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u32()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u32()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u32()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u32()); }
    /// ```
    pub fn random_u32(&mut self) -> u32
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_u64(&mut self) -> u64
    /// Generates random numbers of type `u64`.
    /// 
    /// # Output
    /// It returns a random number of type `u64`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_u64()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u64()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u64()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u64()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u64()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u64()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u64()); }
    /// ```
    pub fn random_u64(&mut self) -> u64
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_u128(&mut self) -> u128
    /// Generates random numbers of type `u128`.
    /// 
    /// # Output
    /// It returns a random number of type `u128`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_u128()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_u128()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_u128()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_u128()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_SHA2_512) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_u128()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_u128()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_u128()); }
    /// ```
    pub fn random_u128(&mut self) -> u128
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_usize(&mut self) -> usize
    /// Generates random numbers of type `usize`.
    /// 
    /// # Output
    /// It returns a random number of type `usize`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_usize()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_usize()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_usize()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_usize()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_256) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_SHA2_512) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_usize()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_usize()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_usize()); }
    /// ```
    #[inline]
    pub fn random_usize(&mut self) -> usize
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_uint<T>(&mut self) -> T
    /// Generates random numbers of type `T`.
    /// 
    /// # Output
    /// It returns a random number of type `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random) = {}", i, rand.random_uint::<u8>()); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any) = {}", i, any.random_uint::<u16>()); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_BIG_KECCAK_1024) = {}", i, rand.random_uint::<u32>()); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA3_512) = {}", i, rand.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, rand.random_uint::<u128>()); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_256) = {}", i, any.random_uint::<usize>()); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHAKE_128) = {}", i, any.random_uint::<u16>()); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA3_512) = {}", i, any.random_uint::<u32>()); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA2_512) = {}", i, any.random_uint::<u128>()); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_SHA2_512) = {}", i, any.random_uint::<u8>()); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA1) = {}", i, slapdash.random_uint::<usize>()); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_SHA0) = {}", i, slapdash.random_uint::<u32>()); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD5) = {}", i, slapdash.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_MD4) = {}", i, slapdash.random_uint::<u128>()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Random number (Random_Rijndael) = {}", i, rand.random_uint::<u8>()); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Rijndael) = {}", i, any.random_uint::<u16>()); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_DES) = {}", i, slapdash.random_uint::<usize>()); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash_Num_C) = {}", i, slapdash.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// for i in 0..10
    ///     { println!("{} Slapdash number (Slapdash) = {}", i, slapdash.random_uint::<u128>()); }
    /// ```
    pub fn random_uint<T>(&mut self) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`
    /// wrapped by enum `Some` of `Option`.
    /// 
    /// # Features
    /// If `ceiling` is `0`, it returns `None`. Otherwise, it returns a
    /// random number of type `T` wrapped by enum `Some` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// if let Some(num) = rand.random_under_uint(12_u8)
    ///     { println!("Random number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// if let Some(num) = any.random_under_uint(1234_u16)
    ///     { println!("Any number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// if let Some(num) = rand.random_under_uint(12345678_u32)
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// if let Some(num) = rand.random_under_uint(12345678901234567890_u128)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// if let Some(num) = any.random_under_uint(1234_usize)
    ///     { println!("Any number usize = {}", num); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// if let Some(num) = any.random_under_uint(0_usize)
    ///     { println!("Any number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned number under 0!"); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// if let Some(num) = any.random_under_uint(12_u8)
    ///     { println!("Any number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// if let Some(num) = any.random_under_uint(1234_u16)
    ///     { println!("Any number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// if let Some(num) = any.random_under_uint(12345678_u32)
    ///     { println!("Any number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// if let Some(num) = any.random_under_uint(1234567890123456_u64)
    ///     { println!("Any number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// if let Some(num) = slapdash.random_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// if let Some(num) = slapdash.random_under_uint(1234_usize)
    ///     { println!("Slapdash number usize = {}", num); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// if let Some(num) = slapdash.random_under_uint(0_u64)
    ///     { println!("Slapdash number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned number under 0!"); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// if let Some(num) = slapdash.random_under_uint(12_u8)
    ///     { println!("Slapdash number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// if let Some(num) = rand.random_under_uint(1234_u16)
    ///     { println!("Random number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// if let Some(num) = any.random_under_uint(12345678_u32)
    ///     { println!("Any number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// if let Some(num) = slapdash.random_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// if let Some(num) = slapdash.random_under_uint(0_u32)
    ///     { println!("Slapdash number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned number under 0!"); }
    /// ```
    #[inline]
    pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`.
    /// 
    /// # Panics
    /// If `ceiling` is `0`, it will panic.
    /// 
    /// # Caution
    /// Use only when `ceiling` is not `0`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let num = rand.random_under_uint_(12_u8);
    /// println!("Random number u8 = {}", num);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let num = any.random_under_uint_(1234_u16);
    /// println!("Random number u16 = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let num = rand.random_under_uint_(12345678_u32);
    /// println!("Random number u32 = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let num = rand.random_under_uint_(1234567890123456_u64);
    /// println!("Random number u64 = {}", num);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let num = rand.random_under_uint_(12345678901234567890_u128);
    /// println!("Random number u128 = {}", num);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let num = any.random_under_uint_(1234_usize);
    /// println!("Any number usize = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num = any.random_under_uint_(12_u8);
    /// println!("Any number u8 = {}", num);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num = any.random_under_uint_(1234_u16);
    /// println!("Any number u16 = {}", num);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num = any.random_under_uint_(12345678_u32);
    /// println!("Any number u32 = {}", num);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num = any.random_under_uint_(1234567890123456_u64);
    /// println!("Any number u64 = {}", num);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num = any.random_under_uint_(12345678901234567890_u128);
    /// println!("Any number u128 = {}", num);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let num = slapdash.random_under_uint_(1234_usize);
    /// println!("Slapdash number usize = {}", num);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let num = slapdash.random_under_uint_(12_u8);
    /// println!("Slapdash number u8 = {}", num);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let num = slapdash.random_under_uint_(1234_u16);
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let num = slapdash.random_under_uint_(12345678_u32);
    /// println!("Slapdash number u32 = {}", num);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let num = rand.random_under_uint_(1234567890123456_u64);
    /// println!("Random number u64 = {}", num);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let num = any.random_under_uint_(12345678901234567890_u128);
    /// println!("Any number u128 = {}", num);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let num = slapdash.random_under_uint_(1234_usize);
    /// println!("Slapdash number usize = {}", num);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let num = slapdash.random_under_uint_(12_u8);
    /// println!("Slapdash number u8 = {}", num);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let num = slapdash.random_under_uint_(1234_u16);
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// // Example for Random
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let _num = rand.random_under_uint_(0_u8);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let _num = any.random_under_uint_(0_u16);
    /// println!("Any number u16 = {}", _num);
    /// 
    /// // Example for Random_BIG_KECCAK_1024
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let _num = rand.random_under_uint_(0_u32);
    /// println!("Random number u32 = {}", _num);
    /// 
    /// // Example for Random_SHA3_512
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let _num = rand.random_under_uint_(0_u64);
    /// println!("Random number u64 = {}", _num);
    /// 
    /// // Example for Random_SHA2_512
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let _num = rand.random_under_uint_(0_u64);
    /// println!("Random number u128 = {}", _num);
    /// 
    /// // Example for Any_SHAKE_256
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let _num = any.random_under_uint_(0_usize);
    /// println!("Any number usize = {}", _num);
    /// 
    /// // Example for Any_SHAKE_128
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let _num = any.random_under_uint_(0_u8);
    /// println!("Any number u8 = {}", _num);
    /// 
    /// // Example for Any_SHA3_512
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let _num = any.random_under_uint_(0_u16);
    /// println!("Any number u16 = {}", _num);
    /// 
    /// // Example for Any_SHA3_256
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let _num = any.random_under_uint_(0_u32);
    /// println!("Any number u32 = {}", _num);
    /// 
    /// // Example for Any_SHA2_512
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let _num = any.random_under_uint_(0_u64);
    /// println!("Any number u64 = {}", _num);
    /// 
    /// // Example for Any_SHA2_256
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let _num = any.random_under_uint_(0_u64);
    /// println!("Any number u128 = {}", _num);
    /// 
    /// // Example for Slapdash_SHA1
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let _num = slapdash.random_under_uint_(0_usize);
    /// println!("Slapdash number usize = {}", _num);
    /// 
    /// // Example for Slapdash_SHA0
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let _num = slapdash.random_under_uint_(0_u8);
    /// println!("Slapdash number u8 = {}", _num);
    /// 
    /// // Example for Slapdash_MD5
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let _num = slapdash.random_under_uint_(0_u16);
    /// println!("Slapdash number u16 = {}", _num);
    /// 
    /// // Example for Slapdash_MD4
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let _num = slapdash.random_under_uint_(0_u32);
    /// println!("Slapdash number u32 = {}", _num);
    /// 
    /// // Example for Random_Rijndael
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let _num = rand.random_under_uint_(0_u64);
    /// println!("Random number u64 = {}", _num);
    /// 
    /// // Example for Any_Rijndael
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let _num = any.random_under_uint_(0_u64);
    /// println!("Any number u128 = {}", _num);
    /// 
    /// // Example for Slapdash_DES
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let _num = slapdash.random_under_uint_(0_usize);
    /// println!("Slapdash number usize = {}", _num);
    /// 
    /// // Example for Slapdash_Num_C
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let _num = slapdash.random_under_uint_(0_u8);
    /// println!("Slapdash number u8 = {}", _num);
    /// 
    /// // Example for Slapdash
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let _num = slapdash.random_under_uint_(0_u16);
    /// println!("Slapdash number u16 = {}", _num);
    /// ```
    pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
    /// 
    /// # Arguments
    /// - `from` is the lower limitation which the generated random number
    ///   should be greater than or equal to, and is of the type `T`..
    /// - `ceiling` is the upper limitation which the generated random number
    ///   should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// If `ceiling` is `0` or `from` is greater than or equal to `ceiling`,
    /// it returns a random number of type `T` less than `ceiling` and greater
    /// than or equal to `from`, and the returned random number is wrapped by
    /// enum `Some` of `Option`. Otherwise, it returns enum `None` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// if let Some(num) = rand.random_minmax_uint(12_u8, 21)
    ///     { println!("Random number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
    ///     { println!("Any number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// if let Some(num) = rand.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// if let Some(num) = rand.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// if let Some(num) = rand.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// if let Some(num) = any.random_minmax_uint(123456789_usize, 987654321)
    ///     { println!("Any number usize = {}", num); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// if let Some(num) = any.random_minmax_uint(10, 8_usize)
    ///     { println!("Any number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// if let Some(num) = any.random_minmax_uint(12_u8, 21)
    ///     { println!("Any number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
    ///     { println!("Any number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Any number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// if let Some(num) = any.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Any number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// if let Some(num) = slapdash.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Slapdash number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// if let Some(num) = slapdash.random_minmax_uint(123456789_usize, 987654321)
    ///     { println!("Slapdash number usize = {}", num); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// if let Some(num) = slapdash.random_minmax_uint(10, 8_usize)
    ///     { println!("Slapdash number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// if let Some(num) = slapdash.random_minmax_uint(12_u8, 21)
    ///     { println!("Slapdash number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
    ///     { println!("Random number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Any number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// if let Some(num) = slapdash.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Slapdash number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// if let Some(num) = slapdash.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Slapdash number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// if let Some(num) = slapdash.random_minmax_uint(10, 8_usize)
    ///     { println!("Slapdash number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    #[inline]
    pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
    /// 
    /// # Arguments
    /// - `from` is the lower limitation which the generated random number
    ///   should be greater than or equal to, and is of the type `T`..
    /// - `ceiling` is the upper limitation which the generated random number
    ///   should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// It returns a random number of type `T` less than `ceiling`
    /// and greater than or equal to `from`.
    /// 
    /// # Panics
    /// If `ceiling` is `0` or `from` is greater than or equal to `ceiling`,
    /// it will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not `0`
    /// and `from` is less than `ceiling`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let num = rand.;random_minmax_uint_(12_u8, 21)
    /// println!("Random number u8 = {}", num);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let num = any.;random_minmax_uint_(1234_u16, 6321)
    /// println!("Any number u16 = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let num = rand.;random_minmax_uint_(12345678_u32, 87654321)
    /// println!("Random number u32 = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let num = rand.;random_minmax_uint_(1234567890123456_u64, 6543210987654321)
    /// println!("Random number u64 = {}", num);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let num = rand.;random_minmax_uint_(12345678901234567890_u128, 19876543210987654321)
    /// println!("Random number u128 = {}", num);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let num = any.;random_minmax_uint_(123456789_usize, 987654321)
    /// println!("Any number usize = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num = any.;random_minmax_uint_(12_u8, 21)
    /// println!("Any number u8 = {}", num);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num = any.;random_minmax_uint_(1234_u16, 6321)
    /// println!("Any number u16 = {}", num);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num = any.;random_minmax_uint_(12345678_u32, 87654321)
    /// println!("Any number u32 = {}", num);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num = any.;random_minmax_uint_(1234567890123456_u64, 6543210987654321)
    /// println!("Any number u64 = {}", num);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num = any.;random_minmax_uint_(12345678901234567890_u128, 19876543210987654321)
    /// println!("Any number u128 = {}", num);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let num = slapdash.;random_minmax_uint_(123456789_usize, 987654321)
    /// println!("Slapdash number usize = {}", num);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let num = slapdash.;random_minmax_uint_(12_u8, 21)
    /// println!("Slapdash number u8 = {}", num);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let num = slapdash.;random_minmax_uint_(1234_u16, 6321)
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let num = slapdash.;random_minmax_uint_(12345678_u32, 87654321)
    /// println!("Slapdash number u32 = {}", num);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let num = rand.;random_minmax_uint_(1234567890123456_u64, 6543210987654321)
    /// println!("Random number u64 = {}", num);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let num = any.;random_minmax_uint_(12345678901234567890_u128, 19876543210987654321)
    /// println!("Any number u128 = {}", num);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let num = slapdash.;random_minmax_uint_(123456789_usize, 987654321)
    /// println!("Slapdash number usize = {}", num);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let num = slapdash.;random_minmax_uint_(12_u8, 21)
    /// println!("Slapdash number u8 = {}", num);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let num = slapdash.;random_minmax_uint_(1234_u16, 6321)
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// // Example for Random
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let _num = rand.random_minmax_uint_(121_u8, 21);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let _num = any.random_minmax_uint_(12345_u16, 6321);
    /// println!("Any number u16 = {}", _num);
    /// 
    /// // Example for Random_BIG_KECCAK_1024
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let _num = rand.random_minmax_uint_(123456789_u32, 87654321);
    /// println!("Random number u32 = {}", _num);
    /// 
    /// // Example for Random_SHA3_512
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let _num = rand.random_minmax_uint_(12345678901234567_u64, 6543210987654321);
    /// println!("Random number u64 = {}", _num);
    /// 
    /// // Example for Random_SHA2_512
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let _num = rand.random_minmax_uint_(123456789012345678901_u128, 19876543210987654321);
    /// println!("Random number u128 = {}", _num);
    /// 
    /// // Example for Any_SHAKE_256
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let num = any.;random_minmax_uint_(1234567890_usize, 987654321)
    /// println!("Any number usize = {}", num);
    /// 
    /// // Example for Any_SHAKE_128
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num = any.;random_minmax_uint_(123_u8, 21)
    /// println!("Any number u8 = {}", num);
    /// 
    /// // Example for Any_SHA3_512
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num = any.;random_minmax_uint_(12345_u16, 6321)
    /// println!("Any number u16 = {}", num);
    /// 
    /// // Example for Any_SHA3_256
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num = any.;random_minmax_uint_(123456789_u32, 87654321)
    /// println!("Any number u32 = {}", num);
    /// 
    /// // Example for Any_SHA2_512
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num = any.;random_minmax_uint_(12345678901234567_u64, 6543210987654321)
    /// println!("Any number u64 = {}", num);
    /// 
    /// // Example for Any_SHA2_256
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num = any.;random_minmax_uint_(123456789012345678901_u128, 19876543210987654321)
    /// println!("Any number u128 = {}", num);
    /// 
    /// // Example for Slapdash_SHA1
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let num = slapdash.;random_minmax_uint_(1234567890_usize, 987654321)
    /// println!("Slapdash number usize = {}", num);
    /// 
    /// // Example for Slapdash_SHA0
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let num = slapdash.;random_minmax_uint_(123_u8, 21)
    /// println!("Slapdash number u8 = {}", num);
    /// 
    /// // Example for Slapdash_MD5
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let num = slapdash.;random_minmax_uint_(12345_u16, 6321)
    /// println!("Slapdash number u16 = {}", num);
    /// 
    /// // Example for Slapdash_MD4
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let num = slapdash.;random_minmax_uint_(123456789_u32, 87654321)
    /// println!("Slapdash number u32 = {}", num);
    /// 
    /// // Example for Random_Rijndael
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let num = rand.;random_minmax_uint_(12345678901234567_u64, 6543210987654321)
    /// println!("Random number u64 = {}", num);
    /// 
    /// // Example for Any_Rijndael
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let num = any.;random_minmax_uint_(123456789012345678901_u128, 19876543210987654321)
    /// println!("Any number u128 = {}", num);
    /// 
    /// // Example for Slapdash_DES
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let num = slapdash.;random_minmax_uint_(1234567890_usize, 987654321)
    /// println!("Slapdash number usize = {}", num);
    /// 
    /// // Example for Slapdash_Num_C
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let num = slapdash.;random_minmax_uint_(123_u8, 21)
    /// println!("Slapdash number u8 = {}", num);
    /// 
    /// // Example for Slapdash
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let num = slapdash.;random_minmax_uint_(12345_u16, 6321)
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    #[inline]
    pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_uint<T>(&mut self) -> T
    /// Generates random odd numbers of type `T`.
    /// 
    /// # Output
    /// It returns a random odd number of type `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// println!("Random odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// println!("Any odd number u8 = {}", any.random_odd_uint::<u8>());
    /// println!("Any odd number u16 = {}", any.random_odd_uint::<u16>());
    /// println!("Any odd number u32 = {}", any.random_odd_uint::<u32>());
    /// println!("Any odd number u64 = {}", any.random_odd_uint::<u64>());
    /// println!("Any odd number u128 = {}", any.random_odd_uint::<u128>());
    /// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// println!("Slapdash odd number u8 = {}", slapdash.random_odd_uint::<u8>());
    /// println!("Slapdash odd number u16 = {}", slapdash.random_odd_uint::<u16>());
    /// println!("Slapdash odd number u32 = {}", slapdash.random_odd_uint::<u32>());
    /// println!("Slapdash odd number u64 = {}", slapdash.random_odd_uint::<u64>());
    /// println!("Slapdash odd number u128 = {}", slapdash.random_odd_uint::<u128>());
    /// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
    /// ```
    pub fn random_odd_uint<T>(&mut self) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// It returns a random odd numbers of type `T` less than `ceiling`
    /// wrapped by enum `Some` of `Option`.
    /// 
    /// # Features
    /// If `ceiling` is `0`, it returns `None`. Otherwise, it returns a
    /// random odd numbers of type `T` wrapped by enum `Some` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(0_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(0_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(0_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(0_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(0_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(0_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(0_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(0_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// if let Some(num) = rand.random_odd_under_uint(12_u8)
    ///     { println!("Random odd number u8 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234_u16)
    ///     { println!("Random odd number u16 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678_u32)
    ///     { println!("Random odd number u32 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Random odd number u64 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Random odd number u128 = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(123456789_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// if let Some(num) = rand.random_odd_under_uint(1_usize)
    ///     { println!("Random odd number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// if let Some(num) = any.random_odd_under_uint(12_u8)
    ///     { println!("Any odd number u8 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234_u16)
    ///     { println!("Any odd number u16 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678_u32)
    ///     { println!("Any odd number u32 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Any odd number u64 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Any odd number u128 = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(123456789_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// if let Some(num) = any.random_odd_under_uint(0_usize)
    ///     { println!("Any odd number usize = {}", num); }
    /// else
    ///     { println!("No any unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 1!"); }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(0_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 0!"); }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// if let Some(num) = slapdash.random_odd_under_uint(12_u8)
    ///     { println!("Slapdash odd number u8 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
    ///     { println!("Slapdash odd number u16 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678_u32)
    ///     { println!("Slapdash odd number u32 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1234567890123456_u64)
    ///     { println!("Slapdash odd number u64 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(12345678901234567890_u128)
    ///     { println!("Slapdash odd number u128 = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(123456789_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// if let Some(num) = slapdash.random_odd_under_uint(1_usize)
    ///     { println!("Slapdash odd number usize = {}", num); }
    /// else
    ///     { println!("No slapdash unsigned odd number under 1!"); }
    /// ```
    #[inline]
    pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of the type `T`.
    /// 
    /// # Output
    /// It returns a random odd numbers of type `T` less than `ceiling`.
    /// 
    /// # Panics
    /// If `ceiling` is `0`, it will panic.
    /// 
    /// # Caution
    /// Use only when `ceiling` is not `0`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// 
    /// let num = rand.random_odd_under_uint_(12_u8);
    /// println!("Random odd number u8 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234_u16);
    /// println!("Random odd number u16 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678_u32);
    /// println!("Random odd number u32 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Random odd number u64 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Random odd number u128 = {}", num);
    /// 
    /// let num = rand.random_odd_under_uint_(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// 
    /// let num = any.random_odd_under_uint_(12_u8);
    /// println!("Any odd number u8 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234_u16);
    /// println!("Any odd number u16 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678_u32);
    /// println!("Any odd number u32 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Any odd number u64 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Any odd number u128 = {}", num);
    /// 
    /// let num = any.random_odd_under_uint_(123456789_usize);
    /// println!("Any odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// 
    /// let num = slapdash.random_odd_under_uint_(12_u8);
    /// println!("Slapdash odd number u8 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234_u16);
    /// println!("Slapdash odd number u16 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678_u32);
    /// println!("Slapdash odd number u32 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(1234567890123456_u64);
    /// println!("Slapdash odd number u64 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(12345678901234567890_u128);
    /// println!("Slapdash odd number u128 = {}", num);
    /// 
    /// let num = slapdash.random_odd_under_uint_(123456789_usize);
    /// println!("Slapdash odd number usize = {}", num);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// // Example for Random
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let _num = rand.random_odd_under_uint_(0_u8);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let _num = any.random_odd_under_uint_(1_u16);
    /// println!("Any number u16 = {}", _num);
    /// 
    /// // Example for Random_BIG_KECCAK_1024
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let _num = rand.random_odd_under_uint_(0_u32);
    /// println!("Random number u32 = {}", _num);
    /// 
    /// // Example for Random_SHA3_512
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let _num = rand.random_odd_under_uint_(1_u64);
    /// println!("Random number u64 = {}", _num);
    /// 
    /// // Example for Random_SHA2_512
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let _num = rand.random_odd_under_uint_(0_u128);
    /// println!("Random number u128 = {}", _num);
    /// 
    /// // Example for Any_SHAKE_256
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let num = any.random_odd_under_uint_(1_usize);
    /// println!("Any number usize = {}", num);
    /// 
    /// // Example for Any_SHAKE_128
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num = any.random_odd_under_uint_(0_u8);
    /// println!("Any number u8 = {}", num);
    /// 
    /// // Example for Any_SHA3_512
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num = any.random_odd_under_uint_(1_u16);
    /// println!("Any number u16 = {}", num);
    /// 
    /// // Example for Any_SHA3_256
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num = any.random_odd_under_uint_(0_u32);
    /// println!("Any number u32 = {}", num);
    /// 
    /// // Example for Any_SHA2_512
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num = any.random_odd_under_uint_(1_u64);
    /// println!("Any number u64 = {}", num);
    /// 
    /// // Example for Any_SHA2_256
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num = any.random_odd_under_uint_(0_u128);
    /// println!("Any number u128 = {}", num);
    /// 
    /// // Example for Slapdash_SHA1
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let num = slapdash.random_odd_under_uint_(1_usize);
    /// println!("Slapdash number usize = {}", num);
    /// 
    /// // Example for Slapdash_SHA0
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let num = slapdash.random_odd_under_uint_(0_u8);
    /// println!("Slapdash number u8 = {}", num);
    /// 
    /// // Example for Slapdash_MD5
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let num = any.random_odd_under_uint_(1_u16);
    /// println!("Slapdash number u16 = {}", num);
    /// 
    /// // Example for Slapdash_MD4
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let num = slapdash.random_odd_under_uint_(0_u32);
    /// println!("Slapdash number u32 = {}", num);
    /// 
    /// // Example for Random_Rijndael
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let num = rand.random_odd_under_uint_(1_u64);
    /// println!("Random number u64 = {}", num);
    /// 
    /// // Example for Any_Rijndael
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let num = any.random_odd_under_uint_(0_u128);
    /// println!("Any number u128 = {}", num);
    /// 
    /// // Example for Slapdash_DES
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let num = slapdash.random_odd_under_uint_(1_usize);
    /// println!("Slapdash number usize = {}", num);
    /// 
    /// // Example for Slapdash_Num_C
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let num = slapdash.random_odd_under_uint_(0_u8);
    /// println!("Slapdash number u8 = {}", num);
    /// 
    /// // Example for Slapdash
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let num = slapdash.random_odd_under_uint_(1_u16);
    /// println!("Slapdash number u16 = {}", num);
    /// ```
    pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_with_msb_set_uint<T>(&mut self) -> T
    /// Generates random numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Output
    /// It returns random numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit number = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// println!("Random 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// println!("Any 8-bit number = {}", any.random_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit number = {}", any.random_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit number = {}", any.random_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit number = {}", any.random_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit number = {}", any.random_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized number = {}", any.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// println!("Slapdash 8-bit number = {}", slapdash.random_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit number = {}", slapdash.random_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit number = {}", slapdash.random_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit number = {}", slapdash.random_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit number = {}", slapdash.random_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized number = {}", slapdash.random_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_with_msb_set_uint<T>(&mut self) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_with_msb_set_uint<T>(&mut self) -> T
    /// Generates random odd numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Output
    /// It returns random odd numbers of type `T` and of maximum size of `T`.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number = {}", rand.random_odd_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number = {}", rand.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// println!("Random 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// println!("Any 8-bit odd number = {}", any.random_odd_with_msb_set_uint::<u8>());
    /// println!("Any 16-bit odd number = {}", any.random_odd_with_msb_set_uint::<u16>());
    /// println!("Any 32-bit odd number = {}", any.random_odd_with_msb_set_uint::<u32>());
    /// println!("Any 64-bit odd number = {}", any.random_odd_with_msb_set_uint::<u64>());
    /// println!("Any 128-bit odd number = {}", any.random_odd_with_msb_set_uint::<u128>());
    /// println!("Any usize-sized odd number = {}", any.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// println!("Slapdash 8-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u8>());
    /// println!("Slapdash 16-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u16>());
    /// println!("Slapdash 32-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u32>());
    /// println!("Slapdash 64-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u64>());
    /// println!("Slapdash 128-bit odd number = {}", slapdash.random_odd_with_msb_set_uint::<u128>());
    /// println!("Slapdash usize-sized odd number = {}", slapdash.random_odd_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_odd_with_msb_set_uint<T>(&mut self) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a random prime number.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
    /// 
    /// # Output
    /// A random prime number whose range is from 2 up to T::max() inclusively.
    /// 
    /// # Features
    /// - It uses
    ///   [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in notprime number, the
    ///   probability that the tested number is not a prime number is 1/4 (= 25%).
    ///   So, if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4 = 6.25%).
    ///   Therefore, if you test any number 5 times and they all result in a prime
    ///   number, the probability that the tested number is not a prime number is
    ///   1/1024 = (= 1/4 * 1/4 * 1/4 * 1/4 * 1/4 = 0.09765625%). In other words,
    ///   it is about 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// If you want to use a `(sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to use the method
    /// [random_prime_with_msb_set_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// println!("Random 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a full-sized random prime number, which is its MSB (Most
    /// Segnificant Bit) is set `1`.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
    /// 
    /// # Output
    /// A full-sized random prime number, which is its MSB (Most Segnificant
    /// Bit) is set `1` and whose range is from 2 up to T::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply sets its MSB
    /// (Most Significant Bit) to be one, and then checks whether or not the
    /// generated random number is prime number, and then it repeats until it
    /// will generate a prime number.
    /// - It uses
    /// [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in notprime number, the
    /// probability that the tested number is not a prime number is 1/4 (= 25%).
    /// So, if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4 = 6.25%).
    /// Therefore, if you test any number 5 times and they all result in a prime
    /// number, the probability that the tested number is not a prime number is
    /// 1/1024 = (= 1/4 * 1/4 * 1/4 * 1/4 * 1/4 = 0.09765625%). In other words,
    /// it is about 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// If you want to use a normal random prime number, you are highly
    /// recommended to use the method
    /// [random_prime_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// println!("Random 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// println!("Any 8-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Any 16-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Any 32-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Any 64-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Any 128-bit prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Any usize-sized prime number = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// println!("Slapdash 8-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Slapdash 16-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Slapdash 32-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Slapdash 64-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Slapdash 128-bit prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Slapdash usize-sized prime number = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: TraitsBigUInt<T>
    {
        unimplemented!(); // Dummy code for documentation
    }
}
