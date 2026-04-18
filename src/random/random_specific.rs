// Copyright 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a random number generator struct

#![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]


use crate::hash::{ MD4, MD5, SHA0, SHA1, SHA2_256, SHA2_512,
                    SHA3_256, SHA3_512, SHAKE_128, SHAKE_256, BIG_KECCAK_1024 };
use crate::symmetric::{ AES_128, DES };
use crate::random::{ Random_Generic, CPRNG_Engine,
                    SECURE_COUNT, LESS_SECURE_COUNT, INSECURE_COUNT };



macro_rules! PRNG_Creator_methods {
    ($engine:ident, $count:expr) => {
        // pub fn create() -> Random_Generic<COUNT>
        /// Creates a new `Random_Generic` object.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
        /// use cryptocol::define_utypes_with;
        /// define_utypes_with!(u64);
        /// 
        /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create();
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// println!("Random number = {}", num);
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_random_specific/struct.PRNG_Creator.html#method.create)
        #[inline]
        pub fn create() -> Random_Generic<$count>
        {
            Random_Generic::<$count>::new_with(Box::new($engine::new()), Box::new($engine::new()))
        }

        // pub fn create_with_seeds(seed: u64, aux: u64) -> Random_Generic<COUNT>
        /// Creates a new struct Random_Generic with two seeds of type `u64`.
        /// 
        /// # Arguments
        /// - `seed` is the seed number of the type `u64`.
        /// - `aux` is the seed number of the type `u64`.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Cryptographical Security
        /// You are highly recommended to use the method `Creates_with_seed_arrays()`
        /// rather than this method for security reason. It is because the default
        /// seed collector function collects 1024 bits as a seed. If you use this
        /// method, it results that you give only '128' bits (= '64' bits + '64'
        /// bits) as a seed and the other '896' bits will be made out of the '128'
        /// bits that you provided.
        /// 
        /// # Example 1 for PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::PRNG_Creator_BIG_KECCAK_1024;
        /// use cryptocol::define_utypes_with;
        /// define_utypes_with!(u64);
        /// 
        /// let mut rand = PRNG_Creator_BIG_KECCAK_1024::create_with_seeds(0, 0);
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// println!("Random number = {}", num);
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_specific/struct.PRNG_Creator.html#method.create_with_seeds)
        #[inline]
        pub fn create_with_seeds(seed: u64, aux: u64) -> Random_Generic<$count>
        {
            Random_Generic::<$count>::new_with_generators_seeds(Box::new($engine::new()), Box::new($engine::new()), seed, aux)
        }

        // pub fn create_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<COUNT>
        /// Creates a new struct Random_Generic with two seed arrays of type `u64`.
        /// 
        /// # Arguments
        /// - `seed` is the seed array and is of `[u64; 8]`.
        /// - `aux` is the seed array and is of `[u64; 8]`.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Cryptographical Security
        /// You are highly recommended to use this method rather than the method
        /// create_with_seed_collector_seeds for security reason. It is because the
        /// default seed collector function collects 1024 bits as a seed. If you
        /// use this method, it results that you give full '1024' bits (= '64'
        /// bits X '8' X '2') as a seed and it is equivalent to use a seed
        /// collector function.
        /// 
        /// # Example 1 for PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::PRNG_Creator_BIG_KECCAK_1024;
        /// use cryptocol::define_utypes_with;
        /// define_utypes_with!(u64);
        /// 
        /// let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
        /// let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
        /// let mut rand = PRNG_Creator_BIG_KECCAK_1024::create_with_seed_arrays(seed, aux);
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_specific/struct.PRNG_Creator.html#method.create_with_seeds)
        #[inline]
        pub fn create_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<$count> 
        {
            Random_Generic::<$count>::new_with_generators_seed_arrays(Box::new($engine::new()), Box::new($engine::new()), seed, aux)
        }
        
        // pub fn create_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<COUNT>
        /// Creates a new `Random_Generic` object with a seed collector function.
        /// 
        /// # Arguments
        /// `seed_collector` is a seed collector function to collect seeds, and
        /// is of the type `fn() -> [u64; 8]`.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Example 1 for PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::PRNG_Creator_BIG_KECCAK_1024;
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
        /// let mut rand = PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector(seed_collector);
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// println!("Random number = {}", num);
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_specific/struct.PRNG_Creator.html#method.create_with_seed_collector)
        #[inline]
        pub fn create_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<$count>
        {
            Random_Generic::<$count>::new_with_generators_seed_collector(Box::new($engine::new()), Box::new($engine::new()), seed_collector)
        }

        // pub fn create_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<COUNT>
        /// Creates a new struct Random_Generic with a seed collector function
        /// and two seeds of type `u64`.
        /// 
        /// # Arguments
        /// - `seed_collector` is a seed collector function to collect seeds, and
        ///   is of the type `fn() -> [u64; 8]`.
        /// - `seed` is the seed number of the type `u64`.
        /// - `aux` is the seed number of the type `u64`.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Cryptographical Security
        /// You are highly recommended to use the method `create_with_collector_seed_arrays()`
        /// rather than this method for security reason. It is because the default
        /// seed collector function collects 1024 bits as a seed. If you use this
        /// method, it results that you give only '128' bits (= '64' bits + '64'
        /// bits) as a seed and the other '896' bits will be made out of the '128'
        /// bits that you provided.
        /// 
        /// # Example 1 for PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::PRNG_Creator_BIG_KECCAK_1024;
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
        /// let mut rand = PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seeds(seed_collector, 0, 0);
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// println!("Random number = {}", num);
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_specific/struct.PRNG_Creator.html#method.create_with_seed_collector_seeds)
        #[inline]
        pub fn create_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<$count>
        {
            Random_Generic::<$count>::new_with_generators_seed_collector_seeds(Box::new($engine::new()), Box::new($engine::new()), seed_collector, seed, aux)
        }

        // pub fn create_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<COUNT>
        /// Creates a new struct Random_Generic with a seed collector function
        /// and two seed arrays of type `u64`.
        /// 
        /// # Arguments
        /// - `seed_collector` is a seed collector function to collect seeds, and
        ///   is of the type `fn() -> [u64; 8]`.
        /// - `seed` is the seed array and is of `[u64; 8]`.
        /// - `aux` is the seed array and is of `[u64; 8]`.
        /// 
        /// # Returns
        /// It returns a new object of `Random_Generic`.
        /// 
        /// # Cryptographical Security
        /// You are highly recommended to use this method rather than the method
        /// create_with_seed_collector_seeds for security reason. It is because the
        /// default seed collector function collects 1024 bits as a seed. If you
        /// use this method, it results that you give full '1024' bits (= '64'
        /// bits X '8' X '2') as a seed and it is equivalent to use a seed
        /// collector function.
        /// 
        /// # Example 1 for PRNG_Creator_BIG_KECCAK_1024
        /// ```
        /// use cryptocol::random::PRNG_Creator_BIG_KECCAK_1024;
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
        /// let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
        /// let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
        /// let mut rand = PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
        /// let num: U1024 = rand.random_with_msb_set_biguint();
        /// println!("Random number = {}", num);
        /// ```
        /// 
        /// # For more examples,
        /// click [here](./documentation/random_specific/struct.PRNG_Creator.html#method.create_with_seed_collector_seed_arrays)
        #[inline]
        pub fn create_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<$count>
        {
            Random_Generic::<$count>::new_with_generators_seed_collector_seed_arrays(Box::new($engine::new()), Box::new($engine::new()), seed_collector, seed, aux)
        }
    };
}
pub(crate) use PRNG_Creator_methods;



/// The type [`Random_PRNG_Creator`](type@Random_PRNG_Creator) which is a
/// creator for a random number generator and is a synonym of
/// [`Random_PRNG_Creator_BIG_KECCAK_1024`](type@Random_PRNG_Creator_BIG_KECCAK_1024)
/// at the moment. It means that the prng created by `Random_PRNG_Creator` uses
/// a hash algorithm `BIG_KECCAK_1024`. It is cryptographically securer than the prng
/// created by its counterpart type [`Any_PRNG_Creator`](type@Any_PRNG_Creator)
/// and may be a bit slower than the prng created by
/// [`Any_PRNG_Creator`](type@Any_PRNG_Creator).
/// _In the near future, `Random_PRNG_Creator` may be silently changed to create
/// a better prng._ If you want to keep using the prng with `BIG_KECCAK_1024`
/// for a pseudo-random number generator, you may want to use
/// [`Random_PRNG_Creator_BIG_KECCAK_1024`](type@Random_PRNG_Creator_BIG_KECCAK_1024).
/// If you are happy that you will automatically use the better algotrithm in
/// the future, you may want to use [`Random_PRNG_Creator`](type@Random_PRNG_Creator).
#[allow(non_camel_case_types)]
pub type Random_PRNG_Creator = Random_PRNG_Creator_BIG_KECCAK_1024;


/// The type [`Any_PRNG_Creator`](type@Any_PRNG_Creator) which is a random
/// number generator and is a synonym of
/// [`Any_PRNG_Creator_SHA2_512`](type@Any_PRNG_Creator_SHA2_512) at the moment.
/// It means that the prng created by [`Any_PRNG_Creator_SHA2_512`](type@Any_PRNG_Creator_SHA2_512)
/// uses a hash algorithm SHA2_512. It is cryptographically less secure than the
/// prng created by its counterpart type [`Random_PRNG_Creator`](type@Random_PRNG_Creator)
/// and may be a bit faster than the prng created by [`Random_PRNG_Creator`](type@Random_PRNG_Creator).
/// _In the near future, [`Any_PRNG_Creator`](type@Any_PRNG_Creator) may be
/// silently changed to create a better prng._ If you want to keep using SHA-512
/// for a pseudo-random number generator, you may want to use [`Any_PRNG_Creator`](type@Any_PRNG_Creator).
/// If you are happy that you will automatically use the better algotrithm in
/// the future, you may want to use [`Any_PRNG_Creator`](type@Any_PRNG_Creator).
#[allow(non_camel_case_types)]
pub type Any_PRNG_Creator = Any_PRNG_Creator_SHA2_512;


/// The type [`Slapdash_PRNG_Creator`](type@Slapdash_PRNG_Creator) which is a
/// creator for a random number generator and is
/// a synonym of [`Slapdash_PRNG_Creator_CPRNG_Engine`](type@Slapdash_PRNG_Creator_CPRNG_Engine)
/// at the moment. It means that the prng created by [`Slapdash_PRNG_Creator_CPRNG_Engine`](type@Slapdash_PRNG_Creator_CPRNG_Engine)
/// uses a pseudo-random number generator algorithm of the function rand() of C
/// standard library. It is cryptographically insecure. _In the near future,
/// [`Slapdash_PRNG_Creator`](type@Slapdash_PRNG_Creator) may be silently
/// changed to create a better prng. If you want to keep using the prng with the
/// algorithm of C standard libraray for a pseudo-random number generator, you
/// may want to use [`Slapdash_PRNG_Creator_CPRNG_Engine`](type@Slapdash_PRNG_Creator_CPRNG_Engine).
/// If you are happy that you will automatically use the better algotrithm in
/// the future, you may want to use [`Slapdash_PRNG_Creator`](type@Slapdash_PRNG_Creator).
#[allow(non_camel_case_types)]
pub type Slapdash_PRNG_Creator = Slapdash_PRNG_Creator_CPRNG_Engine;



macro_rules! DOC_STRING {
    ($engine:expr, $cat:expr) => { concat!(
"A PRNG creator that produces a [`Random_Generic`](struct@Random_Generic)
instance using the **", stringify!($engine),
r##"** hash algorithm as its underlying
engine.

This struct produces a pseudo-random number generator (PRNG) capable of
generating both primitive unsigned integers (`u8`, `u16`, `u32`, `u64`,
`u128`, `usize`) and `BigUInt` values. In other words, it is a factory
struct for [`Random_Generic`](struct@Random_Generic) instances.

# Quick Start
`"##, stringify!($cat), "_PRNG_Creator_", stringify!($engine),
"` is designed for standard cryptographic purposes. If you require a
secure random number generator for cryptography,
refer to the following examples.", r#"

"#)
    };

    ($engine:expr) => { concat!(
"A PRNG creator that produces a [`Random_Generic`](struct@Random_Generic)
instance using the **", stringify!($engine),
r##"** hash algorithm as its underlying
engine.

This struct produces a pseudo-random number generator (PRNG) capable of
generating both primitive unsigned integers (`u8`, `u16`, `u32`, `u64`,
`u128`, `usize`) and `BigUInt` values. In other words, it is a factory
struct for [`Random_Generic`](struct@Random_Generic) instances.

# Quick Start
`"##, "Slapdash_PRNG_Creator_", stringify!($engine),
"` is designed for non-cryptographic purposes. Unless you require a
secure random number generator for cryptography,
refer to the following examples.", r#"

"#)
    };
}
pub(crate) use DOC_STRING;



#[doc = DOC_STRING!(BIG_KECCAK_1024, RANDOM)]
/// ## Example
/// ```
/// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create();
/// println!("Random number = {}", rand.random_u128());
/// println!("Random number = {}", rand.random_u64());
/// println!("Random number = {}", rand.random_u32());
/// println!("Random number = {}", rand.random_u16());
/// println!("Random number = {}", rand.random_u8());
/// 
/// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
///     { println!("Random number u64 = {}", num); }
/// 
/// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
///     { println!("Random number u16 = {}", num); }
/// 
/// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
/// if let Some(num) = rand.random_odd_under_uint(1234_u16)
///     { println!("Random odd number u16 = {}", num); }
/// 
/// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
/// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
/// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = rand.random_array();
/// for i in 0..20
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// rand.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = rand.random_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = rand.random_under_biguint(&ceiling)
/// {
///     println!("Random Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = rand.random_under_biguint_(&ceiling);
/// println!("Random Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
/// {
///     println!("Random odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = rand.random_with_msb_set_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// biguint = rand.random_odd_with_msb_set_biguint();
/// println!("512-bit Random Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_using_miller_rabin_biguint(5);
/// println!("Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Random_PRNG_Creator_BIG_KECCAK_1024 {}
impl Random_PRNG_Creator_BIG_KECCAK_1024
{
    PRNG_Creator_methods!{BIG_KECCAK_1024, SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA3_512, RANDOM)]
/// ## Example
/// ```
/// use cryptocol::random::Random_SHA3_512;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_SHA3_512::new();
/// println!("Random number = {}", rand.random_u128());
/// println!("Random number = {}", rand.random_u64());
/// println!("Random number = {}", rand.random_u32());
/// println!("Random number = {}", rand.random_u16());
/// println!("Random number = {}", rand.random_u8());
/// 
/// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
///     { println!("Random number u64 = {}", num); }
/// 
/// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
///     { println!("Random number u16 = {}", num); }
/// 
/// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
/// if let Some(num) = rand.random_odd_under_uint(1234_u16)
///     { println!("Random odd number u16 = {}", num); }
/// 
/// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
/// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
/// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = rand.random_array();
/// for i in 0..20
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// rand.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = rand.random_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = rand.random_under_biguint(&ceiling)
/// {
///     println!("Random Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = rand.random_under_biguint_(&ceiling);
/// println!("Random Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
/// {
///     println!("Random odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = rand.random_with_msb_set_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// biguint = rand.random_odd_with_msb_set_biguint();
/// println!("512-bit Random Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_using_miller_rabin_biguint(5);
/// println!("Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Random_PRNG_Creator_SHA3_512 {}
impl Random_PRNG_Creator_SHA3_512
{
    PRNG_Creator_methods!{SHA3_512, SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA2_512, RANDOM)]
/// ## Example
/// ```
/// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_SHA2_512::new();
/// println!("Random number = {}", rand.random_u128());
/// println!("Random number = {}", rand.random_u64());
/// println!("Random number = {}", rand.random_u32());
/// println!("Random number = {}", rand.random_u16());
/// println!("Random number = {}", rand.random_u8());
/// 
/// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
///     { println!("Random number u64 = {}", num); }
/// 
/// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
///     { println!("Random number u16 = {}", num); }
/// 
/// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
/// if let Some(num) = rand.random_odd_under_uint(1234_u16)
///     { println!("Random odd number u16 = {}", num); }
/// 
/// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
/// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
/// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = rand.random_array();
/// for i in 0..20
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// rand.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = rand.random_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = rand.random_under_biguint(&ceiling)
/// {
///     println!("Random Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = rand.random_under_biguint_(&ceiling);
/// println!("Random Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
/// {
///     println!("Random odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = rand.random_with_msb_set_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// biguint = rand.random_odd_with_msb_set_biguint();
/// println!("512-bit Random Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_using_miller_rabin_biguint(5);
/// println!("Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Random_PRNG_Creator_SHA2_512 {}
impl Random_PRNG_Creator_SHA2_512
{
    PRNG_Creator_methods!{SHA2_512, SECURE_COUNT}
}



#[doc = DOC_STRING!(AES_128, RANDOM)]
/// ## Example
/// ```
/// use cryptocol::random::Random_PRNG_Creator_AES_128;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_PRNG_Creator_AES_128::create();
/// println!("Random number = {}", rand.random_u128());
/// println!("Random number = {}", rand.random_u64());
/// println!("Random number = {}", rand.random_u32());
/// println!("Random number = {}", rand.random_u16());
/// println!("Random number = {}", rand.random_u8());
/// 
/// if let Some(num) = rand.random_under_uint(1234567890123456_u64)
///     { println!("Random number u64 = {}", num); }
/// 
/// if let Some(num) = rand.random_minmax_uint(1234_u16, 6321)
///     { println!("Random number u16 = {}", num); }
/// 
/// println!("Random odd number usize = {}", rand.random_odd_uint::<usize>());
/// if let Some(num) = rand.random_odd_under_uint(1234_u16)
///     { println!("Random odd number u16 = {}", num); }
/// 
/// println!("Random 128-bit number u128 = {}", rand.random_with_msb_set_uint::<u128>());
/// println!("Random 16-bit odd number u16 = {}", rand.random_with_msb_set_uint::<u16>());
/// println!("Random prime number u64 = {}", rand.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Random usize-sized prime number usize = {}", rand.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = rand.random_array();
/// for i in 0..20
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// rand.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Random number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = rand.random_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = rand.random_under_biguint(&ceiling)
/// {
///     println!("Random Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = rand.random_under_biguint_(&ceiling);
/// println!("Random Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = rand.random_odd_under_biguint(&ceiling)
/// {
///     println!("Random odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = rand.random_with_msb_set_biguint();
/// println!("Random Number: {}", biguint);
/// 
/// biguint = rand.random_odd_with_msb_set_biguint();
/// println!("512-bit Random Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_using_miller_rabin_biguint(5);
/// println!("Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = rand.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Random Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Random_PRNG_Creator_AES_128 {}
impl Random_PRNG_Creator_AES_128
{
    PRNG_Creator_methods!{AES_128, SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA3_512, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHA3_512::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHA3_512<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHA3_512
{
    PRNG_Creator_methods!{SHA3_512, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA3_256, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHA3_256;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHA3_256::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHA3_256<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHA3_256
{
    PRNG_Creator_methods!{SHA3_256, LESS_SECURE_COUNT}
}


#[doc = DOC_STRING!(SHAKE_256, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHAKE_256;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHAKE_256::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHAKE_256<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHAKE_256
{
    PRNG_Creator_methods!{SHAKE_256, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(SHAKE_128, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHAKE_128;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHAKE_128::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHAKE_128<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHAKE_128
{
    PRNG_Creator_methods!{SHAKE_128, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA2_512, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHA2_512;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHA2_512::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHA2_512<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHA2_512
{
    PRNG_Creator_methods!{SHA2_512, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA2_256, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_SHA2_256;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_PRNG_Creator_SHA2_256::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
/// 
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
/// 
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
/// 
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
/// 
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
/// 
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Any_PRNG_Creator_SHA2_256<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_SHA2_256
{
    PRNG_Creator_methods!{SHA2_256, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(AES_128, Any)]
/// ## Example
/// ```
/// use cryptocol::random::Any_PRNG_Creator_AES_128;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
///
/// let mut any = Any_PRNG_Creator_AES_128::create();
/// println!("Any number = {}", any.random_u128());
/// println!("Any number = {}", any.random_u64());
/// println!("Any number = {}", any.random_u32());
/// println!("Any number = {}", any.random_u16());
/// println!("Any number = {}", any.random_u8());
///
/// if let Some(num) = any.random_under_uint(1234567890123456_u64)
///     { println!("Any number u64 = {}", num); }
///
/// if let Some(num) = any.random_minmax_uint(1234_u16, 6321)
///     { println!("Any number u16 = {}", num); }
///
/// println!("Any odd number usize = {}", any.random_odd_uint::<usize>());
/// if let Some(num) = any.random_odd_under_uint(1234_u16)
///     { println!("Any odd number u16 = {}", num); }
///
/// println!("Any 128-bit number u128 = {}", any.random_with_msb_set_uint::<u128>());
/// println!("Any 16-bit odd number u16 = {}", any.random_with_msb_set_uint::<u16>());
/// println!("Any prime number u64 = {}", any.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Any usize-sized prime number usize = {}", any.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
///
/// let num: [u128; 20] = any.random_array();
/// for i in 0..20
///     { println!("Any number {} => {}", i, num[i]); }
///
/// let mut num = [0_u64; 32];
/// any.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Any number {} => {}", i, num[i]); }
///
/// let mut biguint: U512 = any.random_biguint();
/// println!("Any Number: {}", biguint);
///
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = any.random_under_biguint(&ceiling)
/// {
///     println!("Any Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
///
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = any.random_under_biguint_(&ceiling);
/// println!("Any Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
///
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = any.random_odd_under_biguint(&ceiling)
/// {
///     println!("Any odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
///
/// biguint = any.random_with_msb_set_biguint();
/// println!("Any Number: {}", biguint);
///
/// biguint = any.random_odd_with_msb_set_biguint();
/// println!("512-bit Any Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
///
/// biguint = any.random_prime_using_miller_rabin_biguint(5);
/// println!("Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
///
/// biguint = any.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Any Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Any_PRNG_Creator_AES_128<const COUNT: u64 = LESS_SECURE_COUNT> {}
impl Any_PRNG_Creator_AES_128
{
    PRNG_Creator_methods!{AES_128, LESS_SECURE_COUNT}
}



#[doc = DOC_STRING!(SHA1)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_SHA1;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut slapdash = Slapdash_PRNG_Creator_SHA1::crate();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
/// 
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
/// 
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
/// 
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
/// 
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Slapdash_PRNG_Creator_SHA1<const COUNT: u64 = INSECURE_COUNT> {}
impl Slapdash_PRNG_Creator_SHA1
{
    PRNG_Creator_methods!{SHA1, INSECURE_COUNT}
}



#[doc = DOC_STRING!(SHA0)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_SHA0;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut slapdash = Slapdash_PRNG_Creator_SHA0::create();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
/// 
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
/// 
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
/// 
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
/// 
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Slapdash_PRNG_Creator_SHA0 {}
impl Slapdash_PRNG_Creator_SHA0
{
    PRNG_Creator_methods!{SHA0, INSECURE_COUNT}
}



#[doc = DOC_STRING!(MD5)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_MD5;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut slapdash = Slapdash_PRNG_Creator_MD5::create();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
/// 
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
/// 
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
/// 
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
/// 
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Slapdash_PRNG_Creator_MD5 {}
impl Slapdash_PRNG_Creator_MD5
{
    PRNG_Creator_methods!{MD5, INSECURE_COUNT}
}



#[doc = DOC_STRING!(MD4)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_MD4;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut slapdash = Slapdash_PRNG_Creator_MD4::create();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
/// 
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
/// 
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
/// 
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
/// 
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Slapdash_PRNG_Creator_MD4 {}
impl Slapdash_PRNG_Creator_MD4
{
    PRNG_Creator_methods!{MD4, INSECURE_COUNT}
}



#[doc = DOC_STRING!(DES)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_DES;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
///
/// let mut slapdash = Slapdash_PRNG_Creator_DES::create();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
///
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
///
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
///
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
///
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
///
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
///
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
///
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
///
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
///
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
///
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
///
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
///
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
///
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
///
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)] 
pub struct Slapdash_PRNG_Creator_DES {}
impl Slapdash_PRNG_Creator_DES
{
    PRNG_Creator_methods!{DES, INSECURE_COUNT}
}



#[doc = DOC_STRING!(CPRNG_Engine)]
/// ## Example
/// ```
/// use cryptocol::random::Slapdash_PRNG_Creator_CPRNG_Engine;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut slapdash = Slapdash_PRNG_Creator_CPRNG_Engine::create();
/// println!("Slapdash number = {}", slapdash.random_u128());
/// println!("Slapdash number = {}", slapdash.random_u64());
/// println!("Slapdash number = {}", slapdash.random_u32());
/// println!("Slapdash number = {}", slapdash.random_u16());
/// println!("Slapdash number = {}", slapdash.random_u8());
/// 
/// if let Some(num) = slapdash.random_under_uint(1234567890123456_u64)
///     { println!("Slapdash number u64 = {}", num); }
/// 
/// if let Some(num) = slapdash.random_minmax_uint(1234_u16, 6321)
///     { println!("Slapdash number u16 = {}", num); }
/// 
/// println!("Slapdash odd number usize = {}", slapdash.random_odd_uint::<usize>());
/// if let Some(num) = slapdash.random_odd_under_uint(1234_u16)
///     { println!("Slapdash odd number u16 = {}", num); }
/// 
/// println!("Slapdash 128-bit number u128 = {}", slapdash.random_with_msb_set_uint::<u128>());
/// println!("Slapdash 16-bit odd number u16 = {}", slapdash.random_with_msb_set_uint::<u16>());
/// println!("Slapdash prime number u64 = {}", slapdash.random_prime_using_miller_rabin_uint::<u64>(5));
/// println!("Slapdash usize-sized prime number usize = {}", slapdash.random_prime_with_msb_set_using_miller_rabin_uint::<usize>(5));
/// 
/// let num: [u128; 20] = slapdash.random_array();
/// for i in 0..20
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut num = [0_u64; 32];
/// slapdash.put_random_in_array(&mut num);
/// for i in 0..32
///     { println!("Slapdash number {} => {}", i, num[i]); }
/// 
/// let mut biguint: U512 = slapdash.random_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// let mut ceiling = U1024::max().wrapping_div_uint(3_u8);
/// if let Some(r) = slapdash.random_under_biguint(&ceiling)
/// {
///     println!("Slapdash Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// ceiling = U1024::max().wrapping_div_uint(5_u8);
/// let r = slapdash.random_under_biguint_(&ceiling);
/// println!("Slapdash Number less than {} is\n{}", ceiling, r);
/// assert!(r < ceiling);
/// 
/// ceiling = U1024::max().wrapping_div_uint(4_u8);
/// if let Some(r) = slapdash.random_odd_under_biguint(&ceiling)
/// {
///     println!("Slapdash odd Number less than {} is\n{}", ceiling, r);
///     assert!(r < ceiling);
/// }
/// 
/// biguint = slapdash.random_with_msb_set_biguint();
/// println!("Slapdash Number: {}", biguint);
/// 
/// biguint = slapdash.random_odd_with_msb_set_biguint();
/// println!("512-bit Slapdash Odd Number = {}", biguint);
/// assert!(biguint > U512::halfmax());
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_using_miller_rabin_biguint(5);
/// println!("Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// 
/// biguint = slapdash.random_prime_with_msb_set_using_miller_rabin_biguint(5);
/// println!("512-bit Slapdash Prime Number = {}", biguint);
/// assert!(biguint.is_odd());
/// ```
#[allow(non_camel_case_types)]
pub struct Slapdash_PRNG_Creator_CPRNG_Engine {}
impl Slapdash_PRNG_Creator_CPRNG_Engine
{
    PRNG_Creator_methods!{CPRNG_Engine, INSECURE_COUNT}
}
