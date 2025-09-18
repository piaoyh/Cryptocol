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
