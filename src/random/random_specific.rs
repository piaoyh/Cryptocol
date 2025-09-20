// Copyright 2024, 2025 PARK Youngho.
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
use crate::symmetric::{ Rijndael_64_64, AES_128, DES };
use crate::random::{ Random_Generic, AnyNumber_Engine_C, SECURE_COUNT };


/// The type `Random` which is a random number generator and is a synonym of
/// [`Random_BIG_KECCAK_1024`](type@Random_BIG_KECCAK_1024) at the moment.
/// It means `Random` uses a hash algorithm Random_BIG_KECCAK_1024. It is
/// cryptographically securer than its counterpart type `Any` and a bit slower
/// than [`Any`](type@Any). _In the near future, `Random` may be silently
/// changed to use better alogrithm._ If you want to keep using BIG_KECCAK_1024
/// for a pseudo-random number generator, you may want to use
/// Random_BIG_KECCAK_1024. If you are happy that you will automatically use
/// the better algotrithm in the future, you may want to use `Random`.
pub type Random = Random_BIG_KECCAK_1024;

/// The type `Any` which is a random number generator and is a synonym of
/// [`Any_SHAKE_128`](type@Any_SHAKE_128) at the moment. It means `Any` uses
/// a hash algorithm SHAKE-128. It is cryptographically less secure than its
/// counterpart struct `Random` and a bit faster than [`Random`](type@Random).
/// _In the near future, `Any` may be silently changed to have better algorithm._
/// If you want to keep using SHAKE_128 for a pseudo-random number generator,
/// you may want to use Any_SHAKE_128. If you are happy that you will
/// automatically use the better algotrithm in the future, you may want
/// to use `Any`.
pub type Any = Any_SHAKE_128;

/// The type `Any_Num` which is a random number generator and is a synonym of
/// [`Any_Num_C`](type@Any_Num_C) at the moment. It means `Any_Num` uses a
/// pseudo-random number generator algorithm of the function rand() of C
/// standard library. It is a specific version of the generic struct. It is
/// cryptographically insecure. _In the near future, `Any_Num` may be silently
/// changed to use better alogrithm and to be a synonym of the better type
/// when the better alogrithm is implemented._ If you want to keep using the
/// algorithm of C standard libraray for a pseudo-random number generator,
/// you may want to use Any_Num_C. If you are happy that you will
/// automatically use the better algotrithm in the future, you may want to
/// use `Any_Num`.
#[allow(non_camel_case_types)]
pub type Any_Num = Any_Num_C;




/// The struct `Random_BIG_KECCAK_1024` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Random_BIG_KECCAK_1024` constructs
/// uses the hash algorithm `BIG_KECCAK_1024` as a pseudo-random number engine
/// generator.
/// 
/// # QUICK START
/// You can use `Random_BIG_KECCAK_1024` to create an if you use random number
/// for cryptographic purpose. `Random_BIG_KECCAK_1024` is for normal
/// cryptographical purpose Look into the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Random_BIG_KECCAK_1024;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_BIG_KECCAK_1024::new();
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
pub struct Random_BIG_KECCAK_1024 {}
impl Random_BIG_KECCAK_1024
{
    // pub fn new() -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new();
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    #[inline]
    pub fn new() -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_BIG_KECCAK_1024::new_with_seeds(0, 0);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seeds(BIG_KECCAK_1024::new(), BIG_KECCAK_1024::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
    /// let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_BIG_KECCAK_1024::new_with_seed_arrays(seed, aux);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seeds(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seed_arrays(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }
}



/// The struct `Random_SHA3_512` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Random_SHA3_512` constructs
/// uses the hash algorithm `Random_SHA3_512` as a pseudo-random number engine
/// generator.
/// 
/// # QUICK START
/// You can use `Random_SHA3_512` to create an if you use random number
/// for cryptographic purpose. `Random_SHA3_512` is for normal
/// cryptographical purpose Look into the following examples.
/// 
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
pub struct Random_SHA3_512 {}
impl Random_SHA3_512
{
    // pub fn new() -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new();
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    #[inline]
    pub fn new() -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with(SHA3_512::new(), SHA3_512::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1 for Random_SHA3_512
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_SHA3_512::new_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 12_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    /// let mut rand = Random_SHA3_512::new_with_seed_arrays(seed, aux);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seeds(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seed_arrays(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }
}



/*---------------------------- */
/// The struct `Random_Rijndael` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Random_Rijndael` constructs
/// uses the encryption/decryption algorithm `Random_Rijndael` with CTR
/// (CounTeR) mode as a pseudo-random number engine generator.
/// 
/// # QUICK START
/// You can use `Random_Rijndael` to create an if you use random number
/// for cryptographic purpose. `Random_Rijndael` is for normal
/// cryptographical purpose. Look into the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Random_Rijndael;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut rand = Random_Rijndael::new();
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
pub struct Random_Rijndael {}
impl Random_Rijndael
{
    // pub fn new() -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    #[inline]
    pub fn new() -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with(AES_128::new(), AES_128::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seeds(AES_128::new(), AES_128::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    ///
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_Rijndael::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", rand.random_u32());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_arrays(AES_128::new(), AES_128::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector(AES_128::new(), AES_128::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seeds(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT> 
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seed_arrays(AES_128::new(), AES_128::new(), seed_collector, seed, aux)
    }
}


/// The struct `Any_Rijndael` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Any_Rijndael` constructs
/// uses the encryption/decryption algorithm `Any_Rijndael` with CTR
/// (CounTeR) mode as a pseudo-random number engine generator.
/// 
/// # QUICK START
/// You can use `Any_Rijndael` to create an if you use random number
/// for cryptographic purpose. `Any_Rijndael` is for normal
/// cryptographical purpose. Look into the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Any_Rijndael;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
///
/// let mut any = Any_Rijndael::new();
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
pub struct Any_Rijndael {}
impl Any_Rijndael
{
    // pub fn new() -> Random_Generic
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Rijndael::new();
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(Rijndael_64_64::new(), Rijndael_64_64::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1 for Any_Rijndael
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// 
    /// let mut any = Any_Rijndael::new_with_seeds(u16::MAX as u64, u16::MAX as u64);
    /// println!("Any number = {}", any.random_u16());
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(Rijndael_64_64::new(), Rijndael_64_64::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_Rijndael;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_Rijndael::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_u16());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(Rijndael_64_64::new(), Rijndael_64_64::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(Rijndael_64_64::new(), Rijndael_64_64::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(Rijndael_64_64::new(), Rijndael_64_64::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(Rijndael_64_64::new(), Rijndael_64_64::new(), seed_collector, seed, aux)
    }
}


/// The struct `Any_DES` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Any_DES` constructs
/// uses the encryption/decryption algorithm `Any_DES` with CTR
/// (CounTeR) mode as a pseudo-random number engine generator.
/// 
/// # QUICK START
/// You can use `Any_DES` to create an if you use random number
/// for cryptographic purpose. `Any_DES` is for normal
/// cryptographical purpose. Look into the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Any_DES;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
///
/// let mut any = Any_DES::new();
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
pub struct Any_DES {}
impl Any_DES
{
    // pub fn new() -> Random_Generic
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_DES::new();
    /// println!("Any number = {}", any.random_odd_biguint());
    /// ```
    #[inline]
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(DES::new(), DES::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1 for Any_DES
    /// ```
    /// use cryptocol::random::Any_DES;
    /// 
    /// let mut any = Any_DES::new_with_seeds(u8::MAX as u64, u8::MAX as u64);
    /// println!("Any number = {}", any.random_u8());
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(DES::new(), DES::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_DES;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_DES::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_u8());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic 
    {
        Random_Generic::new_with_generators_seed_arrays(DES::new(), DES::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(DES::new(), DES::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic 
    {
        Random_Generic::new_with_generators_seed_collector_seeds(DES::new(), DES::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic 
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(DES::new(), DES::new(), seed_collector, seed, aux)
    }
}



/// The struct `Any_Num_C` that constructs the
/// [`Random_Generic`](struct@Random_Generic) 
/// object for implementing a pseudo-random number generator both for primitive
/// unsigned integers such as `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`,
/// and for `BigUInt`. The object which this `Any_Num_C` constructs
/// uses a pseudo-random number generator algorithm of rand() of C standard
/// library but it is still cryptographically not secure enough.
/// 
/// It is __for non-cryptographic purpose__. So, normally it is OK to use this
/// struct `Any_Num_C` to create an object of pseudo-random number generator.
/// However, __DO NOT USE THIS STRUCT FOR SERIOUS CRYPTOGRAPHIC PURPOSE__
/// because it does not guarrantee the cryptographic security.
/// 
/// # QUICK START
/// You can use `Any_Num_C` to create an if you use random number for non-cryptographic
/// purpose. `Any_Num_C` is for normal non-cryptographical purpose Look into
/// the following examples.
/// 
/// ## Example
/// ```
/// use cryptocol::random::Any_Num_C;
/// use cryptocol::define_utypes_with;
/// define_utypes_with!(u64);
/// 
/// let mut any = Any_Num_C::new();
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
pub struct Any_Num_C {}
impl Any_Num_C
{
    // pub fn new() -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Example 1 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Num_C::new();
    /// println!("Any number = {}", any.random_usize());
    /// ```
    /// 
    /// # Example 2 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_Num::new();
    /// println!("Any number = {}", any.random_u8());
    /// ```
    #[inline]
    pub fn new() -> Random_Generic<{u32::MAX as u128}>
    {
        Random_Generic::<{u32::MAX as u128}>::new_with(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new())
    }

    // pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new struct Random_Generic with two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
    /// 
    /// # Example 1 for Any_Num_C
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// let mut any = Any_Num_C::new_with_seeds(458861005, 793621585);
    /// println!("Any number = {}", any.random_u64());
    /// ```
    /// 
    /// # Example 2 for Any_Num
    /// ```
    /// use cryptocol::random::Any_Num;
    /// let mut any = Any_Num::new_with_seeds(50558, 18782);
    /// println!("Any number = {}", any.random_u32());
    /// ```
    #[inline]
    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u32::MAX as u128}>   // COUNT = u32::MAX
    {
        Random_Generic::<{u32::MAX as u128}>::new_with_generators_seeds(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u32::MAX as u128}> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_Num_C;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_Num_C::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_u64());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u32::MAX as u128}> 
    {
        Random_Generic::<{u32::MAX as u128}>::new_with_generators_seed_arrays(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    {
        Random_Generic::<{u32::MAX as u128}>::new_with_generators_seed_collector(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u32::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u32::MAX as u128}> 
    {
        Random_Generic::<{u32::MAX as u128}>::new_with_generators_seed_collector_seeds(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u32::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u32::MAX as u128}> 
    {
        Random_Generic::<{u32::MAX as u128}>::new_with_generators_seed_collector_seed_arrays(AnyNumber_Engine_C::new(), AnyNumber_Engine_C::new(), seed_collector, seed, aux)
    }
}

///////////////////////




/// The type `Any_MD4` which is a pseudo-random number generator using a hash
/// algorithm MD4.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)]
pub struct Any_MD4 {}
impl Any_MD4
{
    pub fn new() -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with(MD4::new(), MD4::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seeds(MD4::new(), MD4::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_MD4;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_MD4::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_u64());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_arrays(MD4::new(), MD4::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector(MD4::new(), MD4::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seeds(MD4::new(), MD4::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seed_arrays(MD4::new(), MD4::new(), seed_collector, seed, aux)
    }
}


/// The type `Any_MD5` which is a pseudo-random number generator using a hash
/// algorithm MD5.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)]
pub struct Any_MD5 {}
impl Any_MD5
{
    pub fn new() -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with(MD5::new(), MD5::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seeds(MD5::new(), MD5::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_MD5;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_MD5::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_u128());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_arrays(MD5::new(), MD5::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector(MD5::new(), MD5::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seeds(MD5::new(), MD5::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seed_arrays(MD5::new(), MD5::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA0` which is a pseudo-random number generator using a hash
/// algorithm SHA0.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA0 {}
impl Any_SHA0
{
    pub fn new() -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with(SHA0::new(), SHA0::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seeds(SHA0::new(), SHA0::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA0;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_SHA0::new_with_seed_arrays(seed, aux);
    /// println!("Any prime number = {}", any.random_prime_using_miller_rabin_uint::<u128>(5));
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_arrays(SHA0::new(), SHA0::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector(SHA0::new(), SHA0::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seeds(SHA0::new(), SHA0::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seed_arrays(SHA0::new(), SHA0::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA1` which is a pseudo-random number generator using a hash
/// algorithm SHA1.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA1 {}
impl Any_SHA1
{
    pub fn new() -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with(SHA1::new(), SHA1::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seeds(SHA1::new(), SHA1::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA1;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_SHA1::new_with_seed_arrays(seed, aux);
    /// println!("Any number = {}", any.random_uint::<u8>());
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_arrays(SHA1::new(), SHA1::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u32::MAX as u128}>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<{u64::MAX as u128}>
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector(SHA1::new(), SHA1::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seeds(SHA1::new(), SHA1::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<{u64::MAX as u128}> 
    {
        Random_Generic::<{u64::MAX as u128}>::new_with_generators_seed_collector_seed_arrays(SHA1::new(), SHA1::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA2_256` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-256.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA2_256 {}
impl Any_SHA2_256
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHA2_256::new(), SHA2_256::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHA2_256::new(), SHA2_256::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA2_256;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_SHA2_256::new_with_seed_arrays(seed, aux);
    /// if let Some(num) = any.random_under_uint(1234_u16)
    ///     { println!("Any number = {}", num); }
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHA2_256::new(), SHA2_256::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHA2_256::new(), SHA2_256::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHA2_256::new(), SHA2_256::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA2_512` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-512.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA2_512 {}
impl Any_SHA2_512
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHA2_512::new(), SHA2_512::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA2_512;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_SHA2_512::new_with_seed_arrays(seed, aux);
    /// if let Some(num) = any.random_minmax_uint(12345678_u32, 87654321)
    ///     { println!("Any number = {}", num); }
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHA2_512::new(), SHA2_512::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux)
    }
}

/// The type `Random_SHA2_512` which is a pseudo-random number generator using
/// a hash algorithm SHA-2-512.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Random_SHA2_512 {}
impl Random_SHA2_512
{
    pub fn new() -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with(SHA2_512::new(), SHA2_512::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seeds(SHA2_512::new(), SHA2_512::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Random_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_SHA2_512::new_with_seed_arrays(seed, aux);
    /// let num: U256 = rand.random_biguint();
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector(SHA2_512::new(), SHA2_512::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seeds(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT>
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<SECURE_COUNT>
    {
        Random_Generic::<SECURE_COUNT>::new_with_generators_seed_collector_seed_arrays(SHA2_512::new(), SHA2_512::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA3_256` which is a pseudo-random number generator using
/// a hash algorithm SHA-3-256.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA3_256 {}
impl Any_SHA3_256
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHA3_256::new(), SHA3_256::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHA3_256::new(), SHA3_256::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA3_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [10500872879054459758_u64, 777777777777_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789654123_u64, 5_u64, 789456123_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut any = Any_SHA3_256::new_with_seed_arrays(seed, aux);
    /// let num: U768 = any.random_odd_with_msb_set_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHA3_256::new(), SHA3_256::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHA3_256::new(), SHA3_256::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHA3_256::new(), SHA3_256::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHA3_512` which is a pseudo-random number generator using
/// a hash algorithm SHA-3-512.
/// It is a specific version of the generic struct
/// [`Random_Generic`](struct@Random_Generic).
#[allow(non_camel_case_types)] 
pub struct Any_SHA3_512 {}
impl Any_SHA3_512
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHA3_512::new(), SHA3_512::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHA3_512::new(), SHA3_512::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHA3_512::new_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHA3_512::new(), SHA3_512::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHA3_512::new(), SHA3_512::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHA3_512::new(), SHA3_512::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHAKE_256` which is a pseudo-random number generator using
/// a hash algorithm SHAKE-256. It is a specific version of the generic struct
#[allow(non_camel_case_types)] 
pub struct Any_SHAKE_256 {}
impl Any_SHAKE_256
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHAKE_256::new(), SHAKE_256::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHAKE_256::new(), SHAKE_256::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHAKE_256;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_256::new_with_seeds(123456789, 987654321);
    /// let num: U512 = any.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHAKE_256::new(), SHAKE_256::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHAKE_256::new(), SHAKE_256::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHAKE_256::new(), SHAKE_256::new(), seed_collector, seed, aux)
    }
}

/// The type `Any_SHAKE_128` which is a pseudo-random number generator using
/// a hash algorithm SHAKE-128. It is a specific version of the generic struct
#[allow(non_camel_case_types)] 
pub struct Any_SHAKE_128 {}
impl Any_SHAKE_128
{
    pub fn new() -> Random_Generic
    {
        Random_Generic::new_with(SHAKE_128::new(), SHAKE_128::new())
    }

    pub fn new_with_seeds(seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seeds(SHAKE_128::new(), SHAKE_128::new(), seed, aux)
    }

    // pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with two seed arrays of type `u64`.
    /// 
    /// # Arguments
    /// - `seed` is the seed array and is of `[u64; 8]`.
    /// - `aux` is the seed array and is of `[u64; 8]`.
    /// 
    /// # Output
    /// It returns a new object of `Random_Generic`.
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::random::Any_SHAKE_128;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_SHAKE_128::new_with_seeds(u32::MAX as u64, u32::MAX as u64);
    /// let num: U384 = any.random_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn new_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed, aux)
    }
    
    // pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    /// Constructs a new `Random_Generic` object with a seed collector function.
    /// 
    /// # Arguments
    /// `seed_collector` is a seed collector function to collect seeds, and
    /// is of the type `fn() -> [u64; 8]`.
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
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_Rijndael::new();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector(SHAKE_128::new(), SHAKE_128::new(), seed_collector)
    }

    // pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<{u64::MAX as u128}> 
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seeds of type `u64`.
    /// 
    /// # Arguments
    /// - `seed_collector` is a seed collector function to collect seeds, and
    ///   is of the type `fn() -> [u64; 8]`.
    /// - `seed` is the seed number of the type `u64`.
    /// - `aux` is the seed number of the type `u64`.
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
    /// # Cryptographical Security
    /// You are highly recommended to use the method `new_with_seed_arrays()`
    /// rather than this method for security reason. It is because the default
    /// seed collector function collects 1024 bits as a seed. If you use this
    /// method, it results that you give only '128' bits (= '64' bits + '64'
    /// bits) as a seed and the other '896' bits will be made out of the '128'
    /// bits that you provided.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seeds(SHAKE_128::new(), SHAKE_128::new(), seed_collector, seed, aux)
    }

    // pub fn new_with_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    /// Constructs a new struct Random_Generic with a seed collector function
    /// and two seed arrays of type `u64`.
    /// 
    /// # Arguments
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
    /// 
    /// # Cryptographical Security
    /// You are highly recommended to use this method rather than the method
    /// new_with_seed_collector_seeds for security reason. It is because the
    /// default seed collector function collects 1024 bits as a seed. If you
    /// use this method, it results that you give full '1024' bits (= '64'
    /// bits X '8' X '2') as a seed and it is equivalent to use a seed
    /// collector function.
/*  /// 
    /// # Example 1 for Random_Rijndael
    /// ```
    /// use cryptocol::random::Random_Rijndael;
    /// 
    /// let mut rand = Random_Rijndael::new_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Any number = {}", rand.random_u32());
    /// ``` */
    #[inline]
    pub fn new_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic
    {
        Random_Generic::new_with_generators_seed_collector_seed_arrays(SHAKE_128::new(), SHAKE_128::new(), seed_collector, seed, aux)
    }
}
