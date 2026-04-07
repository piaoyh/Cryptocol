// Copyright 2026 PARK Youngho.
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

use crate::random::Random_Generic;

#[allow(non_camel_case_types)] 
pub struct PRNG_Creator<const COUNT: u64> {}

impl<const COUNT: u64> PRNG_Creator<COUNT>
{
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
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_SHA3_512::new();
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create();
    /// let num: U512 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_Rijndael::create();
    /// let num: U512 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    /// let mut any = Any_PRNG_Creator_SHA3_512::create();
    /// println!("Any number = {}", any.random_u64());
    /// ```
    pub fn create() -> Random_Generic<COUNT>
    {
        unimplemented!()
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
    /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seeds(0, 0);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seeds(15698731215687456325, 10684237915728469725);
    /// let num: U256 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
    /// 
    /// let mut rand = Random_PRNG_Creator_Rijndael::create_with_seeds(112233445566778899, 998877665544332211);
    /// println!("Random number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_PRNG_Creator_SHA3_512::create_with_seeds(u64::MAX, u64::MAX);
    /// let num: U768 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn create_with_seeds(seed: u64, aux: u64) -> Random_Generic<COUNT>
    {
        unimplemented!()
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
    /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [777777777777_u64, 10500872879054459758_u64, 12_u64, 555555555555_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 741258963_u64];
    /// let aux = [789456123_u64, 15887751380961987625_u64, 789654123_u64, 5_u64, 9632587414_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_arrays(seed, aux);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// ```
    /// 
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 12_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    /// let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_arrays(seed, aux);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_arrays(seed, aux);
    /// let num: U256 = rand.random_biguint();
    /// ```
    /// 
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
    /// 
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_PRNG_Creator_Rijndael::create_with_seed_arrays(seed, aux);
    /// println!("Random number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_arrays(u64::MAX, u64::MAX);
    /// let num: U768 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn create_with_seed_arrays(seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<COUNT> 
    {
        unimplemented!()
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
    /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
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
    /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector(seed_collector);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
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
    /// let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector(seed_collector);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
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
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector(seed_collector);
    /// let num: U256 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    ///
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
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
    /// let mut rand = Random_PRNG_Creator_Rijndael::create_with_seed_collector(seed_collector);
    /// println!("Random number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
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
    /// let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector(seed_collector);
    /// let num: U512 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn create_with_seed_collector(seed_collector: fn() -> [u64; 8]) -> Random_Generic<COUNT>
    {
        unimplemented!()
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
    /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
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
    /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seeds(seed_collector, 0, 0);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
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
    /// let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector_seeds(seed_collector, u64::MAX, u64::MAX);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
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
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector_seeds(seed_collector, 15698731215687456325, 10684237915728469725);
    /// let num: U256 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
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
    /// let mut rand = Random_PRNG_Creator_Rijndael::create_with_seed_collector_seeds(seed_collector);
    /// println!("Any number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
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
    /// let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector_seeds(seed_collector, u64::MAX, u64::MAX);
    /// let num: U768 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn create_with_seed_collector_seeds(seed_collector: fn() -> [u64; 8], seed: u64, aux: u64) -> Random_Generic<COUNT>
    {
        unimplemented!()
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
    /// # Example 1 for Random_PRNG_Creator_BIG_KECCAK_1024
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_BIG_KECCAK_1024;
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
    /// let mut rand = Random_PRNG_Creator_BIG_KECCAK_1024::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    /// let num: U1024 = rand.random_with_msb_set_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 2 for Random_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA3_512;
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
    /// let seed = [123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 12_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    /// let mut rand = Random_PRNG_Creator_SHA3_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    /// let num: U768 = rand.random_odd_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 3 for Random_PRNG_Creator_SHA2_512
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_SHA2_512;
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
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_PRNG_Creator_SHA2_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    /// let num: U256 = rand.random_biguint();
    /// println!("Random number = {}", num);
    /// ```
    /// 
    /// # Example 4 for Random_PRNG_Creator_Rijndael
    /// ```
    /// use cryptocol::random::Random_PRNG_Creator_Rijndael;
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
    /// let seed = [10500872879054459758_u64, 12_u64, 123456789_u64, 987654321_u64, 852648791354687_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [15887751380961987625_u64, 789456123_u64, 9632587414_u64, 789654123_u64, 5_u64, 58976541235_u64, 9513574682_u64, 369258147_u64];
    /// let mut rand = Random_PRNG_Creator_Rijndael::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    /// println!("Random number = {}", rand.random_u32());
    /// ```
    /// 
    /// # Example 5 for Any_PRNG_Creator_SHA3_512
    /// ```
    /// use cryptocol::random::Any_PRNG_Creator_SHA3_512;
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
    /// let seed = [12_u64, 123456789_u64, 852648791354687_u64, 10500872879054459758_u64, 987654321_u64, 555555555555_u64, 777777777777_u64, 741258963_u64];
    /// let aux = [9513574682_u64, 9632587414_u64, 15887751380961987625_u64, 789456123_u64,58976541235_u64, 9513574682_u64, 369258147_u64, 789654123_u64, 5_u64];
    /// let mut any = Any_PRNG_Creator_SHA3_512::create_with_seed_collector_seed_arrays(seed_collector, seed, aux);
    /// let num: U512 = any.random_odd_biguint();
    /// println!("Any number = {}", num);
    /// ```
    #[inline]
    pub fn create_with_seed_collector_seed_arrays(seed_collector: fn() -> [u64; 8], seed: [u64; 8], aux: [u64; 8]) -> Random_Generic<COUNT>
    {
        unimplemented!()
    }
}