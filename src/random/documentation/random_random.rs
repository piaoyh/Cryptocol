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


use std::fmt::{ Debug, Display };
// use std::ops::*;
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };
use std::cmp::{ PartialEq, PartialOrd};

use crate::number::{ SmallUInt, BigUInt };
use crate::random::Random_Engine;


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
    // pub fn new() -> Self
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random::new();
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any::new();
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let num: U512 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// println!("Any number = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// println!("Any number = {}", any.random_u32());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// println!("Any number = {}", any.random_u16());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// println!("Any number = {}", any.random_u8());
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA1::new();
    /// println!("Any number = {}", any.random_usize());
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA0::new();
    /// println!("Any number = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_MD5::new();
    /// println!("Any number = {}", any.random_u32());
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_MD4::new();
    /// println!("Any number = {}", any.random_u16());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_DES::new();
    /// println!("Any number = {}", any.random_odd_biguint());
    /// ```
    pub fn new() -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with<SG, AG>(mut main_generator: SG, mut aux_generator: AG) -> Self
    /// Constructs a new `Random_Generic` object
    /// with two random number generator engines.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::RandGen;
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = RandGen::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new());
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with(SHA3_512::new(), SHA3_512::new());
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with(SHA2_512::new(), SHA2_512::new());
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::random::RandGen;
    /// use cryptocol::hash::SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = RandGen::new_with(SHAKE_256::new(), SHAKE_256::new());
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with(SHAKE_128::new(), SHAKE_128::new());
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with(SHA3_256::new(), SHA3_256::new());
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA2_256
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA2_256;
    /// 
    /// let mut any = AnyGen::new_with(SHA2_256::new(), SHA2_256::new());
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 7 for SHA1 and SHA0
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// 
    /// let mut any = AnyGen::new_with(SHA1::new(), SHA0::new());
    /// println!("Any number = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 8 for MD5 and MD4
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::{ MD5, MD4 };
    /// 
    /// let mut any = AnyGen::new_with(MD5::new(), MD4::new());
    /// println!("Any number = {}", any.random_u32());
    /// ```
    /// 
    /// # Example 9 for AES_128
    /// ```
    /// use cryptocol::random::RandGen;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut rand = RandGen::new_with(AES_128::new(), AES_128::new());
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 9 for DES
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let mut any = AnyGen::new_with(DES::new(), DES::new());
    /// println!("Any number = {}", any.random_u8());
    /// ```
    pub fn new_with<SG, AG>(mut main_generator: SG, mut aux_generator: AG) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_seeds<T>(seed: T, aux: T) -> Self
    /// Constructs a new struct Random_Generic with two seeds of type `T` given.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of any type that has the trait `SmallUInt`
    /// such as `u8`, `u16`, `u32`, `u64`, u128`, and `usize`.
    /// - `aux` is the seed number of any type that has trait `SmallUInt`
    /// such as `u8`, `u16`, `u32`, `u64`, u128`, and `usize`.
    /// 
    /// # Output
    /// It returns a new object of Random_Generic.
    /// 
    /// # Panics
    /// If `COUNT` is `0` or greator than `i32::MAX`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
    /// 
    /// #  Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random::new_with_seeds(10500872879054459758_u64, 15887751380961987625_u64);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any::new_with_seeds(100, 25);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new_with_seeds(0, 0);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 5 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_256::new_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new_with_seeds(123456789, 987654321);
    /// let num: U512 = any.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new_with_seeds(u32::MAX as u64, u32::MAX as u64);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 8 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA2_512::new_with_seeds(15698731215687456325, 10684237915728469725);
    /// let num: U256 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 9 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// 
    /// let mut any = Any_SHA2_512::new_with_seeds(2879054410500759758, 15887876257513809619);
    /// if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Any number = {}", num); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// 
    /// let mut any = Any_SHA2_256::new_with_seeds(610458805, 215793685);
    /// if let Some(num) = any.random_under_uint(1234_u16)
    ///     { println!("Any number = {}", num); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// let mut any = Any_SHA1::new_with_seeds(18782, 50558);
    /// println!("Any number = {}", any.random_uint::<u8>());
    /// ```
    /// 
    /// # Example 12 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// 
    /// let mut any = Any_SHA0::new_with_seeds(0, 125);
    /// println!("Any prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// ```
    /// 
    /// # Example 13 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// 
    /// let mut any_md5 = Any_MD5::new_with_seeds(58, 161);
    /// println!("Any number = {}", any_md5.random_u128());
    /// ```
    /// 
    /// # Example 14 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// 
    /// let mut any_md4 = Any_MD4::new_with_seeds(106842379157284697, 18446744073709551615);
    /// println!("Any number = {}", any_md4.random_u64());
    /// ```
    /// 
    /// # Example 14 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 15 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// 
    /// let mut any = Any_Rijndael::new_with_seeds(u16::MAX as u64, u16::MAX as u64);
    /// println!("Any number = {}", any.random_u16());
    /// ```
    /// 
    /// # Example 16 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// 
    /// let mut any = Any_DES::new_with_seeds(u8::MAX as u64, u8::MAX as u64);
    /// println!("Any number = {}", any.random_u8());
    /// ```
    pub fn new_with_seeds(seed: u64, aux: u64) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_generators_seeds<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed: u64, aux: u64) -> Self
    /// Constructs a new `Random_Generic` object with
    /// two random number generator engines and two seeds of type `T` given.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// - `seed` is the seed number of any type that has the trait `SmallUInt`
    ///   such as `u8`, `u16`, `u32`, `u64`, u128`, and `usize`.
    /// - `aux` is the seed number of any type that has trait `SmallUInt`
    ///   such as `u8`, `u16`, `u32`, `u64`, u128`, and `usize`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = RandGen::new_with_generators_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), 10500872879054459758_u64, 15887751380961987625_u64);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), 100, 25);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), 0, 0);
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = RandGen::new_with_generators_seeds(SHAKE_256::new(), SHAKE_256::new(), u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHAKE_128::new(), SHAKE_128::new(), 123456789, 987654321);
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA3_256::new(), SHA3_256::new(), u32::MAX as u64, u32::MAX as u64);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::SHA2_256;
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), 15698731215687456325, 10684237915728469725);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA1::new(), SHA0::new(), 2879054410500759758, 15887876257513809619);
    /// println!("Any number = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::hash::{ MD5, MD4 };
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(MD5::new(), MD4::new(), 610458805, 215793685);
    /// println!("Any number = {}", any.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut any = RandGen::new_with_generators_seeds(AES_128::new(), AES_128::new(), 18782, 50558);
    /// println!("Random number = {}", any.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::random::{ AnyGen, RandGen };
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// use cryptocol::symmetric::DES;
    /// let mut any = AnyGen::new_with_generators_seeds(DES::new(), DES::new(), 0, 125);
    /// println!("Any number = {}", any.random_u8());
    /// ```
    pub fn new_with_generators_seeds<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed: u64, aux: u64) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_u8(&mut self) -> u8
    /// Generates random numbers of type `u8`.
    /// 
    /// # Output
    /// It returns a random number of type `u8`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u8()); }
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
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u8()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_u8()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_u8()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u16()); }
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
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u16()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_u16()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_u16()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u32()); }
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
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u32()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_u32()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_u32()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u64()); }
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
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u64()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_u64()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_u64()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_u128()); }
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
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_u128()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_u128()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_u128()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Any_SHA3_256) = {}", i, any.random_usize()); }
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
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_usize()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_usize()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_usize()); }
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("{} Random number (Random_SHA2_512) = {}", i, any.random_uint::<u8>()); }
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA1) = {}", i, any.random_uint::<usize>()); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_SHA0) = {}", i, any.random_uint::<u32>()); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD5) = {}", i, any.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_MD4) = {}", i, any.random_uint::<u128>()); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Random_Rijndael) = {}", i, rand.random_uint::<u8>()); }
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
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_DES) = {}", i, any.random_uint::<usize>()); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num_C) = {}", i, any.random_uint::<u64>()); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// for i in 0..10
    ///     { println!("{} Any number (Any_Num) = {}", i, any.random_uint::<u128>()); }
    /// ```
    pub fn random_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling`.
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("Random number u16 = {}", num); }
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
    ///     { println!("Random number usize = {}", num); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// if let Some(num) = any.random_under_uint(0_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number under 0!"); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// if let Some(num) = any.random_under_uint(12_u8)
    ///     { println!("Random number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// if let Some(num) = any.random_under_uint(1234_u16)
    ///     { println!("Random number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// if let Some(num) = any.random_under_uint(12345678_u32)
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// if let Some(num) = any.random_under_uint(1234567890123456_u64)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// if let Some(num) = any.random_under_uint(12345678901234567890_u128)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// if let Some(num) = any.random_under_uint(1234_usize)
    ///     { println!("Random number usize = {}", num); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// if let Some(num) = any.random_under_uint(0_u64)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number under 0!"); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// if let Some(num) = any.random_under_uint(12_u8)
    ///     { println!("Random number u8 = {}", num); }
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
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// if let Some(num) = any.random_under_uint(1234567890123456_u64)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// if let Some(num) = any.random_under_uint(12345678901234567890_u128)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// if let Some(num) = any.random_under_uint(0_u32)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number under 0!"); }
    /// ```
    #[inline]
    pub fn random_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling`.
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    /// println!("Random number usize = {}", num);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num = any.random_under_uint_(12_u8);
    /// println!("Random number u8 = {}", num);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num = any.random_under_uint_(1234_u16);
    /// println!("Random number u16 = {}", num);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num = any.random_under_uint_(12345678_u32);
    /// println!("Random number u32 = {}", num);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num = any.random_under_uint_(1234567890123456_u64);
    /// println!("Random number u64 = {}", num);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num = any.random_under_uint_(12345678901234567890_u128);
    /// println!("Random number u128 = {}", num);
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// let num = any.random_under_uint_(1234_usize);
    /// println!("Random number usize = {}", num);
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// let num = any.random_under_uint_(12_u8);
    /// println!("Random number u8 = {}", num);
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// let num = any.random_under_uint_(1234_u16);
    /// println!("Random number u16 = {}", num);
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// let num = any.random_under_uint_(12345678_u32);
    /// println!("Random number u32 = {}", num);
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
    /// println!("Random number u128 = {}", num);
    /// ```
    /// 
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// let num = any.random_under_uint_(1234_usize);
    /// println!("Random number usize = {}", num);
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// let num = any.random_under_uint_(12_u8);
    /// println!("Random number u8 = {}", num);
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// let num = any.random_under_uint_(1234_u16);
    /// println!("Random number u16 = {}", num);
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
    /// println!("Random number u16 = {}", _num);
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
    /// println!("Random number usize = {}", _num);
    /// 
    /// // Example for Any_SHAKE_128
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let _num = any.random_under_uint_(0_u8);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any_SHA3_512
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let _num = any.random_under_uint_(0_u16);
    /// println!("Random number u16 = {}", _num);
    /// 
    /// // Example for Any_SHA3_256
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let _num = any.random_under_uint_(0_u32);
    /// println!("Random number u32 = {}", _num);
    /// 
    /// // Example for Any_SHA2_512
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let _num = any.random_under_uint_(0_u64);
    /// println!("Random number u64 = {}", _num);
    /// 
    /// // Example for Any_SHA2_256
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let _num = any.random_under_uint_(0_u64);
    /// println!("Random number u128 = {}", _num);
    /// 
    /// // Example for Any_SHA1
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// let _num = any.random_under_uint_(0_usize);
    /// println!("Random number usize = {}", _num);
    /// 
    /// // Example for Any_SHA0
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// let _num = any.random_under_uint_(0_u8);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any_MD5
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// let _num = any.random_under_uint_(0_u16);
    /// println!("Random number u16 = {}", _num);
    /// 
    /// // Example for Any_MD4
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// let _num = any.random_under_uint_(0_u32);
    /// println!("Random number u32 = {}", _num);
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
    /// println!("Random number u128 = {}", _num);
    /// 
    /// // Example for Any_DES
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// let _num = any.random_under_uint_(0_usize);
    /// println!("Random number usize = {}", _num);
    /// 
    /// // Example for Any_Num_C
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// let _num = any.random_under_uint_(0_u8);
    /// println!("Random number u8 = {}", _num);
    /// 
    /// // Example for Any_Num
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// let _num = any.random_under_uint_(0_u16);
    /// println!("Random number u16 = {}", _num);
    /// ```
    pub fn random_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
    /// 
    /// # Output
    /// If `ceiling` is `0` or `from` is greater than or equal to `ceiling`,
    /// it returns a random number of type `T` less than `ceiling` and greater
    /// than or equal to `from`, and the returned random number is wrapped by
    /// enum `Some` of `Option`. Otherwise, it returns enum `None` of `Option`.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    ///   insecure.
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
    ///     { println!("Random number u16 = {}", num); }
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
    ///     { println!("Random number usize = {}", num); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// if let Some(num) = any.random_minmax_uint(10, 8_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// if let Some(num) = any.random_minmax_uint(12_u8, 21)
    ///     { println!("Random number u8 = {}", num); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
    ///     { println!("Random number u16 = {}", num); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// if let Some(num) = any.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 12 for Any_SHA1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut any = Any_SHA1::new();
    /// if let Some(num) = any.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 13 for Any_SHA0
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut any = Any_SHA0::new();
    /// if let Some(num) = any.random_minmax_uint(123456789_usize, 987654321)
    ///     { println!("Random number usize = {}", num); }
    /// ```
    /// 
    /// # Example 14 for Any_MD5
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut any = Any_MD5::new();
    /// if let Some(num) = any.random_minmax_uint(10, 8_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    /// 
    /// # Example 15 for Any_MD4
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut any = Any_MD4::new();
    /// if let Some(num) = any.random_minmax_uint(12_u8, 21)
    ///     { println!("Random number u8 = {}", num); }
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
    ///     { println!("Random number u32 = {}", num); }
    /// ```
    /// 
    /// # Example 18 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// let mut any = Any_DES::new();
    /// if let Some(num) = any.random_minmax_uint(1234567890123456_u64, 6543210987654321)
    ///     { println!("Random number u64 = {}", num); }
    /// ```
    /// 
    /// # Example 19 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new();
    /// if let Some(num) = any.random_minmax_uint(12345678901234567890_u128, 19876543210987654321)
    ///     { println!("Random number u128 = {}", num); }
    /// ```
    /// 
    /// # Example 20 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new();
    /// if let Some(num) = any.random_minmax_uint(10, 8_usize)
    ///     { println!("Random number usize = {}", num); }
    /// else
    ///     { println!("No random unsigned number number greater than or equal to 10 and less than 8!"); }
    /// ```
    #[inline]
    pub fn random_minmax_uint<T>(&mut self, from: T, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    /// Generates random numbers of type `T` less than `ceiling` exclusively
    /// and greater than or equal to `from` inclusively.
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// let mut rand = Any_MD5::new();
    /// 
    /// let num = rand.random_minmax_uint_(12_u8, 21);
    /// println!("Random number u8 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(1234_u16, 6321);
    /// println!("Random number u16 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(12345678_u32, 87654321);
    /// println!("Random number u32 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(1234567890123456_u64, 6543210987654321);
    /// println!("Random number u64 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(12345678901234567890_u128, 19876543210987654321);
    /// println!("Random number u128 = {}", num);
    /// 
    /// let num = rand.random_minmax_uint_(123456789_usize, 987654321);
    /// println!("Random number usize = {}", num);
    /// 
    /// // It will panic!
    /// // let num = rand.random_minmax_uint_(10, 8_usize);
    /// // println!("Random number usize = {}", num);
    /// ```
    #[inline]
    pub fn random_minmax_uint_<T>(&mut self, from: T, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// let mut rand = Any_SHA0::new();
    /// println!("Random odd number u8 = {}", rand.random_odd_uint::<u8>());
    /// println!("Random odd number u16 = {}", rand.random_odd_uint::<u16>());
    /// println!("Random odd number u32 = {}", rand.random_odd_uint::<u32>());
    /// println!("Random odd number u64 = {}", rand.random_odd_uint::<u64>());
    /// println!("Random odd number u128 = {}", rand.random_odd_uint::<u128>());
    /// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
    /// ```
    pub fn random_odd_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    /// Generates random odd numbers of type `T` less than `ceiling`.
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// let mut rand = Any_SHA1::new();
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
    ///     { println!("No random unsigned number number under 0!"); }
    /// ```
    #[inline]
    pub fn random_odd_under_uint<T>(&mut self, ceiling: T) -> Option<T>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    /// Generates random odd numbers of type `T` less than `ceiling`.
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut rand = Any_SHA2_256::new();
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
    /// let num = rand.random_odd_under_uint_::<usize>(123456789_usize);
    /// println!("Random odd number usize = {}", num);
    /// 
    /// // It will panic.
    /// // let num = rand.random_odd_under_uint_::<usize>(0_usize);
    /// // println!("Random odd number usize = {}", num);
    /// ```
    pub fn random_odd_under_uint_<T>(&mut self, ceiling: T) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// println!("Random 8-bit number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized number usize = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_with_msb_set_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
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
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// println!("Random 8-bit odd number u8 = {}", rand.random_with_msb_set_uint::<u8>());
    /// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
    /// println!("Random 32-bit odd number u32 = {}", rand.random_with_msb_set_uint::<u32>());
    /// println!("Random 64-bit odd number u64 = {}", rand.random_with_msb_set_uint::<u64>());
    /// println!("Random 128-bit odd number u128 = {}", rand.random_with_msb_set_uint::<u128>());
    /// println!("Random usize-sized odd number usize = {}", rand.random_with_msb_set_uint::<usize>());
    /// ```
    pub fn random_odd_with_msb_set_uint<T>(&mut self) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a random prime number.
    /// 
    /// # Output
    /// A random prime number whose range is from 2 up to T::max() inclusively.
    /// 
    /// # Features
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
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a `(sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method
    /// [random_prime_with_msb_set_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// println!("Random prime number u8 = {}", rand.random_prime_using_miller_rabin_uint::<u8>(5));
    /// println!("Random prime number u16 = {}", rand.random_prime_using_miller_rabin_uint::<u16>(5));
    /// println!("Random prime number u32 = {}", rand.random_prime_using_miller_rabin_uint::<u32>(5));
    /// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
    /// println!("Random prime number u128 = {}", rand.random_prime_using_miller_rabin_uint::<u128>(5));
    /// println!("Random prime number usize = {}", rand.random_prime_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    /// Returns a full-sized random prime number, which is its MSB (Most
    /// Segnificant Bit) is set `1`.
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
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether or not
    /// the generated random number is prime. Usually, `repetition` is given to
    /// be 5 to have 99.9% hit rate.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random prime number, you are highly
    /// recommended to use the method
    /// [random_prime_using_miller_rabin_uint()](struct@Random_Generic#method.random_prime_using_miller_rabin_uint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// println!("Random 8-bit prime number u8 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u8>(5));
    /// println!("Random 16-bit prime number u16 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u16>(5));
    /// println!("Random 32-bit prime number u32 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u32>(5));
    /// println!("Random 64-bit prime number u64 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u64>(5));
    /// println!("Random 128-bit prime number u128 = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<u128>(5));
    /// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_uint<T>(&mut self, repetition: usize) -> T
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_array<T, const N: usize>(&mut self) -> [T; N]
    /// Returns random number array [T; N].
    /// 
    /// # Output
    /// A random number array [T; N] each element of which has a range from
    /// `0` up to `T::max()` inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
    /// depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any;
    /// let mut rand = Any::new();
    /// let num: [u128; 20] = rand.random_array();
    /// for i in 0..20
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    pub fn random_array<T, const N: usize>(&mut self) -> [T; N]
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn put_random_in_array<T, const N: usize>(&mut self, out: &mut [T; N])
    /// Puts random number array [T; N] in `out`.
    /// 
    /// # Argument
    /// `out` is a random number array [T; N] each element of which has
    /// a range from `0` up to `T::max()` inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
    /// depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// let mut rand = Any_MD4::new();
    /// let mut num = [0_u64; 32];
    /// rand.put_random_in_array(&mut num);
    /// for i in 0..32
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    pub fn put_random_in_array<T, const N: usize>(&mut self, out: &mut [T; N])
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value.
    /// 
    /// # Output
    /// A random number whose range is from `0` up to `BigUInt::max()`
    /// inclusively for both ends.
    /// 
    /// # Features
    /// The random numbers that may or may not be cryptographically secure
    /// depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD5;
    /// 
    /// define_utypes_with!(u128);
    /// let mut rand = Any_MD5::new();
    /// let biguint: U512 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    #[inline]
    pub fn random_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Output
    /// A random number wrapped by enum `Some` of `Option`, whose range is
    /// between 0 inclusively and the certain value exclusively when `ceiling`
    /// is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    /// the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA0;
    /// 
    /// define_utypes_with!(u64);
    /// let mut rand = Any_SHA0::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    #[inline]
    pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    /// the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// define_utypes_with!(u32);
    /// let mut rand = Any_SHA1::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline]
    pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value.
    /// 
    /// # Output
    /// The random number that this method `any_odd()` returns is a pure
    /// random odd number whose range is from `1` up to `BigUInt::max()`
    /// inclusively for both ends.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA2_256;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Any_SHA2_256::new();
    /// let r: U256 = rand.random_odd_biguint();
    /// ```
    pub fn random_odd_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively, wrapped by enum `Some` of `Option` if
    /// `ceiling` is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    /// by the certain value to get its remainder and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD4;
    /// 
    /// define_utypes_with!(u128);
    /// let mut rand = Any_MD4::new();
    /// let ceiling = U1024::max().wrapping_div_uint(4_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    #[inline]
    pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    /// by the certain value to get its remainder and then simply set the LSB
    /// (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA2_512;
    /// 
    /// define_utypes_with!(u8);
    /// let mut rand = Any_SHA2_512::new();
    /// let ceiling = U1024::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline]
    pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random value
    /// with `(N * sizeof::<T>() * 8)`-bit length.
    /// 
    /// # Output
    /// The random number whose range is from !(BigUInt::max() >> 1) up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_MD5;
    /// 
    /// define_utypes_with!(u64);
    /// let mut rand = Any_MD5::new();
    /// let biguint: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    #[inline]
    pub fn random_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value with `(N * sizeof::<T>() * 8)`-bit length
    /// 
    /// # Output
    /// The random number that this method random_odd_with_msb_set() returns is
    /// a random odd number whose range is from !(BigUInt::max() >> 1) + 1 up to
    /// BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply set the MSB
    /// (Most Significant Bit) and LSB (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any;
    /// 
    /// define_utypes_with!(u32);
    /// let mut rand = Any::new();
    /// let num:U512 = rand.random_odd_with_msb_set_biguint();
    /// println!("512-bit Random Odd Number = {}", num);
    /// assert!(num > U512::halfmax());
    /// assert!(num.is_odd());
    /// ```
    pub fn random_odd_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// 2 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in prime number, the
    /// probability that the tested number is not a prime number is 1/4. So,
    /// if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    /// if you test any number 5 times and they all result in a prime number,
    /// it is 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    /// number, you are highly recommended to
    /// use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::number::BigUInt_Prime;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Random;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Random::new();
    /// let num:U512 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random Prime Number = {}", num);
    /// assert!(num.is_odd());
    /// ```
    pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number of full-size of BigUInt<T, N>.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply sets its MSB
    /// (Most Significant Bit) to be one, and then checks whether or not the
    /// generated random number is prime number, and then it repeats until it
    /// will generate a prime number.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    /// a composite number. If this test results in prime number, the
    /// probability that the tested number is not a prime number is 1/4. So,
    /// if the test results in prime number twice, the probability that the
    /// tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    /// if you test any number 5 times and they all result in a prime number,
    /// it is 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    /// secure depending on what pseudo-random number generator is used.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy. 
    /// 
    /// # Cryptographical Security
    /// - If you use `Random`, it is considered to be cryptographically secure.
    /// - If you use `Any`, it is considered that it may be cryptographically
    /// insecure.
    /// - However, if you really want to use cryptographically secure
    /// random number with high quality, you may want to use
    /// [rand::rngs::OsRng](https://docs.rs/rand/latest/rand/rngs/struct.OsRng.html)).
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    /// highly recommended to use the method
    /// [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    /// use the method
    /// [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    /// rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    /// you are highly recommended to use the method
    /// [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    /// number, you are highly recommended to use the method
    /// [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    /// number, you are highly recommended to
    /// use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    /// rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    /// use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    /// rather than this method.
    /// 
    /// # Big-endian issue
    /// It is just experimental for Big Endian CPUs. So, you are not encouraged
    /// to use it for Big Endian CPUs for serious purpose. Only use this crate
    /// for Big-endian CPUs with your own full responsibility.
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// define_utypes_with!(u16);
    /// let mut rand = Any_SHA1::new();
    /// let num:U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("512-bit Random Prime Number = {}", num);
    /// assert!(num.is_odd());
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
            + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
            + Rem<Output=T> + RemAssign
            + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
            + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
            + BitXor<Output=T> + BitXorAssign + Not<Output=T>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }
}
