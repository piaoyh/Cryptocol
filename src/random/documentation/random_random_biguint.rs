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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
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
    /// let mut rand = Random_BIG_KECCAK_1024::new();
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
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// println!("Slapdash number = {}", slapdash.random_usize());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// println!("Slapdash number = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// println!("Slapdash number = {}", slapdash.random_u16());
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
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let num: U256 = slapdash.random_odd_biguint();
    /// println!("Slapdash number = {}", num);
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
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
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA2_256;
    /// 
    /// let mut any = AnyGen::new_with(SHA2_256::new(), SHA2_256::new());
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// 
    /// let mut slapdashGen = SlapdashGen::new_with(SHA1::new(), SHA0::new());
    /// println!("Any number = {}", slapdashGen.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::hash::{ MD5, MD4 };
    /// 
    /// let mut slapdashGen = SlapdashGen::new_with(MD5::new(), MD4::new());
    /// println!("Any number = {}", slapdashGen.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::random::RandGen;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut rand = RandGen::new_with(AES_128::new(), AES_128::new());
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::symmetric::DES;
    /// 
    /// let mut slapdashGen = SlapdashGen::new_with(DES::new(), DES::new());
    /// println!("Any number = {}", slapdashGen.random_u8());
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
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
    /// println!("Any number = {}", num);
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
    /// # Example 11 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// 
    /// let mut slapdash = Slapdash_SHA1::new_with_seeds(18782, 50558);
    /// println!("Slapdash number = {}", slapdash.random_uint::<u8>());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// 
    /// let mut slapdash = Slapdash_SHA0::new_with_seeds(0, 125);
    /// println!("Slapdash prime number = {}", slapdash.random_prime_using_miller_rabin_uint::<u128>(5));
    /// ```
    /// 
    /// # Example 13 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// 
    /// let mut slapdash = Slapdash_MD5::new_with_seeds(58, 161);
    /// println!("Slapdash number = {}", slapdash.random_u128());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// 
    /// let mut slapdash = Slapdash_MD4::new_with_seeds(106842379157284697, 18446744073709551615);
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 14 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Random number = {}", rand.random_u32());
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
    /// # Example 16 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// 
    /// let mut slapdash = Slapdash_DES::new_with_seeds(u8::MAX as u64, u8::MAX as u64);
    /// println!("Slapdash number = {}", slapdash.random_u8());
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::RandGen;
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
    /// use cryptocol::random::AnyGen;
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
    /// use cryptocol::random::AnyGen;
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
    /// use cryptocol::random::RandGen;
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
    /// use cryptocol::random::AnyGen;
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
    /// use cryptocol::random::AnyGen;
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
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::hash::SHA2_256;
    /// 
    /// let mut any = AnyGen::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), 15698731215687456325, 10684237915728469725);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// 
    /// let mut slapdashGen = SlapdashGen::new_with_generators_seeds(SHA1::new(), SHA0::new(), 2879054410500759758, 15887876257513809619);
    /// println!("SlapdashGen number = {}", slapdashGen.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::hash::{ MD5, MD4 };
    /// 
    /// let mut slapdashGen = AnyGen::new_with_generators_seeds(MD5::new(), MD4::new(), 610458805, 215793685);
    /// println!("SlapdashGen number = {}", slapdashGen.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::random::RandGen;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// let mut rand = RandGen::new_with_generators_seeds(AES_128::new(), AES_128::new(), 18782, 50558);
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::symmetric::AES_128;
    /// 
    /// use cryptocol::symmetric::DES;
    /// let mut slapdashGen = SlapdashGen::new_with_generators_seeds(DES::new(), DES::new(), 0, 125);
    /// println!("SlapdashGen number = {}", slapdashGen.random_u8());
    /// ```
    pub fn new_with_generators_seeds<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed: u64, aux: u64) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_generators_seed_arrays<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed: [u64; 8], aux: [u64; 8]) -> Self
    /// Constructs a new `Random_Generic` object with two random
    /// number generator engines and two seed arrays of type `u64` given.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// - You are highly recommended to use this method rather than the method
    ///   new_with_generators_seeds for security reason. It is because the
    ///   default seed collector function collects 1024 bits as a seed. If you
    ///   use this method, it results that you give full '1024' bits (= '64'
    ///   bits X '8' X '2') as a seed and it is equivalent to use a seed
    ///   collector function.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::{ RandGen, AnyGen, SlapdashGen };
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed, aux);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed, aux);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::hash::SHA2_512;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux);
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed, aux);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed, aux);
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed, aux);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed, aux);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(SHA1::new(), SHA0::new(), seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::hash::{ MD5, MD4 };
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(MD5::new(), MD4::new(), seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux);
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::symmetric::DES;
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_arrays(DES::new(), DES::new(), seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u8());
    /// ```
    pub fn new_with_generators_seed_arrays<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed: [u64; 8], aux: [u64; 8]) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_generators_seed_collector<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8]) -> Self
    /// Constructs a new `Random_Generic` object
    /// with two random number generator engines and a seed collector method.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Features
    /// - The default seed collector function is provided in this module,
    ///   but it is optimized for Unix/Linux though it works under Windows too.
    /// - If you use this crate under Windows and/or you have a better one,
    ///   you can use your own seed collector function by replacing the default
    ///   seed collector function with your own one. 
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector(SHA3_512::new(), SHA3_512::new(), seed_collector);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::hash::SHA2_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector(SHA2_512::new(), SHA2_512::new(),seed_collector);
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector(SHAKE_256::new(), SHAKE_256::new(), seed_collector);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector(SHAKE_128::new(), SHAKE_128::new(), seed_collector);
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector(SHA3_256::new(), SHA3_256::new(), seed_collector);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// use cryptocol::random::AnyGen;
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector(SHA2_256::new(), SHA2_256::new(), seed_collector);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// use cryptocol::random::SlapdashGen;
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector(SHA1::new(), SHA0::new(), seed_collector);
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::hash::{ MD5, MD4 };
    /// use cryptocol::random::SlapdashGen;
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector(MD5::new(), MD4::new(), seed_collector);
    /// println!("Slapdash number = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// use cryptocol::random::RandGen;
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector);
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::symmetric::DES;
    /// use cryptocol::random::SlapdashGen;
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector(DES::new(), DES::new(), seed_collector);
    /// println!("Slapdash number = {}", slapdash.random_u8());
    /// ```
    pub fn new_with_generators_seed_collector<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8]) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_generators_seed_collector_seeds<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Self
    /// Constructs a new `Random_Generic` object with two random number
    /// generator engines, a seed collector function, and two seeds
    /// of type `u64` given.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of `u64`.
    /// - `aux` is the seed number of `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Features
    /// - The default seed collector function is provided in this module,
    ///   but it is optimized for Unix/Linux though it works under Windows too.
    /// - If you use this crate under Windows and/or you have a better one,
    ///   you can use your own seed collector function by replacing the default
    ///   seed collector function with your own one.
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// - You are highly recommended to use the method
    ///   `new_with_generators_seed_collector_seed_arrays()` rather than this
    ///   method for security reason. It is because the default seed collector
    ///   function collects 1024 bits as a seed. If you use this method, it
    ///   results that you give only '128' bits (= '64' bits + '64' bits) as a
    ///   seed and the other '896' bits will be made out of the '128' bits that
    ///   you provided.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector, 10500872879054459758_u64, 15887751380961987625_u64);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA3_512::new(), SHA3_512::new(), seed_collector, 100, 25);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::hash::SHA2_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA2_512::new(), SHA2_512::new(), seed_collector, 0, 0);
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector_seeds(SHAKE_256::new(), SHAKE_256::new(), seed_collector, u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHAKE_128::new(), SHAKE_128::new(), seed_collector, 123456789, 987654321);
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA3_256::new(), SHA3_256::new(), seed_collector, u32::MAX as u64, u32::MAX as u64);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = AnyGen::new_with_generators_seed_collector_seeds(SHA2_256::new(), SHA2_256::new(), seed_collector, 15698731215687456325, 10684237915728469725);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 and SHA0
    /// ```
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(SHA1::new(), SHA0::new(), seed_collector, 2879054410500759758, 15887876257513809619);
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::hash::{ MD5, MD4 };
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(MD5::new(), MD4::new(), seed_collector, 610458805, 215793685);
    /// println!("Slapdash number = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = RandGen::new_with_generators_seed_collector_seeds(AES_128::new(), AES_128::new(), seed_collector, 18782, 50558);
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::symmetric::DES;
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seeds(DES::new(), DES::new(), seed_collector, 0, 125);
    /// println!("Slapdash number = {}", slapdash.random_u8());
    /// ```
    pub fn new_with_generators_seed_collector_seeds<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn new_with_generators_seed_collector_seed_arrays<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Self
    /// Constructs a new `Random_Generic` object with two random
    /// number generator engines and two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `main_generator` is a main random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating main pseudo-random numbers.
    /// - `aux_generator` is an auxiliary random number generator engine
    ///   which is of `Random_Engine`-type and
    ///   for generating auxiliary pseudo-random numbers to use in
    ///   generating the main pseudo-random numbers.
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Features
    /// - The default seed collector function is provided in this module,
    ///   but it is optimized for Unix/Linux though it works under Windows too.
    /// - If you use this crate under Windows and/or you have a better one,
    ///   you can use your own seed collector function by replacing the default
    ///   seed collector function with your own one.
    /// - You are highly recommended to use this method rather than the method
    ///   new_with_generators_seed_collector_seeds for security reason. It is
    ///   because the default seed collector function collects 1024 bits as a
    ///   seed. If you use this method, it results that you give full '1024'
    ///   bits (= '64' bits X '8' X '2') as a seed and it is equivalent to use
    ///   a seed collector function
    /// 
    /// # Panics
    /// If `COUNT` is `0`, this method will panic!
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Example 1 for BIG_KECCAK_1024
    /// ```
    /// use cryptocol::hash::BIG_KECCAK_1024;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed_collector, seed, aux);
    /// let num: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for SHA3_512
    /// ```
    /// use cryptocol::hash::SHA3_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed_collector, seed, aux);
    /// let num: U256 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for SHA2_512
    /// ```
    /// use cryptocol::hash::SHA2_512;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux);
    /// let num: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 4 for SHAKE_256
    /// ```
    /// use cryptocol::hash::SHAKE_256;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed_collector, seed, aux);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for SHAKE_128
    /// ```
    /// use cryptocol::hash::SHAKE_128;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed_collector, seed, aux);
    /// let num: U512 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 6 for SHA3_256
    /// ```
    /// use cryptocol::hash::SHA3_256;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed_collector, seed, aux);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 7 for SHA2_256
    /// ```
    /// use cryptocol::hash::SHA2_256;
    /// use cryptocol::random::AnyGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut any = AnyGen::new_with_generators_seed_collector_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed_collector, seed, aux);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 8 for SHA1 amd SHA0
    /// ```
    /// use cryptocol::hash::{ SHA1, SHA0 };
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(SHA1::new(), SHA0::new(), seed_collector, seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 9 for MD5 and MD4
    /// ```
    /// use cryptocol::hash::{ MD5, MD4 };
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(MD5::new(), MD4::new(), seed_collector, seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 10 for AES_128
    /// ```
    /// use cryptocol::symmetric::AES_128;
    /// use cryptocol::random::RandGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut rand = RandGen::new_with_generators_seed_collector_seed_arrays(AES_128::new(), AES_128::new(), seed_collector, seed, aux);
    /// println!("Random number = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 11 for DES
    /// ```
    /// use cryptocol::symmetric::DES;
    /// use cryptocol::random::SlapdashGen;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let seed = [10500872879054459758_u64, 14597581050087285790, 10790544591050087758, 17281050044597905758, 15900810579072854758, 10572800879059744558, 13758728710500905448, 15054105075808728459];
    /// let aux = [10500054459758872879_u64, 15810500854459728790, 10790877585445910500, 10044597872810905758, 10579072855900814758, 14410572800879059558, 17105448597050095872, 18087279054105078459];
    /// let mut slapdash = SlapdashGen::new_with_generators_seed_collector_seed_arrays(DES::new(), DES::new(), seed_collector, seed, aux);
    /// println!("Slapdash number = {}", slapdash.random_u8());
    /// ```
    pub fn new_with_generators_seed_collector_seed_arrays<SG, AG>(mut main_generator: SG, mut aux_generator: AG, seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Self
    where SG: Random_Engine + 'static, AG: Random_Engine + 'static
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn get_seed_collector(&self) -> fn() -> [u64; 8]
    /// Returns the function pointer to the current seed collector function.
    /// 
    /// # Output
    /// It returns the function pointer to the current seed collector function,
    /// and is of the type `fn() -> [u64; 8]`.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let rand = Random::new();
    /// let seed = rand.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let any = Any::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let rand = Random_BIG_KECCAK_1024::new();
    /// let seed = rand.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let rand = Random_SHA3_512::new();
    /// let seed = rand.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let rand = Random_SHA2_512::new();
    /// let seed = rand.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let any = Any_SHAKE_256::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let any = Any_SHAKE_128::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let any = Any_SHA3_512::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let any = Any_SHA3_256::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let any = Any_SHA2_512::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let any = Any_SHA2_256::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let slapdash = Slapdash_SHA1::new();
    /// let seed = slapdash.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let slapdash = Slapdash_SHA0::new();
    /// let seed = slapdash.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let slapdash = Slapdash_MD5::new();
    /// let seed = slapdash.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let slapdash = Slapdash_MD4::new();
    /// let seed = slapdash.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let rand = Random_Rijndael::new();
    /// let seed = rand.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let any = Any_Rijndael::new();
    /// let seed = any.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///     { print!("{} ", seed[i]); }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let slapdash = Slapdash_DES::new();
    /// let seed = slapdash.get_seed_collector()();
    /// print!("seed = ");
    /// for i in 0..8
    ///    { print!("{} ", seed[i]); }
    /// ```
    pub fn get_seed_collector(&self) -> fn() -> [u64; 8]
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn set_seed_collector(&mut self, collect_seed: fn() -> [u64; 8])
    /// Replaces the default seed collector function
    /// with new seed collector function.
    /// 
    /// # Arguments
    /// `collect_seed` is a new seed collector function to replace the default
    /// seed collector function, and is of the type `fn() -> [u64; 8]`.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut rand = Random::new();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// println!("seed = {}", rand.random_u128());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// println!("seed = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// println!("seed = {}", rand.random_u16());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// println!("seed = {}", rand.random_u8());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u32());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u16());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u8());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u128());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// println!("seed = {}", slapdash.random_u64());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// println!("seed = {}", slapdash.random_u32());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// println!("seed = {}", slapdash.random_u16());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// println!("seed = {}", slapdash.random_u8());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// println!("seed = {}", rand.random_u128());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut any = Any_Rijndael::new();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// println!("seed = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     use std::time::{ SystemTime, UNIX_EPOCH };
    ///     use cryptocol::number::LongerUnion;
    /// 
    ///     let ptr = seed_collector as *const fn() -> [u64; 8] as u64;
    ///     let mut seed_buffer = [ptr; 8];
    ///     for i in 0..8
    ///         { seed_buffer[i] ^= ptr.swap_bytes().rotate_left(i as u32); }
    /// 
    ///     if let Ok(nanos) = SystemTime::now().duration_since(UNIX_EPOCH)
    ///     {
    ///         let common = LongerUnion::new_with(nanos.as_nanos());
    ///         for i in 0..4
    ///         {
    ///             let j = i << 1;
    ///             seed_buffer[j] = common.get_ulong_(0);
    ///             seed_buffer[j + 1] = common.get_ulong_(1);
    ///         }
    ///     }
    ///     seed_buffer
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// println!("seed = {}", slapdash.random_u32());
    /// ```
    pub fn set_seed_collector(&mut self, seed_collector: fn() -> [u64; 8])
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn reset_seed_collector(&mut self)
    /// Replace the current seed collector function
    /// with the default seed collector function.
    /// 
    /// # Features
    /// After this method is performed, the previous seed collector function
    /// will be lost.
    /// 
    /// # Example 1 for Random
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let collector = rand.get_seed_collector();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// rand.reset_seed_collector();
    /// assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let collector = rand.get_seed_collector();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// rand.reset_seed_collector();
    /// assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let collector = rand.get_seed_collector();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// rand.reset_seed_collector();
    /// assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let collector = rand.get_seed_collector();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// rand.reset_seed_collector();
    /// assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let collector = slapdash.get_seed_collector();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// slapdash.reset_seed_collector();
    /// assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let collector = slapdash.get_seed_collector();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// slapdash.reset_seed_collector();
    /// assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let collector = slapdash.get_seed_collector();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// slapdash.reset_seed_collector();
    /// assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let collector = slapdash.get_seed_collector();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// slapdash.reset_seed_collector();
    /// assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let collector = rand.get_seed_collector();
    /// rand.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, rand.get_seed_collector() as Func);
    /// rand.reset_seed_collector();
    /// assert_eq!(collector as Func, rand.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let collector = any.get_seed_collector();
    /// any.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, any.get_seed_collector() as Func);
    /// any.reset_seed_collector();
    /// assert_eq!(collector as Func, any.get_seed_collector() as Func);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// fn seed_collector() -> [u64; 8]
    /// {
    ///     [0_u64; 8]
    /// }
    /// type Func = *const fn() -> [u64; 8];
    /// 
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let collector = slapdash.get_seed_collector();
    /// slapdash.set_seed_collector(seed_collector);
    /// assert_eq!(seed_collector as Func, slapdash.get_seed_collector() as Func);
    /// slapdash.reset_seed_collector();
    /// assert_eq!(collector as Func, slapdash.get_seed_collector() as Func);
    /// ```
    pub fn reset_seed_collector(&mut self)
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let num: [u128; 5] = rand.random_array();
    /// for i in 0..5
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let num: [u64; 10] = any.random_array();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let num: [u32; 16] = rand.random_array();
    /// for i in 0..16
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let num: [u16; 20] = rand.random_array();
    /// for i in 0..20
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let num: [u8; 32] = rand.random_array();
    /// for i in 0..32
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let num: [usize; 10] = any.random_array();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let num: [u128; 4] = any.random_array();
    /// for i in 0..4
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let num: [u64; 10] = any.random_array();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let num: [u32; 16] = any.random_array();
    /// for i in 0..16
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let num: [u16; 10] = any.random_array();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let num: [u8; 8] = any.random_array();
    /// for i in 0..8
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let num: [usize; 16] = slapdash.random_array();
    /// for i in 0..16
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let num: [u128; 16] = slapdash.random_array();
    /// for i in 0..16
    ///     { println!("slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let num: [u64; 8] = slapdash.random_array();
    /// for i in 0..8
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let num: [u32; 4] = slapdash.random_array();
    /// for i in 0..4
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let num: [u16; 5] = rand.random_array();
    /// for i in 0..5
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let num: [u8; 10] = any.random_array();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let num: [u128; 4] = slapdash.random_array();
    /// for i in 0..4
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let num: [u64; 16] = slapdash.random_array();
    /// for i in 0..16
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let num: [u32; 8] = slapdash.random_array();
    /// for i in 0..8
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    pub fn random_array<T, const N: usize>(&mut self) -> [T; N]
    where T: SmallUInt
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// If you want random BigUInt, you are highly recommended
    /// to use the method
    /// [random_biguint()](struct@Random_Generic#method.random_biguint)
    /// rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// let mut rand = Random::new();
    /// let mut num = [0_u128; 5];
    /// rand.put_random_in_array(&mut num);
    /// for i in 0..5
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// let mut any = Any::new();
    /// let mut num = [0_u64; 10];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let mut num = [0_u32; 16];
    /// rand.put_random_in_array(&mut num);();
    /// for i in 0..16
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// let mut rand = Random_SHA3_512::new();
    /// let mut num = [0_u16; 20];
    /// rand.put_random_in_array(&mut num);();
    /// for i in 0..20
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// let mut rand = Random_SHA2_512::new();
    /// let mut num = [0_u8; 32];
    /// rand.put_random_in_array(&mut num);();
    /// for i in 0..32
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// let mut any = Any_SHAKE_256::new();
    /// let mut num = [0_usize; 10];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// let mut any = Any_SHAKE_128::new();
    /// let mut num = [0_u128; 4];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..4
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// let mut any = Any_SHA3_512::new();
    /// let mut num = [0_u64; 10];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// let mut any = Any_SHA3_256::new();
    /// let mut num = [0_u32; 16];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..16
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// let mut any = Any_SHA2_512::new();
    /// let mut num = [0_u16; 10];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// let mut any = Any_SHA2_256::new();
    /// let mut num = [0_u8; 8];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..8
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let mut num = [0_usize; 16];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..16
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let mut num = [0_u128; 16];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..16
    ///     { println!("slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// let mut slapdash = Slapdash_MD5::new();
    /// let mut num = [0_u64; 8];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..8
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// let mut slapdash = Slapdash_MD4::new();
    /// let mut num = [0_u32; 4];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..4
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// let mut rand = Random_Rijndael::new();
    /// let mut num = [0_u16; 5];
    /// rand.put_random_in_array(&mut num);();
    /// for i in 0..5
    ///     { println!("Random number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// let mut any = Any_Rijndael::new();
    /// let mut num = [0_u8; 10];
    /// any.put_random_in_array(&mut num);();
    /// for i in 0..10
    ///     { println!("Any number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// let mut slapdash = Slapdash_DES::new();
    /// let mut num = [0_u128; 4];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..4
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let mut num = [0_u64; 16];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..16
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    ///     
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// let mut slapdash = Slapdash::new();
    /// let mut num = [0_u32; 8];
    /// slapdash.put_random_in_array(&mut num);();
    /// for i in 0..8
    ///     { println!("Slapdash number {} => {}", i, num[i]); }
    /// ```
    pub fn put_random_in_array<T, const N: usize>(&mut self, out: &mut [T; N])
    where T: SmallUInt
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to
    ///   use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly recommended to
    ///   use the method [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to
    ///   use the method [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random::new();
    /// let biguint: U256 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any::new();
    /// let biguint: U384 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let biguint: U512 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let biguint: U768 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let biguint: U1024 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let biguint: U2048 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let biguint: U3072 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let biguint: U4096 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let biguint: U5120 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let biguint: U6144 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let biguint: U7168 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let biguint: U8192 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let biguint: U16384 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let biguint: U256 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let biguint: U384 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let biguint: U512 = rand.random_biguint();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let biguint: U768 = any.random_biguint();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let biguint: U1024 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let biguint: U2048 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let biguint: U3072 = slapdash.random_biguint();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    #[inline]
    pub fn random_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of type `&BigUInt<T, N>`.
    /// 
    /// # Output
    /// A random number wrapped by enum `Some` of `Option`, whose range is
    /// between 0 inclusively and the certain value exclusively when `ceiling`
    /// is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    ///   the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random::new();
    /// let ceiling = U16384::max().wrapping_div_uint(3_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any::new();
    /// let ceiling = U8192::max().wrapping_div_uint(4_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let ceiling = U7168::max().wrapping_div_uint(5_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let ceiling = U6144::max().wrapping_div_uint(6_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let ceiling = U5120::max().wrapping_div_uint(7_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let ceiling = U4096::max().wrapping_div_uint(8_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let ceiling = U3072::max().wrapping_div_uint(9_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let ceiling = U2048::max().wrapping_div_uint(10_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let ceiling = U1024::max().wrapping_div_uint(11_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let ceiling = U768::max().wrapping_div_uint(12_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let ceiling = U512::max().wrapping_div_uint(13_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let ceiling = U384::max().wrapping_div_uint(14_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let ceiling = U256::max().wrapping_div_uint(15_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let ceiling = U16384::max().wrapping_div_uint(16_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let ceiling = U8192::max().wrapping_div_uint(17_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let ceiling = U7168::max().wrapping_div_uint(18_u8);
    /// if let Some(r) = rand.random_under_biguint(&ceiling)
    /// {
    ///     println!("Random Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let ceiling = U6144::max().wrapping_div_uint(19_u8);
    /// if let Some(r) = any.random_under_biguint(&ceiling)
    /// {
    ///     println!("Any Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let ceiling = U5120::max().wrapping_div_uint(20_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let ceiling = U4096::max().wrapping_div_uint(21_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let ceiling = U2048::max().wrapping_div_uint(22_u8);
    /// if let Some(r) = slapdash.random_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    /// }
    /// ```
    #[inline]
    pub fn random_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random
    /// value less than a certain value.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of type `&BigUInt<T, N>`.
    /// 
    /// # Output
    /// The random number whose range is between 0 inclusively
    /// and the certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it by
    ///   the certain value to get its remainder.
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random::new();
    /// let ceiling = U16384::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 2 for Random
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any::new();
    /// let ceiling = U8192::max().wrapping_div_uint(4_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let ceiling = U7168::max().wrapping_div_uint(5_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let ceiling = U6144::max().wrapping_div_uint(6_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let ceiling = U5120::max().wrapping_div_uint(7_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let ceiling = U4096::max().wrapping_div_uint(8_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let ceiling = U3072::max().wrapping_div_uint(9_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let ceiling = U2048::max().wrapping_div_uint(10_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let ceiling = U1024::max().wrapping_div_uint(11_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let ceiling = U768::max().wrapping_div_uint(12_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let ceiling = U512::max().wrapping_div_uint(13_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let ceiling = U384::max().wrapping_div_uint(14_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let ceiling = U256::max().wrapping_div_uint(15_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let ceiling = U16384::max().wrapping_div_uint(16_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let ceiling = U8192::max().wrapping_div_uint(17_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let ceiling = U7168::max().wrapping_div_uint(18_u8);
    /// let r = rand.random_under_biguint_(&ceiling);
    /// println!("Random Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let ceiling = U6144::max().wrapping_div_uint(19_u8);
    /// let r = any.random_under_biguint_(&ceiling);
    /// println!("Any Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let ceiling = U5120::max().wrapping_div_uint(20_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let ceiling = U4096::max().wrapping_div_uint(21_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let ceiling = U2048::max().wrapping_div_uint(22_u8);
    /// let r = slapdash.random_under_biguint_(&ceiling);
    /// println!("Slapdash Number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// ```
    #[inline]
    pub fn random_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt
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
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random number,
    ///   you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random::new();
    /// let r: U16384 = rand.random_odd_biguint();
    /// println!("Random odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any::new();
    /// let r: U8192 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let r: U7168 = rand.random_odd_biguint();
    /// println!("Random odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let r: U6144 = rand.random_odd_biguint();
    /// println!("Random odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let r: U5120 = rand.random_odd_biguint();
    /// println!("Random odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let r: U4096 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let r: U3072 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let r: U2048 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let r: U1024 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let r: U768 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let r: U512 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let r: U384 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let r: U256 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let r: U16384 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let r: U8192 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let r: U7168 = rand.random_odd_biguint();
    /// println!("Random odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let r: U6144 = any.random_odd_biguint();
    /// println!("Any odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let r: U5120 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let r: U4096 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let r: U3072 = slapdash.random_odd_biguint();
    /// println!("Slapdash odd number is {}.", r);
    /// assert!(r.is_odd());
    /// ```
    pub fn random_odd_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value, wrapped by enum `Some` of `Option`.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of type `&BigUInt<T, N>`.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively, wrapped by enum `Some` of `Option` if
    /// `ceiling` is not zero. If `ceiling` is zero, `None` will be returned.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    ///   by the certain value to get its remainder and then simply set the LSB
    ///   (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random::new();
    /// let ceiling = U16384::max().wrapping_div_uint(3_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any::new();
    /// let ceiling = U8192::max().wrapping_div_uint(4_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let ceiling = U7168::max().wrapping_div_uint(5_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let ceiling = U6144::max().wrapping_div_uint(6_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let ceiling = U5120::max().wrapping_div_uint(7_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let ceiling = U4096::max().wrapping_div_uint(8_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let ceiling = U3072::max().wrapping_div_uint(9_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let ceiling = U2048::max().wrapping_div_uint(10_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let ceiling = U1024::max().wrapping_div_uint(11_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let ceiling = U768::max().wrapping_div_uint(12_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let ceiling = U512::max().wrapping_div_uint(13_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let ceiling = U384::max().wrapping_div_uint(14_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let ceiling = U256::max().wrapping_div_uint(15_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let ceiling = U16384::max().wrapping_div_uint(16_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let ceiling = U8192::max().wrapping_div_uint(17_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let ceiling = U7168::max().wrapping_div_uint(18_u8);
    /// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Random odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let ceiling = U6144::max().wrapping_div_uint(19_u8);
    /// if let Some(r) = any.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Any odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let ceiling = U5120::max().wrapping_div_uint(20_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let ceiling = U4096::max().wrapping_div_uint(21_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let ceiling = U2048::max().wrapping_div_uint(22_u8);
    /// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
    /// {
    ///     println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    ///     assert!(r < ceiling);
    ///     assert!(r.is_odd());
    /// }
    /// ```
    #[inline]
    pub fn random_odd_under_biguint<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> Option<BigUInt<T, N>>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which has the random odd
    /// value less than a certain value.
    /// 
    /// # Argument
    /// The argument `ceiling` is the upper limitation which the generated
    /// random number should be less than, and is of type `&BigUInt<T, N>`.
    /// 
    /// # Output
    /// The random odd number whose range is between 0 inclusively and the
    /// certain value exclusively.
    /// 
    /// # Features
    /// - This method generates a random number, and then simply divides it
    ///   by the certain value to get its remainder and then simply set the LSB
    ///   (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Panics
    /// If `ceiling` is zero, this method will panic.
    /// 
    /// # Caution
    /// Use this method only when you are sure that `ceiling` is not zero.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to
    ///   use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random::new();
    /// let ceiling = U16384::max().wrapping_div_uint(3_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any::new();
    /// let ceiling = U8192::max().wrapping_div_uint(4_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let ceiling = U7168::max().wrapping_div_uint(5_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let ceiling = U6144::max().wrapping_div_uint(6_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let ceiling = U5120::max().wrapping_div_uint(7_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let ceiling = U4096::max().wrapping_div_uint(8_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let ceiling = U3072::max().wrapping_div_uint(9_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let ceiling = U2048::max().wrapping_div_uint(10_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let ceiling = U1024::max().wrapping_div_uint(11_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let ceiling = U768::max().wrapping_div_uint(12_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let ceiling = U512::max().wrapping_div_uint(13_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 12for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let ceiling = U384::max().wrapping_div_uint(14_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let ceiling = U256::max().wrapping_div_uint(15_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let ceiling = U16384::max().wrapping_div_uint(16_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let ceiling = U8192::max().wrapping_div_uint(17_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let ceiling = U7168::max().wrapping_div_uint(18_u8);
    /// let r = rand.random_odd_under_biguint_(&ceiling);
    /// println!("Random odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let ceiling = U6144::max().wrapping_div_uint(19_u8);
    /// let r = any.random_odd_under_biguint_(&ceiling);
    /// println!("Any odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let ceiling = U5120::max().wrapping_div_uint(20_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let ceiling = U4096::max().wrapping_div_uint(21_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let ceiling = U2048::max().wrapping_div_uint(22_u8);
    /// let r = slapdash.random_odd_under_biguint_(&ceiling);
    /// println!("Slapdash odd number less than {} is\n{}", ceiling, r);
    /// assert!(r < ceiling);
    /// assert!(r.is_odd());
    /// ```
    #[inline]
    pub fn random_odd_under_biguint_<T, const N: usize>(&mut self, ceiling: &BigUInt<T, N>) -> BigUInt<T, N>
    where T: SmallUInt
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
    ///   (Most Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to
    ///   use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to
    ///   use the method [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random::new();
    /// let r: u64384 = rand.random_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > u64384::halfmax());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any::new();
    /// let r: U8192 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U8192::halfmax());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let r: U7168 = rand.random_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U7168::halfmax());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let r: U6144 = rand.random_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U6144::halfmax());
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let r: U5120 = rand.random_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U5120::halfmax());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let r: U4096 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U4096::halfmax());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let r: U3072 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U3072::halfmax());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let r: U2048 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U2048::halfmax());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let r: U1024 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U1024::halfmax());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let r: U768 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U768::halfmax());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let r: U512 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U512::halfmax());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let r: U384 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U384::halfmax());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let r: U256 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U256::halfmax());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let r: u64384 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > u64384::halfmax());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let r: U8192 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U8192::halfmax());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let r: U7168 = rand.random_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U7168::halfmax());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let r: U6144 = any.random_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U6144::halfmax());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let r: U5120 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U5120::halfmax());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let r: U4096 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U4096::halfmax());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let r: U3072 = slapdash.random_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U3072::halfmax());
    /// ```
    #[inline]
    pub fn random_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
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
    ///   (Most Significant Bit) and LSB (Least Significant Bit).
    /// - The random numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Cryptographical Security
    /// - If you use either `Random_*` or `Any_*`, it is considered to be
    ///   cryptographically secure.
    /// - If you use `Slapdash_*`, it is considered that it may be
    ///   cryptographically insecure.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random::new();
    /// let r: U16384 = rand.random_odd_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U16384::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any::new();
    /// let r: U8192 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U8192::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let r: U7168 = rand.random_odd_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U7168::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let r: U6144 = rand.random_odd_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U6144::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 5 for RanRandom_SHA2_512dom
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let r: U5120 = rand.random_odd_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U5120::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let r: U4096 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U4096::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let r: U3072 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U3072::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let r: U2048 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U2048::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let r: U1024 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U1024::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let r: U768 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U768::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let r: U512 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U512::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let r: U384 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U384::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let r: U256 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U256::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let r: U16384 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U16384::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let r: U8192 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U8192::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let r: U7168 = rand.random_odd_with_msb_set_biguint();
    /// println!("Random number is {}.", r);
    /// assert!(r > U7168::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let r: U6144 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number is {}.", r);
    /// assert!(r > U6144::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let r: U5120 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U5120::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let r: U4096 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U4096::halfmax());
    /// assert!(r.is_odd());
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let r: U3072 = slapdash.random_odd_with_msb_set_biguint();
    /// println!("Slapdash number is {}.", r);
    /// assert!(r > U3072::halfmax());
    /// assert!(r.is_odd());
    /// ```
    pub fn random_odd_with_msb_set_biguint<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// `5` for 99.9% accuracy or `7` for 99.99% accuracy.
    /// 
    /// # Output
    /// The random prime number that this method random_prime_Miller_Rabin()
    /// returns is a random prime number whose range is from
    /// 2 up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - It uses
    ///   [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random prime
    ///   number, you are highly recommended to use the method
    ///   [random_prime_with_msb_set_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_with_msb_set_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random::new();
    /// let prime: U256 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any::new();
    /// let prime: U384 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let prime: U512 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let prime: U768 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let prime: U1024 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let prime: U2048 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let prime: U3072 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let prime: U4096 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let prime: U5120 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let prime: U6144 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let prime: U7168 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let prime: U8192 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let prime: U16384 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let prime: U256 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 13 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let prime: U384 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 14 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let prime: U512 = rand.random_prime_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 15 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let prime: U768 = any.random_prime_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let prime: U1024 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let prime: U2048 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let prime: U3072 = slapdash.random_prime_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    pub fn random_prime_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number of full-size of BigUInt<T, N>.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// `5` for 99.9% accuracy or `7` for 99.99% accuracy.
    /// 
    /// # Output
    /// The random prime numbers which ranges from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method uses concurrency.
    /// - This method generates a random number, and then simply sets its MSB
    ///   (Most Significant Bit) to be one, and then checks whether or not the
    ///   generated random number is prime number, and then it repeats until it
    ///   will generate a prime number.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// 
    /// # Counterpart Methods
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random::new();
    /// let prime: U256 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any::new();
    /// let prime: U384 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let prime: U768 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let prime: U1024 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let prime: U2048 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let prime: U3072 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let prime: U4096 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let prime: U5120 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let prime: U6144 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let prime: U7168 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let prime: U8192 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let prime: U16384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let prime: U256 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let prime: U384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let prime: U768 = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let prime: U1024 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let prime: U2048 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let prime: U3072 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_msb_set_using_miller_rabin_biguint_sequentially<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a random
    /// prime number of full-size of BigUInt<T, N>.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// `5` for 99.9% accuracy or `7` for 99.99% accuracy.
    /// 
    /// # Output
    /// The random prime numbers which ranges from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method does not use concurrency.
    /// - This method generates a random number, and then simply sets its MSB
    ///   (Most Significant Bit) to be one, and then checks whether or not the
    ///   generated random number is prime number, and then it repeats until it
    ///   will generate a prime number.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random::new();
    /// let prime: U256 = rand.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any::new();
    /// let prime: U384 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let prime: U768 = rand.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let prime: U1024 = rand.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let prime: U2048 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let prime: U3072 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let prime: U4096 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let prime: U5120 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let prime: U6144 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let prime: U7168 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let prime: U8192 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let prime: U16384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let prime: U256 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let prime: U384 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let prime: U512 = rand.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let prime: U768 = any.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let prime: U1024 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let prime: U2048 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let prime: U3072 = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint_sequentially(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    pub fn random_prime_with_msb_set_using_miller_rabin_biguint_sequentially<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_primes_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize, how_many: usize) -> Vec<BigUInt<T, N>>
    /// Returns a collection of new `BigUInt<T, N>`-type objects which represent
    /// random prime numbers of full-size of BigUInt<T, N>.
    /// 
    /// # Argument
    /// - `repetition`: determines how many times it tests whether the
    ///   generated random number is prime. Usually, `repetition` is given
    ///   to be `5` for 99.9% accuracy or `7` for 99.99% accuracy.
    /// - `how_many`: determines how many prime numbers this method returns.
    /// 
    /// # Output
    /// The random prime number which ranges from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates several threads, each of which checks whether or
    ///   not the given random number is prime number, and then it repeats until
    ///   it will find `how_many` prime numbers.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    pub fn random_primes_with_msb_set_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize, how_many: usize) -> Vec<BigUInt<T, N>>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_prime_with_half_length_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object which represents a 
    /// `N * T::size_in_bits() / 2`-bit random prime number.
    /// 
    /// # Argument
    /// The argument `repetition` defines how many times it tests whether the
    /// generated random number is prime. Usually, `repetition` is given to be
    /// 5 to have 99.9% accuracy.
    /// 
    /// # Output
    /// This method returns a random prime number whose length is
    /// `N * T::size_in_bits() / 2` bits.
    /// 
    /// # Features
    /// - This method generates a random number of half length, and then simply
    ///   sets all the bytes from `N * T::size_in_bits() / 2`-th bit to 
    ///   `N * T::size_in_bits() - 1`-th bit to be `zero`, and then checks
    ///   whether or not the generated random number is prime number, and then
    ///   it repeats until it will generate a prime number. Here, 0-th bit is
    ///   LSB (Least Significant Bit) and `N * T::size_in_bits() - 1`-th bit
    ///   is MSB (Most Significant Bit).
    /// - It uses
    ///   [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
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
    /// - If you want to use a normal random number, you are highly recommended
    ///   to use the method
    ///   [random_biguint()](struct@Random_Generic#method.random_biguint)
    ///   rather than this method.
    /// - If you want to use a random number less than a certain value, you are
    ///   highly recommended to use the method
    ///   [random_under_biguint()](struct@Random_Generic#method.random_under_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number, you are highly recommended to
    ///   use the method
    ///   [random_odd_biguint()](struct@Random_Generic#method.random_odd_biguint)
    ///   rather than this method.
    /// - If you want to use a random odd number less than a certain value,
    ///   you are highly recommended to use the method
    ///   [ranodm_odd_under_biguint()](struct@Random_Generic#method.ranodm_odd_under_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random
    ///   number, you are highly recommended to use the method
    ///   [random_with_msb_set_biguint()](struct@Random_Generic#method.random_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a `(N * sizeof::<T>() * 8)`-bit long random odd
    ///   number, you are highly recommended to use the method
    ///   [random_odd_with_msb_set_biguint()](struct@Random_Generic#method.random_odd_with_msb_set_biguint)
    ///   rather than this method.
    /// - If you want to use a normal random prime number, you are highly
    ///   recommended to use the method
    ///   [random_prime_using_miller_rabin_biguint()](struct@Random_Generic#method.random_prime_using_miller_rabin_biguint)
    ///   rather than this method.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random::new();
    /// let prime: U256 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any::new();
    /// let prime: U384 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let prime: U512 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let prime: U768 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let prime: U1024 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let prime: U2048 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let prime: U3072 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let prime: U4096 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let prime: U5120 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let prime: U6144 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let prime: U7168 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let prime: U8192 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let prime: U16384 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let prime: U256 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let prime: U384 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let prime: U512 = rand.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Random prime number: {}", prime);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let prime: U768 = any.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Any prime number: {}", prime);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let prime: U1024 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let prime: U2048 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let prime: U3072 = slapdash.random_prime_with_half_length_using_miller_rabin_biguint(5);
    /// println!("Slapdash prime number: {}", prime);
    /// ```
    pub fn random_prime_with_half_length_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn random_primes_with_half_length_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize, how_many: usize) -> Vec<BigUInt<T, N>>
    /// Returns a collection of new `BigUInt<T, N>`-type objects which represent
    /// random prime numbers of half-size of BigUInt<T, N>.
    /// 
    /// # Argument
    /// - `repetition`: determines how many times it tests whether the
    ///   generated random number is prime. Usually, `repetition` is given
    ///   to be `5` for 99.9% accuracy or `7` for 99.99% accuracy.
    /// - `how_many`: determines how many prime numbers this method returns.
    /// 
    /// # Output
    /// The random prime number which ranges from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method generates several threads, each of which checks whether or
    ///   not the given random number is prime number, and then it repeats until
    ///   it will find `how_many` prime numbers.
    /// - It uses [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number, the
    ///   probability that the tested number is not a prime number is 1/4. So,
    ///   if the test results in prime number twice, the probability that the
    ///   tested number is not a prime number is 1/16 (= 1/4 * 1/4). Therefore,
    ///   if you test any number 5 times and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// - The random prime numbers that may or may not be cryptographically
    ///   secure depending on what pseudo-random number generator is used.
    pub fn random_primes_with_half_length_using_miller_rabin_biguint<T, const N: usize>(&mut self, repetition: usize, how_many: usize) -> Vec<BigUInt<T, N>>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn prepared_random_prime_with_msb_set<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object out of the prepared prime
    /// number pool, which represents a prime number of full-size of
    /// BigUInt<T, N>.
    /// 
    /// # Output
    /// This method returns a prime number randomly chosen out of the prepared
    /// prime number pool consisting of many prime numbers, whose range is from
    /// BigUInt::halfmax() up to BigUInt::max() inclusively.
    /// 
    /// # Features
    /// - This method chooses a prime number ramdomly out of the prepared prime
    ///   number pool consisting of many prime numbers whose MSB (Most
    ///   Significant Bit) is set to be one.
    /// - It uses a prime number pool.
    /// 
    /// # Caution and Cryptographical Security
    /// The source code of this method is openly publicised. It means that
    /// anyone can have the full familiarity of the prime number pool which
    /// this method uses. So, you are NOT encouraged to use this method for
    /// serious cryptographical purposes. You can use this method temporarily
    /// for testing or debugging purposes because of the slowness of the
    /// Millar-Rabin algorithm for big numbers.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random::new();
    /// let biguint: U256 = rand.prepared_random_prime_with_msb_set();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any::new();
    /// let biguint: U384 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let biguint: U512 = rand.prepared_random_prime_with_msb_set();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let biguint: U768 = rand.prepared_random_prime_with_msb_set();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let biguint: U1024 = rand.prepared_random_prime_with_msb_set();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let biguint: U2048 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let biguint: U3072 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let biguint: U4096 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let biguint: U5120 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let biguint: U6144 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let biguint: U7168 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let biguint: U8192 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let biguint: U16384 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let biguint: U256 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let biguint: U384 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let biguint: U512 = rand.prepared_random_prime_with_msb_set();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let biguint: U768 = any.prepared_random_prime_with_msb_set();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let biguint: U1024 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let biguint: U2048 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let biguint: U3072 = slapdash.prepared_random_prime_with_msb_set();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    #[inline]
    pub fn prepared_random_prime_with_msb_set<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn prepared_random_prime_with_half_length<T, const N: usize>(&mut self) -> BigUInt<T, N>
    /// Constucts a new `BigUInt<T, N>`-type object out of the prepared prime
    /// number pool, which represents a prime number of half-size of
    /// BigUInt<T, N>.
    /// 
    /// # Output
    /// This method returns a prime number randomly chosen out of the prepared
    /// prime number pool consisting of many prime numbers, whose length is
    /// `N * T::size_in_bits() / 2` bits.
    /// 
    /// # Features
    /// 
    /// - This method chooses a prime number ramdomly out of the prepared prime
    ///   number pool consisting of many prime numbers whose length is
    ///   `N * T::size_in_bits() / 2` bits.
    /// - It uses a prime number pool.
    /// 
    /// # Caution and Cryptographical Security
    /// The source code of this method is openly publicised. It means that
    /// anyone can have the full familiarity of the prime number pool which
    /// this method uses. So, you are NOT encouraged to use this method for
    /// serious cryptographical purposes. You can use this method temporarily
    /// for testing or debugging purposes because of the slowness of the
    /// Millar-Rabin algorithm for big numbers.
    /// 
    /// # Example 1 for Random
    /// ```
    /// use cryptocol::random::Random;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random::new();
    /// let biguint: U256 = rand.prepared_random_prime_with_half_length();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 2 for Any
    /// ```
    /// use cryptocol::random::Any;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any::new();
    /// let biguint: U384 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 3 for Random_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let biguint: U512 = rand.prepared_random_prime_with_half_length();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 4 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let biguint: U768 = rand.prepared_random_prime_with_half_length();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 5 for Random_SHA2_512
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_SHA2_512::new();
    /// let biguint: U1024 = rand.prepared_random_prime_with_half_length();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 6 for Any_SHAKE_256
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_256::new();
    /// let biguint: U2048 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 7 for Any_SHAKE_128
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHAKE_128::new();
    /// let biguint: U3072 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 8 for Any_SHA3_512
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_512::new();
    /// let biguint: U4096 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 9 for Any_SHA3_256
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA3_256::new();
    /// let biguint: U5120 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 10 for Any_SHA2_512
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_512::new();
    /// let biguint: U6144 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 11 for Any_SHA2_256
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_SHA2_256::new();
    /// let biguint: U7168 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 12 for Slapdash_SHA1
    /// ```
    /// use cryptocol::random::Slapdash_SHA1;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA1::new();
    /// let biguint: U8192 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 13 for Slapdash_SHA0
    /// ```
    /// use cryptocol::random::Slapdash_SHA0;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_SHA0::new();
    /// let biguint: U16384 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 14 for Slapdash_MD5
    /// ```
    /// use cryptocol::random::Slapdash_MD5;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD5::new();
    /// let biguint: U256 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 15 for Slapdash_MD4
    /// ```
    /// use cryptocol::random::Slapdash_MD4;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_MD4::new();
    /// let biguint: U384 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 16 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let biguint: U512 = rand.prepared_random_prime_with_half_length();
    /// println!("Random Number: {}", biguint);
    /// ```
    /// 
    /// # Example 17 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let biguint: U768 = any.prepared_random_prime_with_half_length();
    /// println!("Any Number: {}", biguint);
    /// ```
    /// 
    /// # Example 18 for Slapdash_DES
    /// ```
    /// use cryptocol::random::Slapdash_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_DES::new();
    /// let biguint: U1024 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 19 for Slapdash_Num_C
    /// ```
    /// use cryptocol::random::Slapdash_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash_Num_C::new();
    /// let biguint: U2048 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    /// 
    /// # Example 20 for Slapdash
    /// ```
    /// use cryptocol::random::Slapdash;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut slapdash = Slapdash::new();
    /// let biguint: U3072 = slapdash.prepared_random_prime_with_half_length();
    /// println!("Slapdash Number: {}", biguint);
    /// ```
    pub fn prepared_random_prime_with_half_length<T, const N: usize>(&mut self) -> BigUInt<T, N>
    where T: SmallUInt
    {
        unimplemented!(); // Dummy code for documentation
    }
}
