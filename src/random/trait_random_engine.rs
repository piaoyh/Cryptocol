// Copyright 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


pub(super) const SALT: u64 = 0x9999_9999_9999_9999;


/// The supporting trait for `Random_Generic`
/// `Random_Generic` uses whatever object that has this trait for pseudo-random
/// number generator engine. So, if you plug any hash algorithm or any
/// symmetric-key crytographic algorithm, which implement this trait, in
/// `Random_Generic` , `Random_Generic` will use the object as its pseudo-random
/// number generator engine.
/// You will hardly use the object that has this trait except in the case
/// that you use it in order to plug it in the `Random_Generic`.
/// 
/// # How to use OsRng with Random_Engine and Random_Generic
/// This is a simple illustration to use OsRng with this module. It is assumed
/// that you will have `main.rs` and `os_rng.rs`.
/// 
/// First, you have to include additional dependencies in your Cargo.toml
/// as following example.
/// 
/// ## Example 1
/// ```
/// [dependencies]
/// cryptocol = "0.9.2"
/// rand = { version = "0.9", features = ["getrandom"] }
/// ```
/// It is good if you keep the version number of each crate as latest version.
/// 
/// Second, you need to make a new empty rust source file in proper folder.
/// Let's say the empty rust source file to be `os_rng.rs` and to be located
/// in the same folder where main.rs is located. In the file `os_rng.rs`,
/// you have to import some `struct`s as following example.
/// 
/// ## Example 2
/// ```
/// use rand::{ rngs, RngCore };
/// use cryptocol::random::{ Random_Engine, Random_Generic };
/// ```
/// 
/// Third, you have to make an empty struct `OsRng` in `os_rng.rs`
/// as following example.
/// 
/// ## Example 3
/// ```
/// pub struct OsRng
/// {
///     pub fn new() -> Random_Generic<340282366920938463463374607431768211455>
///     {
///         Random_Generic::<340282366920938463463374607431768211455>::new_with(rngs::OsRng, rngs::OsRng)
///     }
/// }
/// ```
/// 
/// Fourth, you are supposed to make implementation of trait Random_Engine
/// for the empty struct `OsRng` in `os_rng.rs` as following example.
/// 
/// ## Example 4
/// ```
/// impl Random_Engine for OsRng
/// {
///     #[inline]
///     fn harvest(&mut self, _: u128, _: &[u64; 8]) -> [u64; 8]
///     {
///         [
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64()
///         ]
///     }
/// }
/// ```
/// 
/// Fifth, you can define user-defined data type for your convenience
/// in `os_rng.rs` as following example.
/// 
/// ## Example 5
/// ```
/// pub type Random_OsRng = Random_Generic<OsRng>;
/// ```
/// 
/// If you correctly follow the above-instructions, your `os_rng.rs` will
/// look like as Example 6.
/// 
/// ## Example 6 (os_rng.rs)
/// ```
/// use rand::{ rngs, RngCore };
/// use cryptocol::random::{ Random_Engine, Random_Generic };
/// 
/// pub struct OsRng;
/// 
/// impl Random_Engine for OsRng
/// {
///     #[inline]
///     fn harvest(&mut self, _: u128, _: &[u64; 8]) -> [u64; 8]
///     {
///         [
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64(),
///             rngs::OsRng.next_u64(), rngs::OsRng.next_u64()
///         ]
///     }
/// }
/// 
/// pub type Random_OsRng = Random_Generic;
/// ```
/// 
/// Now, you are very ready to use `Random_OsRng` in your own project. And,
/// all the methods of Random_OsRng has been automagically implemented and
/// you can use them for free. In other source files of your project,
/// you are supposed to import `Random_OsRng`.
/// 
/// In the following example, it is assumed that `os_rng.rs` is placed in
/// the same folder where `main.rs` is located. The following example shows
/// how to use `Random_OsRng` in your `main.rs`.
/// 
/// ## Example 7 (main.rs)
/// ```
/// use super::trait_impl_for_OsRng::Random_OsRng;
/// 
/// let mut r = Random_OsRng::new();
/// println!("Random_OsRng u8 = {}", r.random_u8());
/// println!("Random_OsRng u16 = {}", r.random_u16());
/// println!("Random_OsRng u32 = {}", r.random_u32());
/// println!("Random_OsRng u64 = {}", r.random_u64());
/// println!("Random_OsRng u128 = {}", r.random_u128());
/// println!("Random_OsRng under 123456789 = {}", r.random_under_uint_(123456789_u64));
/// println!("Random_OsRng prime number = {}", r.random_prime_using_Miller_Rabin_uint::<u128>(5));
/// println!("Random_OsRng BigUInt = {}", r.random_BigUInt::<u64, 8>());
/// println!("Random_OsRng odd BigUInt = {}", r.random_odd_BigUInt::<u64, 8>());
/// println!("Random_OsRng BigUInt prime number = {}", r.random_prime_using_Miller_Rabin_BigUInt::<u64, 8>(5));
/// ```
/// 
/// Now, you are ready to embed OsRng in this module and use it in any kind of
/// your projects.
#[allow(non_camel_case_types)]
pub trait Random_Engine
{
    // fn sow_array<T, const N: usize>(&mut self, message: &[T; N])
    /// Provides new seeds for `self`.
    /// 
    /// # Argument
    /// `message` is the new seeds for `self`.
    /// 
    /// # Example 1
    /// ```
    /// impl<const COUNT: u128> Random_Generic<COUNT>
    /// {
    ///     pub fn new() -> Self
    ///     {
    ///         if COUNT == 0
    ///             { panic!("COUNT should be greater than 0."); }
    ///     
    ///         let mut main_generator = Box::new(SHA2_512::new());
    ///         let mut aux_generator = Box::new(SHA2_512::new());
    ///         let seed_array = Self::collect_seed();
    ///         let aux_array = Self::collect_seed();
    ///         main_generator.sow_array(&seed_array);
    ///         aux_generator.sow_array(&aux_array);
    ///         Self
    ///         {
    ///             seed_array,
    ///             aux_array,
    ///             count: COUNT,
    ///             main_generator,
    ///             aux_generator,
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// impl<const COUNT: u128> Random_Generic<COUNT>
    /// {
    ///     fn change_count(&mut self)
    ///     {
    ///         if self.is_restarted()
    ///         {
    ///             self.count = COUNT;
    ///         }
    ///         self.count = self.count.wrapping_sub(1);
    ///         if self.is_restarted()
    ///         {
    ///             self.main_generator.sow_array(&Self::collect_seed());
    ///             self.aux_generator.sow_array(&Self::collect_seed());
    ///         }
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn sow_array(&mut self, message: &[u64; 8], original: &[u64; 8]);

    // fn harvest(&mut self, count: u128) -> [u64; 8]
    /// Outputs the pseudo-random number array.
    /// 
    /// # Argument
    /// `count` is `u128`-typed. If `count` is `0`, the direction of
    /// its pseudo-random number sequence is changed so that the period of the
    /// pseudo-random number sequence may not repeated.
    /// 
    /// # Example 1
    /// ```
    /// impl<const COUNT: u128> Random_Generic<COUNT>
    /// {
    ///     fn produce_seed(&mut self)
    ///     {
    ///         self.change_count();
    ///         self.seed_array = self.main_generator.harvest(self.count, &self.seed_array);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// impl<const COUNT: u128> Random_Generic<COUNT>
    /// {
    ///     fn produce_aux(&mut self)
    ///     {
    ///         self.change_count();
    ///         self.aux_array = self.aux_generator.harvest(self.count, &self.aux_array);
    ///     }
    /// }
    /// ```
    fn harvest(&mut self, count: u128, message: &[u64; 8]) -> [u64; 8];
}
