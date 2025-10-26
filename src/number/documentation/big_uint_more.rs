// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]

use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::{ SmallUInt, TraitsBigUInt };

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_other_calculation_uint.rs.
pub struct BigUInt<T, const N: usize>
where T: TraitsBigUInt<T>
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    /*** Addition ***/

    // pub fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    /// Calculates `self` + `rhs`,
    /// and returns an addition result `self` + `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_add()](struct@BigUInt#method.checked_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.checked_add_uint(1_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} + 1 = {}", a_biguint, num);
    ///         assert_eq!(num, U512::max());
    ///         assert_eq!(num.is_overflow(), false);
    ///         assert_eq!(num.is_underflow(), false);
    ///         assert_eq!(num.is_divided_by_zero(), false);
    ///         assert_eq!(num.is_infinity(), false);
    ///         assert_eq!(num.is_undefined(), false);
    ///         assert_eq!(num.is_left_carry(), false);
    ///         assert_eq!(num.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{} + 1 = overflow", a_biguint);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.checked_add_uint(2_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} + 2 = {}", a_biguint, num);
    ///     },
    ///     None => {
    ///         println!("{} + 2 = overflow", a_biguint);
    ///         assert_eq!(res, None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.checked_add_uint(3_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} + 2 = {}", a_biguint, num);
    ///     },
    ///     None => {
    ///         println!("{} + 2 = overflow", a_biguint);
    ///         assert_eq!(res, None);
    ///     }
    /// }
    /// ```
    pub fn checked_add_uint<U>(&self, _rhs: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` + `rhs`, assuming overflow cannot occur,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    ///   only when you are sure that overflow will not occur.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur at current
    /// operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [unchecked_add()](struct@BigUInt#method.unchecked_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.unchecked_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res, UU64::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_add_uint(2_u8);
    /// ```
    #[inline]
    pub fn unchecked_add_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if the result is less than or equal
    /// to the maximum value of `Self`. If the sum `self` + `rhs` is greater
    /// than the maximum value it returns the maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_add()](struct@BigUInt#method.saturating_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    /// let res = a_biguint.saturating_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    /// let res = a_biguint.saturating_add_uint(2_u8);
    /// println!("{} + 2 = {}", a_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    /// let res = a_biguint.saturating_add_uint(3_u8);
    /// println!("{} + 3 = {}", a_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_add_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_add_assign_uint<U>(&mut self, rhs: T)
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_add_assign()](struct@BigUInt#method.saturating_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_add_assign_uint(1_u8);
    /// println!("After a_biguint.saturating_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_add_assign_uint(2_u8);
    /// println!("After a_biguint.saturating_add_assign_uint(2_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_add_assign_uint(3_u8);
    /// println!("After a_biguint.saturating_add_assign_uint(3_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_add_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_add_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` + `rhs`,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` + `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_add_uint() in release mode.
    /// - This method works as if it was unchecked_add_uint() in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the addition does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_add()](struct@BigUInt#method.safe_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().safe_sub_uint(1_u8);
    /// let res = a_biguint.safe_add_uint(1_u8);
    /// println!("{} + 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let a_biguint = U512::max().safe_sub_uint(1_u8);
    ///     let res = a_biguint.safe_add_uint(2_u8);
    ///     println!("{} + 2 = {}", a_biguint, res);
    ///     assert_eq!(res.to_string(), "0");
    ///     assert_eq!(res.is_overflow(), true);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let a_biguint = U512::max().safe_sub_uint(1_u8);
    ///     let res = a_biguint.safe_add_uint(3_u8);
    ///     println!("{} + 3 = {}", a_biguint, res);
    ///     assert_eq!(res.to_string(), "1");
    ///     assert_eq!(res.is_overflow(), true);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u128);
    /// 
    ///     let _a_biguint = U512::max().wrapping_sub_uint(1_u8);
    ///     let _res = _a_biguint.safe_add_uint(2_u8);
    ///     let _res = _a_biguint.safe_add_uint(3_u8);
    /// }
    /// ```
    pub fn safe_add_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_add_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self` + `rhs`,
    /// and assigns an addition result `self` + `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` + `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_add_assign_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_add_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the addition does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_add_assign()](struct@BigUInt#method.safe_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU64::max().safe_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.safe_add_assign_uint(1_u8);
    /// println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     a_biguint.safe_add_assign_uint(1_u8);
    ///     println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "0");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     a_biguint.safe_add_assign_uint(1_u8);
    ///     println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "1");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u8);
    /// 
    ///     let mut _a_biguint = U512::max();
    ///     _a_biguint.safe_add_assign_uint(1_u8);
    ///     _a_biguint.safe_add_assign_uint(1_u8);
    /// }
    /// ```
    pub fn safe_add_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_add(&self, rhs: &Self) -> Option<Self>
    /// Calculates `self` + `rhs`,
    /// and returns an addition result `self` + `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` wrapped by `Some` of enum `Option`
    /// if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_add_uint()](struct@BigUInt#method.checked_add_uint)
    /// is a bit faster than this method `checked_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_add_uint()](struct@BigUInt#method.checked_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.checked_add(&one_biguint);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, r, r.is_overflow());
    ///             assert_eq!(r, U512::max());
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error: Overflow"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let b_biguint = U512::max();
    /// let one_biguint = U512::one();
    /// let res = b_biguint.checked_add(&one_biguint);
    /// match res
    /// {
    ///     Some(r) => { println!("{} + {} = {}, overflow = {}", b_biguint, one_biguint, r, r.is_overflow()); },
    ///     None => { 
    ///             println!("Error: Overflow");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_add(&self, _rhs: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_add(&self, rhs: &Self) -> Self
    /// Calculates `self` + `rhs`, assuming overflow cannot occur,
    /// and returns an addition result `self` + `rhs`.
    ///
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    ///   only when you are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if overflow did not occur.
    /// Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_add_uint()](struct@BigUInt#method.unchecked_add_uint)
    /// is a bit faster than this method `unchecked_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_add_uint()](struct@BigUInt#method.unchecked_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.unchecked_add(&one_biguint);
    /// println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, res, res.is_overflow());
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _b_biguint = U512::max();
    /// let _one_biguint = U512::one();
    /// // It will panic.
    /// let res = _b_biguint.unchecked_add(&_one_biguint);
    /// ```
    #[inline]
    pub fn unchecked_add(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_add(&self, rhs: &Self) -> Self
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the sum `self` + `rhs` if the result is less than or equal
    /// to the maximum value of `Self`. If the sum `self` + `rhs` is greater
    /// than the maximum value it returns the maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_add_uint()](struct@BigUInt#method.saturating_add_uint)
    /// is a bit faster than this method `saturating_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_uint()](struct@BigUInt#method.saturating_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.saturating_add(&one_biguint);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let b_biguint = U512::max();
    /// let one_biguint = U512::one();
    /// let res = b_biguint.saturating_add(&one_biguint);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_add(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_add_assign(&mut self, rhs: &Self)
    /// Calculates `self` + `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_add_assign_uint()](struct@BigUInt#method.saturating_add_assign_uint)
    /// is a bit faster than this method `saturating_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_assign_uint()](struct@BigUInt#method.saturating_add_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one = U512::one();
    /// a_biguint.saturating_sub_assign(&one);
    /// println!("After a_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::zero();
    /// println!("Originally, b_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one = U512::one();
    /// a_biguint.saturating_sub_assign(&one);
    /// println!("After b_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_uint(5_u8);
    /// println!("Originally, b_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let ten = U512::from_uint(10_u8);
    /// a_biguint.saturating_sub_assign(&ten);
    /// println!("After b_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_add_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_add(& self, rhs: &Self) -> Self
    /// Calculates `self` + `rhs`,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` + `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_add() in release mode.
    /// - This method works as if it was unchecked_add() in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the addition does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().safe_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.safe_add(&one_biguint);
    /// println!("{} + {} = {}", a_biguint, one_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let b_biguint = U512::max();
    ///     let one_biguint = U512::one();
    ///     let res = b_biguint.safe_add(&one_biguint);
    ///     println!("{} + {} = {}", b_biguint, one_biguint, res);
    ///     assert_eq!(res.to_string(), "0");
    ///     assert_eq!(res.is_overflow(), true);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let c_biguint = U512::zero();
    /// let one_biguint = U512::one();
    /// let res = c_biguint.safe_add(&one_biguint);
    /// println!("{} + {} = {}", c_biguint, one_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u128);
    /// 
    ///     let _b_biguint = U512::max();
    ///     let _one_biguint = U512::one();
    ///     let _res = _b_biguint.safe_add(&_one_biguint);
    /// } 
    /// ```
    pub fn safe_add(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_add_assign(&mut self, rhs: &Self)
    /// Calculates `self` + `rhs`,
    /// and assigns an addition result `self` + `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` + `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_add_assign_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_add_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the addition does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_add_assign_uint()](struct@BigUInt#method.safe_add_assign_uint)
    /// is a bit faster than this method `safe_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_assign()](struct@BigUInt#method.safe_add_assign).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::max().safe_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one_biguint = U512::one();
    /// a_biguint.safe_add_assign(&one_biguint);
    /// println!("After a_biguint.safe_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U512::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = U512::max();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let one_biguint = U512::one();
    ///     a_biguint.safe_add_assign(&one_biguint);
    ///     println!("After a_biguint.safe_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "0");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     a_biguint.safe_add_assign(&one_biguint);
    ///     println!("After a_biguint.safe_add_assign(&U512::one()),\ta_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "1");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u8);
    /// 
    ///     let mut _a_biguint = U512::max();
    ///     let _one_biguint = U512::one();
    ///     _a_biguint.safe_add_assign(&_one_biguint);
    ///     _a_biguint.safe_add_assign(&_one_biguint);
    /// }
    /// ```
    pub fn safe_add_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Subtraction ***/


    // pub fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    /// Calculates `self` - `rhs`,
    /// and returns an subtraction result `self` - `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` wrapped by `Some` of enum
    /// `Option` if underflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if underflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_sub()](struct@BigUInt#method.checked_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.checked_sub_uint(1_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} - 1 = {}", a_biguint, num);
    ///         assert_eq!(num.to_string(), "0");
    ///         assert_eq!(num.is_underflow(), false);
    ///         assert_eq!(num.is_overflow(), false);
    ///         assert_eq!(num.is_divided_by_zero(), false);
    ///         assert_eq!(num.is_infinity(), false);
    ///         assert_eq!(num.is_undefined(), false);
    ///         assert_eq!(num.is_left_carry(), false);
    ///         assert_eq!(num.is_right_carry(), false);
    ///     },
    ///     None => {
    ///         println!("{} - 1 = overflow", a_biguint);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.checked_sub_uint(2_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} - 2 = {}", a_biguint, num);
    ///     },
    ///     None => {
    ///         println!("{} - 2 = overflow", a_biguint);
    ///         assert_eq!(res, None);
    ///     }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.checked_sub_uint(3_u8);
    /// match res
    /// {
    ///     Some(num) => {
    ///         println!("{} - 3 = {}", a_biguint, num);
    ///     },
    ///     None => {
    ///         println!("{} - 3 = overflow", a_biguint);
    ///         assert_eq!(res, None);
    ///     }
    /// }
    /// ```
    pub fn checked_sub_uint<U>(&self, _rhs: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` - `rhs`, assuming underflow cannot occur,
    /// and returns an subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow occurred, it will panic. So, use this method
    ///   only when you are sure that underflow will not occur.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur at
    /// current operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if underflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [unchecked_sub()](struct@BigUInt#method.unchecked_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU64::one();
    /// let res = a_biguint.unchecked_sub_uint(1_u8);
    /// println!("{} - 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU64::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_sub_uint(2_u8);
    /// ```
    #[inline]
    pub fn unchecked_sub_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` - `rhs`,
    /// saturating at `0` instead of underflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if the result is less than
    /// or equal to `0` of `Self`. If the difference `self` - `rhs`
    /// is less than `0`, it returns `0`.
    /// 
    /// # Features
    /// - This method saturates when it reaches `0` of `Self`.
    /// - It does not set `UNDERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_sub()](struct@BigUInt#method.saturating_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.saturating_sub_uint(1_u8);
    /// println!("{} - 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.saturating_sub_uint(2_u8);
    /// println!("{} - 2 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.saturating_sub_uint(3_u8);
    /// println!("{} - 3 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_sub_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_sub_assign_uint<U>(&mut self, rhs: T)
    /// Calculates `self` - `rhs`,
    /// saturating at `0` instead of underflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches `0` of `Self`.
    /// - It does not set `UNDERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_sub_assign()](struct@BigUInt#method.saturating_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_sub_assign_uint(1_u8);
    /// println!("After a_biguint.saturating_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_sub_assign_uint(2_u8);
    /// println!("After a_biguint.saturating_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU64::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_sub_assign_uint(3_u8);
    /// println!("After a_biguint.saturating_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_sub_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn abs_diff_uint<U>(&self, other: U) -> Self
    /// Calculates the absolute difference between `self` and `other`.
    /// 
    /// # Arguments
    /// `other` is to be compared to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Features
    /// - It calculates the absolute difference between `self` and `other`.
    /// - It does not change the flags either `OVERFLOW` or `UNDERFLOW`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [abs_diff()](struct@BigUInt#method.abs_diff)
    /// is proper rather than this method `abs_diff_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    /// let res = a_biguint.abs_diff_uint(num_uint);
    /// println!("| {} - {} | = {}", a_biguint, num_uint, res);
    /// assert_eq!(res.to_string(), "115792089237316195423570985008687907853066609319396769656704041438214461985024");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "12345678_9ABCDEF0_12345678_9ABCDEF0";
    /// let a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    /// let res = a_biguint.abs_diff_uint(num_uint);
    /// println!("| {} - {} | = {}", a_biguint, num_uint, res);
    /// assert_eq!(res.to_string(), "179177489040527647888749252028162707471");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str3 = "9900AABB_CCDDEEFF_9900AABB_CCDDEEFF";
    /// let a_biguint = U256::from_str_radix(num_str3, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    /// let res = a_biguint.abs_diff_uint(num_uint);
    /// println!("| {} - {} | = {}", a_biguint, num_uint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn abs_diff_uint<U>(&self, _other: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_sub_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` - `rhs`,
    /// and returns an subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` - `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction in release mode.
    /// - If underflow happened, the flag `UNDERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_sub_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_sub_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the subtraction does not cause underflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if underflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the underflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_sub()](struct@BigUInt#method.safe_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.safe_sub_uint(1_u8);
    /// println!("{} - 1 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let a_biguint = U512::one();
    ///     let res = a_biguint.safe_sub_uint(2_u8);
    ///     println!("{} - 2 = {}", a_biguint, res);
    ///     assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///     assert_eq!(res.is_underflow(), true);
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// 
    ///     let a_biguint = U512::one();
    ///     let res = a_biguint.safe_sub_uint(3_u8);
    ///     println!("{} - 3 = {}", a_biguint, res);
    ///     assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    ///     assert_eq!(res.is_underflow(), true);
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u8);
    /// 
    ///     let _a_biguint = U512::one();
    ///     // It will panic.
    ///     let _res = _a_biguint.safe_sub_uint(2_u8);
    /// 
    ///     let _a_biguint = U512::one();
    ///     // It will panic.
    ///     let _res = _a_biguint.safe_sub_uint(3_u8);
    /// }
    /// ```
    pub fn safe_sub_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_sub_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self` - `rhs`,
    /// and assigns an subtraction result `self` - `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` - `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction in release mode.
    /// - If underflow happened, the flag `UNDERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_sub_assign_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_sub_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the subtraction does not cause underflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if underflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the underflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_sub_assign()](struct@BigUInt#method.safe_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.safe_sub_assign_uint(1_u8);
    /// println!("After a_biguint.safe_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = UU64::one();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "1");
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     a_biguint.safe_sub_assign_uint(2_u8);
    ///     println!("After a_biguint.safe_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///     assert_eq!(a_biguint.is_underflow(), true);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = UU64::one();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "1");
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     a_biguint.safe_sub_assign_uint(3_u8);
    ///     println!("After a_biguint.safe_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    ///     assert_eq!(a_biguint.is_underflow(), true);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     a_biguint.safe_sub_assign_uint(1_u8);
    ///     println!("After a_biguint.safe_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    ///     assert_eq!(a_biguint.is_underflow(), true);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u16);
    /// 
    ///     let mut _a_biguint = UU64::one();
    ///     _a_biguint.safe_sub_assign_uint(2_u8);
    /// 
    ///     let mut _a_biguint = UU64::one();
    ///     _a_biguint.safe_sub_assign_uint(3_u8);
    /// }
    /// ```
    pub fn safe_sub_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_sub(&self, rhs: &Self) -> Option<Self>
    /// Calculates `self` - `rhs`,
    /// and returns an subtraction result `self` - `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` wrapped by `Some` of enum
    /// `Option` if underflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if underflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_sub_uint()](struct@BigUInt#method.checked_sub_uint)
    /// is a bit faster than this method `checked_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_sub_uint()](struct@BigUInt#method.checked_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.checked_sub(&U512::one());
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} - 1 = {}, underflow = {}", a_biguint, r, r.is_underflow());
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(res.is_left_carry(), false);
    ///             assert_eq!(res.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error: Underflow"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let b_biguint = U512::max();
    /// let res = b_biguint.checked_add(&U512::one());
    /// match res
    /// {
    ///     Some(r) => { println!("{} - 1 = {}, underflow = {}", b_biguint, r, r.is_underflow()); },
    ///     None => { 
    ///             println!("Error: Underflow");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_sub(&self, _rhs: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_sub(&self, rhs: &Self) -> Self
    /// Calculates `self` - `rhs`, assuming underflow cannot occur,
    /// and returns an subtraction result `self` - `rhs`.
    ///
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow occurred, it will panic. So, use this method
    ///   only when you are sure that underflow will not occur.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if underflow did not occur at
    /// current operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if underflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_sub_uint()](struct@BigUInt#method.unchecked_sub_uint)
    /// is a bit faster than this method `unchecked_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_sub_uint()](struct@BigUInt#method.unchecked_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.unchecked_sub(&U512::one());
    /// println!("{} - 1 = {}, underflow = {}", a_biguint, res, res.is_underflow());
    /// assert_eq!(res, U512::zero());
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _b_biguint = U512::zero();
    /// // It will panic.
    /// let res = _b_biguint.unchecked_sub(&U512::one());
    /// ```
    #[inline]
    pub fn unchecked_sub(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_sub(&self, rhs: &Self) -> Self
    /// Calculates `self` - `rhs`,
    /// saturating at `0` instead of underflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the difference `self` - `rhs` if the result is less than
    /// or equal to `0` of `Self`. If the difference `self` - `rhs`
    /// is less than `0`, it returns `0`.
    /// 
    /// # Features
    /// - This method saturates when it reaches `0` of `Self`.
    /// - It does not set `UNDERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_sub_uint()](struct@BigUInt#method.saturating_sub_uint)
    /// is a bit faster than this method `saturating_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub_uint()](struct@BigUInt#method.saturating_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::one();
    /// let one = U512::one();
    /// let res = a_biguint.saturating_sub(&one);
    /// println!("{} - {} = {}", a_biguint, one, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let b_biguint = U512::zero();
    /// let one = U512::one();
    /// let res = b_biguint.saturating_sub(&one);
    /// println!("{} - {} = {}", b_biguint, one, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let b_biguint = U512::from_uint(5_u8);
    /// let ten = U512::from_uint(10_u8);
    /// let res = b_biguint.saturating_sub(&ten);
    /// println!("{} - {} = {}", b_biguint, ten, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_sub(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_sub_assign(&mut self, rhs: &Self)
    /// Calculates `self` - `rhs`,
    /// saturating at `0` instead of underflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches `0` of `Self`.
    /// - It does not set `UNDERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_sub_assign_uint()](struct@BigUInt#method.saturating_sub_assign_uint)
    /// is a bit faster than this method `saturating_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub_assign_uint()](struct@BigUInt#method.saturating_sub_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one = U512::one();
    /// a_biguint.saturating_sub_assign(&one);
    /// println!("After a_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_sub_assign(&U512::one());
    /// println!("After a_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_uint(5_u8);
    /// println!("Originally, b_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let ten = U512::from_uint(10_u8);
    /// a_biguint.saturating_sub_assign(&ten);
    /// println!("After b_biguint.saturating_sub_assign(&U512::from_uint(10_u8)), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_sub_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn abs_diff(&self, other: &Self) -> Self
    /// Calculates the absolute difference between `self` and `other`.
    /// 
    /// # Arguments
    /// `other` is to be compared to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the absolute difference between `self` and `other`.
    /// 
    /// # Features
    /// - It calculates the absolute difference between `self` and `other`.
    /// - It does not change the flags either `OVERFLOW` or `UNDERFLOW`.
    /// 
    /// # Counterpart Method
    /// The method [abs_diff_uint()](struct@BigUInt#method.abs_diff_uint)
    /// is a bit faster than this method `abs_diff()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [abs_diff_uint()](struct@BigUInt#method.abs_diff_uint).
    /// 
    /// # Example
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUInt;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    /// let b_biguint = U256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    /// let c_biguint = a_biguint.abs_diff(&b_biguint);
    /// let d_biguint = b_biguint.abs_diff(&a_biguint);
    /// println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c_biguint);
    /// println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d_biguint);
    /// assert_eq!(c_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(d_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    /// assert_eq!(d_biguint.is_overflow(), false);
    /// assert_eq!(d_biguint.is_underflow(), false);
    /// assert_eq!(d_biguint.is_divided_by_zero(), false);
    /// assert_eq!(d_biguint.is_infinity(), false);
    /// assert_eq!(d_biguint.is_undefined(), false);
    /// assert_eq!(d_biguint.is_left_carry(), false);
    /// assert_eq!(d_biguint.is_right_carry(), false);
    /// ```
    pub fn abs_diff(&self, _other: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_sub(&self, rhs: &Self) -> Self
    /// Calculates `self` - `rhs`,
    /// and returns an subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` - `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction in release mode.
    /// - If underflow happened, the flag `UNDERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_sub()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_sub()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the subtraction does not cause underflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if underflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the underflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_sub_uint()](struct@BigUInt#method.safe_sub_uint)
    /// is a bit faster than this method `safe_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_sub_uint()](struct@BigUInt#method.safe_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let one = U512::one();
    /// let res = a_biguint.safe_sub(&one);
    /// println!("{} - {} = {}", a_biguint, one, res);
    /// assert_eq!(res, U512::zero());
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let b_biguint = U512::zero();
    ///     let one = U512::one();
    ///     let res = b_biguint.safe_sub(&one);
    ///     println!("{} - {} = {}", b_biguint, one, res);
    ///     assert_eq!(res, U512::max());
    ///     assert_eq!(res.is_underflow(), true);
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let c_biguint = U512::max();
    /// let one = U512::one();
    /// let res = c_biguint.safe_sub(&one);
    /// println!("{} - {} = {}", c_biguint, one, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u128);
    /// 
    ///     let _b_biguint = U512::zero();
    ///     let _one = U512::one();
    ///     let _res = _b_biguint.safe_sub(&_one);
    /// }
    /// ```
    pub fn safe_sub(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_sub_assign(&mut self, rhs: &Self)
    /// Calculates `self` - `rhs`,
    /// and assigns an subtraction result `self` - `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If underflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` - `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction in release mode.
    /// - If underflow happened, the flag `UNDERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_sub_assign()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_sub()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the subtraction does not cause underflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if underflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the underflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_sub_assign_uint()](struct@BigUInt#method.safe_sub_assign_uint)
    /// is a bit faster than this method `safe_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_sub_assign_uint()](struct@BigUInt#method.safe_sub_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one = U512::one();
    /// a_biguint.safe_sub_assign(&one);
    /// println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = U512::zero();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let one = U512::one();
    ///     a_biguint.safe_sub_assign(&one);
    ///     println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///     assert_eq!(a_biguint.is_underflow(), true);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::max();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let one = U512::one();
    /// a_biguint.safe_sub_assign(&one);
    /// println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u8);
    /// 
    ///     let mut a_biguint = U512::zero();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let one = U512::one();
    ///     // It will panic.
    ///     a_biguint.safe_sub_assign(&one);
    ///     println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    ///     assert_eq!(a_biguint.is_underflow(), true);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    pub fn safe_sub_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Multiplication ***/


    // pub fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    /// Calculates `self` * `rhs`,
    /// and returns an multiplication result `self` * `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the the multiplication result `self` * `rhs` wrapped by
    /// `Some` of enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_mul()](struct@BigUInt#method.checked_mul)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 5_u16;
    /// let res = a_biguint.checked_mul_uint(b_uint);
    /// match &res
    /// {
    ///     Some(r) => {
    ///             println!("{} X {} = {}", a_biguint, b_uint, r);
    ///             assert_eq!(r.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow happend!"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u16;
    /// let res = a_biguint.checked_mul_uint(b_uint);
    /// match &res
    /// {
    ///     Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
    ///     None => {
    ///         println!("Overflow happend!");
    ///         assert_eq!(res, None);
    ///     },
    /// }
    /// ```
    pub fn checked_mul_uint<U>(&self, _rhs: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` * `rhs`, assuming overflow cannot occur,
    /// and returns an multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    ///   only when you are sure that overflow will not occur.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` if overflow did not
    /// occur at current operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [unchecked_mul()](struct@BigUInt#method.unchecked_mul)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let res = a_biguint.unchecked_mul_uint(5_u8);
    /// println!("{} X {} = {}", a_biguint, 5_u8, res);
    /// assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_mul_uint(248_u8);
    /// ```
    #[inline]
    pub fn unchecked_mul_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    /// Calculates `self` * `rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` if the result is
    /// less than or equal to the maximum value of `Self`. If the sum
    /// `self` + `rhs` is greater than the maximum value it returns the
    /// maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_mul()](struct@BigUInt#method.saturating_mul)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let res = a_biguint.saturating_mul_uint(5_u8);
    /// println!("{} X {} = {}", a_biguint, 5_u8, res);
    /// assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let res = a_biguint.saturating_mul_uint(248_u8);
    /// println!("{} X {} = {}", a_biguint, 248_u8, res);
    /// assert_eq!(res.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_mul_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self` * `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of `self`.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_mul_assign()](struct@BigUInt#method.saturating_mul_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_mul_assign_uint(5_u8);
    /// println!("After a_biguint.saturating_mul_assign_uint(5_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_mul_assign_uint(248_u8);
    /// println!("After a_biguint.saturating_mul_assign_uint(248_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_mul_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_mul_uint<U>(& self, rhs: U) -> Self
    /// Calculates `self` * `rhs`,
    /// and returns an multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` * `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_mul_uint()
    ///   in release mode.
    /// - This method works as if it was unchecked_mul_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the multiplication does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_mul()](struct@BigUInt#method.safe_mul)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u16;
    /// let res = a_biguint.safe_mul_uint(b_uint);
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     let b_uint = 248_u16;
    ///     let res = b_biguint.safe_mul_uint(b_uint);
    ///     println!("{} X {} = {}", b_biguint, b_uint, res);
    ///     assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    ///     assert_eq!(res.is_overflow(), true);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u16);
    /// 
    ///     let _b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     let _b_uint = 248_u16;
    ///     let _res = _b_biguint.safe_mul_uint(_b_uint);
    /// }
    /// ```
    pub fn safe_mul_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_mul_assign_uint<U>(&mut self, rhs: U)
    /// Calculates `self` * `rhs`,
    /// and assigns an multiplication result `self` * `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` * `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_mul_assign_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_mul_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the multiplication does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [safe_mul_assign()](struct@BigUInt#method.safe_mul_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "12380187429816690342769003185818648605085375388281194656994643364900608");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_uint = 248_u16;
    /// a_biguint.safe_mul_assign_uint(b_uint);
    /// println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let b_uint = 248_u16;
    ///     a_biguint.safe_mul_assign_uint(b_uint);
    ///     println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u32);
    /// 
    ///     let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let b_uint = 248_u16;
    ///     // It will panic.
    ///     a_biguint.safe_mul_assign_uint(b_uint);
    ///     println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    pub fn safe_mul_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_mul(&self, _rhs: &Self) -> Option<Self>
    /// Calculates `self` * `rhs`,
    /// and returns an multiplication result `self` * `rhs`
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the the multiplication result `self` * `rhs` wrapped by
    /// `Some` of enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_mul_uint()](struct@BigUInt#method.checked_mul_uint)
    /// is a bit faster than this method `checked_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_mul_uint()](struct@BigUInt#method.checked_mul_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("1876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.checked_mul(&b_biguint);
    /// match &res
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} X {} = {}", a_biguint, b_biguint, r);
    ///             assert_eq!(r.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow happend!"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(5_u8);
    /// let res = a_biguint.checked_mul(&b_biguint);
    /// match &res
    /// {
    ///     Some(r) => { println!("{} X {} = {}", a_biguint, b_biguint, r); },
    ///     None =>
    ///         {
    ///             println!("Overflow happend!");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_mul(&self, _rhs: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_mul(&self, _rhs: &Self) -> Self
    /// Calculates `self` * `rhs`, assuming overflow cannot occur,
    /// and returns an multiplication result `self` * `rhs`.
    ///
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurred, it will panic. So, use this method
    ///   only when you are sure that overflow will not occur. 
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` if overflow did not
    /// occur at current operation. Otherwise, it will panic.
    /// 
    /// # Features
    /// It does not wrap around at the boundary of the `Self` type.
    /// So, if overflow happened, it will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_mul_uint()](struct@BigUInt#method.unchecked_mul_uint)
    /// is a bit faster than this method `unchecked_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_mul_uint()](struct@BigUInt#method.unchecked_mul_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = UU32::from_uint(248_u8);
    /// let res = a_biguint.unchecked_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let _b_biguint = UU32::from_uint(248_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_mul(&_b_biguint);
    /// ```
    #[inline]
    pub fn unchecked_mul(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_mul(&self, _rhs: &Self) -> Self
    /// Calculates `self` * `rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` if the result is
    /// less than or equal to the maximum value of `Self`. If the sum
    /// `self` + `rhs` is greater than the maximum value it returns the
    /// maximum value.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_mul_uint()](struct@BigUInt#method.saturating_mul_uint)
    /// is a bit faster than this method `saturating_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_uint()](struct@BigUInt#method.saturating_mul_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = UU32::from_uint(5_u8);
    /// let res = a_biguint.saturating_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = UU32::from_uint(248_u8);
    /// let res = a_biguint.saturating_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn saturating_mul(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_mul_assign(&mut self, _rhs: &Self)
    /// Calculates `self` * `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - This method saturates when it reaches the maximum value of `Self`.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_mul_assign_uint()](struct@BigUInt#method.saturating_mul_assign_uint)
    /// is a bit faster than this method `saturating_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_assign_uint()](struct@BigUInt#method.saturating_mul_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = UU32::from_uint(5_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_mul_assign(&b_biguint);
    /// println!("After a_biguint.saturating_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = UU32::from_uint(248_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_mul_assign(&b_biguint);
    /// println!("After a_biguint.saturating_mul_assign_uint(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn saturating_mul_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_mul(& self, _rhs: &Self) -> Self
    /// Calculates `self` * `rhs`,
    /// and returns an multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` * `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_mul_uint()
    ///   in release mode.
    /// - This method works as if it was unchecked_mul_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the multiplication does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.safe_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     let b_biguint = U256::from_uint(248_u8);
    ///     let res = a_biguint.safe_mul(&b_biguint);
    ///     println!("{} X {} = {}", a_biguint, b_biguint, res);
    ///     assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    ///     assert_eq!(res.is_overflow(), true);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), false);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), false);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u16);
    ///     
    ///     let _a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     let _b_biguint = U256::from_uint(248_u8);
    ///     let _res = _a_biguint.safe_mul(&_b_biguint);
    /// }
    /// ```
    pub fn safe_mul(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn safe_mul_assign(&mut self, _rhs: &Self)
    /// Calculates `self` * `rhs`,
    /// and assigns an multiplication result `self` * `rhs` back to `self`.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow happened, it will panic in debug mode.
    /// 
    /// # Output
    /// It returns `self` * `rhs`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication in release mode.
    /// - If overflow happened, the flag `OVERFLOW` of `self`
    ///   will be set in release mode, but it will panic in debug mode.
    /// - This method works as if it was wrapping_mul_assign_uint()
    ///   in release mode.
    /// - This method works as if it was *self = unchecked_mul_uint()
    ///   in debug mode.
    /// 
    /// # Why does this method work differently between release mode and debug mode?
    /// If you want to make sure that the multiplication does not cause overflow
    /// at all, you may want to use this method. When you test your code that
    /// uses this method in debug mode, this method will cause panic if overflow
    /// happens with this method. It will help you find the bug in your code.
    /// After you fix all the bugs you found, all the overflow checking code
    /// which may be unnecessary now and only slow down your code will be
    /// removed from your code in release mode.
    /// 
    /// # Counterpart Method
    /// The method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](struct@BigUInt#method.safe_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint.safe_mul_assign(&b_biguint);
    /// println!("After a_biguint.safe_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// #[cfg(not(debug_assertions))]
    /// {
    ///     let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     println!("Originally, a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.is_overflow(), false);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let b_biguint = U256::from_uint(248_u8);
    ///     a_biguint.safe_mul_assign(&b_biguint);
    ///     println!("After c_biguint.safe_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    ///     assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    ///     assert_eq!(a_biguint.is_overflow(), true);
    ///     assert_eq!(a_biguint.is_underflow(), false);
    ///     assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_infinity(), false);
    ///     assert_eq!(a_biguint.is_undefined(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// }
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// #[cfg(debug_assertions)]
    /// {
    ///     use cryptocol::number::BigUint_More;
    ///     use cryptocol::define_utypes_with;
    ///     define_utypes_with!(u32);
    /// 
    ///     let mut _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    ///     let _b_biguint = U256::from_uint(248_u8);
    ///     _a_biguint.safe_mul_assign(&_b_biguint);
    /// }
    /// ```
    pub fn safe_mul_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Division ***/


    // pub fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    /// Divides `self` by `rhs`,
    /// and returns the quotient wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a quotient of `BigUInt` type wrapped by `Some` of
    /// enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// - This division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_div()](struct@BigUInt#method.checked_div)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let quotient = dividend.checked_div_uint(divisor);
    /// match quotient
    /// {
    ///     Some(q) =>
    ///         {
    ///             println!("{} / {} = {}", dividend, divisor, q);
    ///             assert_eq!(q.to_string(), "1419043551905275201680884938348044216837079832");
    ///             assert_eq!(q.is_overflow(), false);
    ///             assert_eq!(q.is_underflow(), false);
    ///             assert_eq!(q.is_infinity(), false);
    ///             assert_eq!(q.is_divided_by_zero(), false);
    ///             assert_eq!(q.is_undefined(), false);
    ///             assert_eq!(q.is_left_carry(), false);
    ///             assert_eq!(q.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u8;
    /// let quotient = dividend.checked_div_uint(divisor);
    /// match quotient
    /// {
    ///     Some(q) =>
    ///         {
    ///             println!("{} / {} = {}", dividend, divisor, q);
    ///             assert_eq!(q.to_string(), "0");
    ///             assert_eq!(q.is_overflow(), false);
    ///             assert_eq!(q.is_underflow(), false);
    ///             assert_eq!(q.is_infinity(), false);
    ///             assert_eq!(q.is_divided_by_zero(), false);
    ///             assert_eq!(q.is_undefined(), false);
    ///             assert_eq!(q.is_left_carry(), false);
    ///             assert_eq!(q.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let quotient = dividend.checked_div_uint(divisor);
    /// match quotient
    /// {
    ///     Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(quotient, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let quotient = dividend.checked_div_uint(divisor);
    /// match quotient
    /// {
    ///     Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(quotient, None);
    ///         },
    /// }
    pub fn checked_div_uint<U>(&self, _rhs: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_div()](struct@BigUInt#method.unchecked_div)
    /// is proper rather than this method `unchecked_div_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let quotient = dividend.unchecked_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let quotient = dividend.unchecked_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic.
    /// let quotient = _dividend.unchecked_div_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic.
    /// let quotient = _dividend.unchecked_div_uint(_divisor);
    /// ```
    #[inline]
    pub fn unchecked_div_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_div_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the quotient of `self` / `rhs`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Output
    /// It returns the quotient of `BigUInt` type as a result of
    /// `self` / `rhs`.
    /// 
    /// # Features
    /// - The quotient would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_div()](struct@BigUInt#method.saturating_div)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let quotient = dividend.saturating_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let quotient = _dividend.saturating_div_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let quotient = _dividend.saturating_div_uint(_divisor);
    /// ```
    pub fn saturating_div_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_div_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the quotient of `self` / `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - The quotient would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_div_assign()](struct@BigUInt#method.saturating_div_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// let divisor = 87_u8;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_div_assign_uint(divisor);
    /// println!("After a_biguint.saturating_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_div_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = 0_u8;
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_div_assign_uint(_divisor);
    /// ```
    pub fn saturating_div_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_div(&self, _rhs: &Self) -> Option<Self>
    /// Divides `self` by `rhs`,
    /// and returns the quotient wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a quotient of `BigUInt` type wrapped by `Some` of
    /// enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// - This division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_div_uint()](struct@BigUInt#method.checked_div_uint)
    /// is a bit faster than this method `checked_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_div_uint()](struct@BigUInt#method.checked_div_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.checked_div(&divisor);
    /// match quotient
    /// {
    ///     Some(q) =>
    ///         {
    ///             println!("{} / {} = {}", dividend, divisor, q);
    ///             assert_eq!(q.to_string(), "1419043551905275201680884938348044216837079832");
    ///             assert_eq!(q.is_overflow(), false);
    ///             assert_eq!(q.is_underflow(), false);
    ///             assert_eq!(q.is_infinity(), false);
    ///             assert_eq!(q.is_undefined(), false);
    ///             assert_eq!(q.is_divided_by_zero(), false);
    ///             assert_eq!(q.is_left_carry(), false);
    ///             assert_eq!(q.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    ///
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.checked_div(&divisor);
    /// match quotient
    /// {
    ///     Some(q) =>
    ///         {
    ///             println!("{} / {} = {}", dividend, divisor, q);
    ///             assert_eq!(q.to_string(), "0");
    ///             assert_eq!(q.is_overflow(), false);
    ///             assert_eq!(q.is_underflow(), false);
    ///             assert_eq!(q.is_infinity(), false);
    ///             assert_eq!(q.is_undefined(), false);
    ///             assert_eq!(q.is_divided_by_zero(), false);
    ///             assert_eq!(q.is_left_carry(), false);
    ///             assert_eq!(q.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::zero();
    /// let quotient = dividend.checked_div(&divisor);
    /// match quotient
    /// {
    ///     Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(quotient, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::zero();
    /// let quotient = dividend.checked_div(&divisor);
    /// match quotient
    /// {
    ///     Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(quotient, None);
    ///         },
    /// }
    /// ```
    pub fn checked_div(&self, _rhs: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_div(&self, _rhs: &Self) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_div_uint()](struct@BigUInt#method.unchecked_div_uint)
    /// is a bit faster than this method `unchecked_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_div_uint()](struct@BigUInt#method.unchecked_div_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let quotient = dividend.unchecked_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let quotient = dividend.unchecked_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic.
    /// let quotient = _dividend.unchecked_div(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic.
    /// let quotient = _dividend.unchecked_div(&_divisor);
    /// ```
    #[inline]
    pub fn unchecked_div(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_div(&self, _rhs: &Self) -> Self
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the quotient of `self` / `rhs`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Output
    /// It returns the quotient of `BigUInt` type as a result of
    /// `self` / `rhs`.
    /// 
    /// # Features
    /// - The quotient would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_div_uint()](struct@BigUInt#method.saturating_div_uint)
    /// is a bit faster than this method `saturating_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_uint()](struct@BigUInt#method.saturating_div_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.saturating_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let quotient = dividend.saturating_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.saturating_div(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let quotient = _dividend.saturating_div(&_divisor);
    /// ```
    pub fn saturating_div(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_div_assign(&mut self, _rhs: &Self)
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the quotient of `self` / `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - The quotient would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_div_assign_uint()](struct@BigUInt#method.saturating_div_assign_uint)
    /// is a bit faster than this method `saturating_div_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_assign_uint()](struct@BigUInt#method.saturating_div_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_div_assign(&divisor);
    /// println!("After a_biguint.saturating_div_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_div_assign(&divisor);
    /// println!("After a_biguint.saturating_div_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_div_assign(&_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = UU32::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_div_assign(&_divisor);
    /// ```
    pub fn saturating_div_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_rem_uint<U>(&self, rhs: U) -> Option<Self>
    /// Divides `self` by `rhs`,
    /// and returns the remainder wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a remainder of `BigUInt` type wrapped by `Some` of
    /// enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// - This division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [checked_rem()](struct@BigUInt#method.checked_rem)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} % {} = {}", dividend, divisor, r);
    ///             assert_eq!(r.to_string(), "8");
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 0_u8;
    /// let remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} % {} = {}", dividend, divisor, r);
    ///             assert_eq!(r.to_string(), "0");
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(remainder, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 0_u8;
    /// let remainder = dividend.checked_rem_uint(divisor);
    /// match remainder
    /// {
    ///     Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(remainder, None);
    ///         },
    /// }
    /// ```
    pub fn checked_rem_uint<U>(&self, _rhs: U) -> Option<U>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_rem_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [unchecked_rem()](struct@BigUInt#method.unchecked_rem)
    /// is proper rather than this method `unchecked_rem_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.unchecked_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::zero();
    /// let mut divisor = 87_u8;
    /// let mut remainder = dividend.unchecked_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic.
    /// let remainder = _dividend.unchecked_rem_uint(_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u8;
    /// // It will panic.
    /// let remainder = _dividend.unchecked_rem_uint(_divisor);
    /// ```
    #[inline]
    pub fn unchecked_rem_uint<U>(&self, _rhs: U) -> U
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_rem_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the remainder of `self` / `rhs`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Output
    /// It returns the remainder of `BigUInt` type as a result of
    /// `self` % `rhs`.
    /// 
    /// # Features
    /// - The remainder would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_rem()](struct@BigUInt#method.saturating_rem)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let remainder = dividend.saturating_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let remainder = dividend.saturating_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let remainder = _dividend.saturating_rem_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let remainder = _dividend.saturating_rem_uint(_divisor);
    /// ```
    pub fn saturating_rem_uint<U>(&self, _rhs: U) -> U
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the remainder of `self` / `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - The remainder would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [saturating_rem_assign()](struct@BigUInt#method.saturating_rem_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u16;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_rem_assign_uint(divisor);
    /// println!("After a_biguint.saturating_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::zero();
    /// let divisor = 87_u16;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.saturating_rem_assign_uint(divisor);
    /// println!("After a_biguint.saturating_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u16;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_rem_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = 0_u16;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_rem_assign_uint(_divisor);
    /// ```
    pub fn saturating_rem_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_rem(&self, _rhs: &Self) -> Option<Self>
    /// Divides `self` by `rhs`,
    /// and returns the remainder wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a remainder of `BigUInt` type wrapped by `Some` of
    /// enum `Option` if overflow did not occur at current operation.
    /// Otherwise, it returns `None` of enum `Option`.
    /// 
    /// # Features
    /// - This division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_rem_uint()](struct@BigUInt#method.checked_rem_uint)
    /// is a bit faster than this method `checked_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_rem_uint()](struct@BigUInt#method.checked_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.checked_rem(&divisor);
    /// match remainder
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} % {} = {}", dividend, divisor, r);
    ///             assert_eq!(r.to_string(), "8");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.checked_rem(&divisor);
    /// match remainder
    /// {
    ///     Some(r) =>
    ///         {
    ///             println!("{} % {} = {}", dividend, divisor, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Divided By Zero"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::zero();
    /// let remainder = dividend.checked_rem(&divisor);
    /// match remainder
    /// {
    ///     Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(remainder, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::zero();
    /// let remainder = dividend.checked_rem(&divisor);
    /// match remainder
    /// {
    ///     Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
    ///     None =>
    ///         {
    ///             println!("Divided By Zero");
    ///             assert_eq!(remainder, None);
    ///         },
    /// }
    /// ```
    pub fn checked_rem(&self, _rhs: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_rem(&self, _rhs: &Self) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_rem_uint()](struct@BigUInt#method.unchecked_rem_uint)
    /// is a bit faster than this method `unchecked_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_rem_uint()](struct@BigUInt#method.unchecked_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let remainder = dividend.unchecked_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let remainder = dividend.unchecked_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic.
    /// let remainder = _dividend.unchecked_rem(&_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// // It will panic.
    /// let remainder = _dividend.unchecked_rem(&_divisor);
    /// ```
    #[inline]
    pub fn unchecked_rem(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_rem(&self, _rhs: &Self) -> Self
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the remainder of `self` / `rhs`.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Output
    /// It returns the remainder of `BigUInt` type as a result of
    /// `self` % `rhs`.
    /// 
    /// # Features
    /// - The remainder would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_rem_uint()](struct@BigUInt#method.saturating_rem_uint)
    /// is a bit faster than this method `saturating_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_uint()](struct@BigUInt#method.saturating_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.saturating_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.saturating_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let remainder = _dividend.saturating_rem(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let remainder = _dividend.saturating_rem(&_divisor);
    /// ```
    pub fn saturating_rem(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_rem_assign(&mut self, _rhs: &Self)
    /// Divides `self` by `rhs`,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assigns the remainder of `self` / `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Features
    /// - The remainder would never overflow so that it can not saturate.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [saturating_rem_assign_uint()](struct@BigUInt#method.saturating_rem_assign_uint)
    /// is a bit faster than this method `saturating_rem_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_assign_uint()](struct@BigUInt#method.saturating_rem_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint.saturating_rem_assign(&divisor);
    /// println!("After a_biguint.saturating_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint.saturating_rem_assign(&divisor);
    /// println!("After a_biguint.saturating_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_rem_assign(&_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.saturating_rem_assign(&_divisor);
    /// ```
    pub fn saturating_rem_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** MULTIPLE OPERATIONS ***/

    // pub fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and returns the result.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is a primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is zero.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`.
    /// However, if overflow occurs, it returns the value wrapped around.
    /// 
    /// # Features
    /// The result will be the smallest value greater than or equal to self,
    /// which is a multiple of `rhs`. However, if overflow occurs,
    /// the result will be the value wrapped around.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [next_multiple_of()](struct@BigUInt#method.next_multiple_of)
    /// is proper rather than this method `next_multiple_of_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = 586478_u32;
    /// let multiple = a_biguint.next_multiple_of_uint(num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::max();
    /// let num = 586478_u32;
    /// let multiple = a_biguint.next_multiple_of_uint(num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "448670");
    /// assert_eq!(multiple.is_overflow(), true);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = 0_u8;
    /// // It will panic.
    /// let multiple = _a_biguint.next_multiple_of_uint(_num);
    /// ```
    pub fn next_multiple_of_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is a primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is zero.
    /// 
    /// # Features
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is is a multiple of `rhs`.
    ///   However, if overflow occurs, `self` will be the value wrapped around.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [next_multiple_of_assign()](struct@BigUInt#method.next_multiple_of_assign)
    /// is proper rather than this method `next_multiple_of_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = 586478_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.next_multiple_of_assign_uint(num);
    /// println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let num = 586478_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.next_multiple_of_assign_uint(num);
    /// println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "448670");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_str_radix("123456789012345678901234567890123697594", 16).unwrap();
    /// let _num = 0_u8;
    /// // It will panic.
    /// _a_biguint.next_multiple_of_assign_uint(_num);
    /// ```
    pub fn next_multiple_of_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_multiple_of_uint<U>(&self, rhs: U) -> bool
    /// Returns `true` if `self` is a multiple of `rhs`, and `false` otherwise.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is a primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - If `self` is a multiple of `rhs`, it returns `true`, and
    ///   otherwise, it returns `false`.
    /// - If both `self` and `rhs` are `zero`, it returns `true`.
    /// - If `self` is not `zero` and `rhs` is `zero`, it returns `false`.
    /// 
    /// # Features
    /// - This function is equivalent to `self` % rhs == 0,
    ///   except that it will not panic for `rhs` == 0.
    /// - If `rhs` is `zero` and `self` is `zero`, it returns `true`.
    /// - If `rhs` is `zero` and `self` is not `zero`, it returns `false`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [is_multiple_of()](struct@BigUInt#method.is_multiple_of)
    /// is proper rather than this method `is_multiple_of_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = 100_u8;
    /// let ans = a_biguint.is_multiple_of_uint(rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = 99_u8;
    /// let ans = a_biguint.is_multiple_of_uint(rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 3 for rhs == 0 and self != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = 0_u8;
    /// let ans = a_biguint.is_multiple_of_uint(rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 4 for rhs == 0 and self == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let rhs = 0_u8;
    /// let ans = a_biguint.is_multiple_of_uint(rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    pub fn is_multiple_of_uint<U>(&self, _rhs: U) -> bool
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn next_multiple_of(&self, rhs: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and returns the result.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is zero.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`.
    /// However, if overflow occurs, it returns the value wrapped around.
    /// 
    /// # Features
    /// The result will be the smallest value greater than or equal to self,
    /// which is a multiple of `rhs`. However, if overflow occurs,
    /// the result will be the value wrapped around.
    /// 
    /// # Counterpart Method
    /// The method
    /// [next_multiple_of_uint()](struct@BigUInt#method.next_multiple_of_uint)
    /// is a bit faster than this method `next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [next_multiple_of_uint()](struct@BigUInt#method.next_multiple_of_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(586478_u32);
    /// let multiple = a_biguint.next_multiple_of(&num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let num = U256::from(586478_u32);
    /// let multiple = a_biguint.next_multiple_of(&num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "448670");
    /// assert_eq!(multiple.is_overflow(), true);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::zero();
    /// // It will panic.
    /// let _multiple = _a_biguint.next_multiple_of(&_num);
    /// ```
    pub fn next_multiple_of(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn next_multiple_of_assign(&mut self, rhs: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if rhs is zero.
    /// 
    /// # Features
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is is a multiple of `rhs`.
    ///   However, if overflow occurs, `self` will be the value wrapped around.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [next_multiple_of_assign_uint()](struct@BigUInt#method.next_multiple_of_assign_uint)
    /// is a bit faster than this method `next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [next_multiple_of_assign_uint()](struct@BigUInt#method.next_multiple_of_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let num = UU32::from(586478_u32);
    /// a_biguint.next_multiple_of_assign(&num);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::max();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let num = UU32::from(586478_u32);
    /// a_biguint.next_multiple_of_assign(&num);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "448670");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = UU32::zero();
    /// // It will panic.
    /// _a_biguint.next_multiple_of_assign(&_num);
    /// ```
    pub fn next_multiple_of_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_multiple_of(&self, rhs: &Self) -> bool
    /// Returns `true` if `self` is a multiple of `rhs`, and `false` otherwise.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - If `self` is a multiple of `rhs`, it returns `true`, and
    ///   otherwise, it returns `false`.
    /// - If both `self` and `rhs` are `zero`, it returns `true`.
    /// - If `self` is not `zero` and `rhs` is `zero`, it returns `false`.
    /// 
    /// # Features
    /// - This function is equivalent to `self` % rhs == 0,
    ///   except that it will not panic for `rhs` == 0.
    /// - If `rhs` is `zero` and `self` is `zero`, it returns `true`.
    /// - If `rhs` is `zero` and `self` is not `zero`, it returns `false`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [is_next_multiple_of_uint()](struct@BigUInt#method.is_next_multiple_of_uint)
    /// is a bit faster than this method `is_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [is_next_multiple_of_uint()](struct@BigUInt#method.is_next_multiple_of_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::from(100_u8);
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::from(99_u8);
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 3 for rhs == 0 and self != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::zero();
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 4 for rhs == 0 and self == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let rhs = U256::zero();
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    pub fn is_multiple_of(&self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    
    /*** MIDDLE POINT ****/


    // pub fn midpoint_uint<U>(&self, rhs: &Self) -> Self
    /// Calculates the middle point of `self` and `rhs`,
    /// and returns the result value.
    /// 
    /// # Arguments
    /// `rhs` is another point to get the middle point, and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns (`self` + `rhs`) / 2
    /// in a sufficiently-large signed integral type.
    /// 
    /// # Features
    /// - a.midpoint(&b) works as if (a + b) / 2 in
    ///   a sufficiently-large signed integral type.
    /// - This implies that the result is always rounded towards 0,
    ///   and that no overflow will ever occur.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [midpoint()](struct@BigUInt#method.midpoint)
    /// is proper rather than this method `midpoint_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("8888866666444442222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// let c_biguint = a_biguint.midpoint_uint(b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "4444433333222221111155555555555555555555555555555555555");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    /// let b_biguint = 66666555554444433333222221111100000_u128;
    /// let c_biguint = a_biguint.midpoint_uint(b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    /// let b_biguint = 66666555554444433333222221111100001_u128;
    /// let c_biguint = a_biguint.midpoint_uint(b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    /// let b_biguint = 66666555554444433333222221111100000_u128;
    /// let c_biguint = a_biguint.midpoint_uint(b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    /// let b_biguint = 66666555554444433333222221111100001_u128;
    /// let c_biguint = a_biguint.midpoint_uint(b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn midpoint_uint<U>(&self, _rhs: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn midpoint_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the middle point of `self` and `rhs`,
    /// and assigns the result value to `self`.
    /// 
    /// # Arguments
    /// `rhs` is another point to get the middle point, and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - a.midpoint_assign_uint(&b) works as if a = (a + b) / 2 in
    ///   a sufficiently-large signed integral type.
    /// - This implies that the result is always rounded towards 0,
    ///   and that no overflow will ever occur.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [midpoint_assign()](struct@BigUInt#method.midpoint_assign)
    /// is proper rather than this method `midpoint_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("8888866666444442222233333444445555566666777778888899999").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// a_biguint.midpoint_assign_uint(b_biguint);
    /// println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4444433333222221111155555555555555555555555555555555555");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = 66666555554444433333222221111100000_u128;
    /// a_biguint.midpoint_assign_uint(b_biguint);
    /// println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = 66666555554444433333222221111100001_u128;
    /// a_biguint.midpoint_assign_uint(b_biguint);
    /// println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = 66666555554444433333222221111100000_u128;
    /// a_biguint.midpoint_assign_uint(b_biguint);
    /// println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = 66666555554444433333222221111100001_u128;
    /// a_biguint.midpoint_assign_uint(b_biguint);
    /// println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn midpoint_assign_uint<U>(&mut self, _rhs: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn midpoint(&self, rhs: &Self) -> Self
    /// Calculates the middle point of `self` and `rhs`,
    /// and returns the result value.
    /// 
    /// # Arguments
    /// `rhs` is another point to get the middle point, and is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns (`self` + `rhs`) / 2
    /// in a sufficiently-large signed integral type.
    /// 
    /// # Features
    /// - a.midpoint(&b) works as if (a + b) / 2 in
    ///   a sufficiently-large signed integral type.
    /// - This implies that the result is always rounded towards 0,
    ///   and that no overflow will ever occur.
    /// 
    /// # Counterpart Method
    /// The method [midpoint_uint()](struct@BigUInt#method.midpoint_uint)
    /// is more efficient than this method `midpoint()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_uint()](struct@BigUInt#method.midpoint_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("999998888877777666665555544444333332222211111").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "44444333332222211111555555555555555555555555555555555555555555555");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    pub fn midpoint(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn midpoint_assign(&mut self, rhs: &Self)
    /// Calculates the middle point of `self` and `rhs`,
    /// and assigns the result value to `self`.
    /// 
    /// # Arguments
    /// `rhs` is another point to get the middle point, and is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - a.midpoint_assign(&b) works as if a = (a + b) / 2 in
    ///   a sufficiently-large signed integral type.
    /// - This implies that the result is always rounded towards 0,
    ///   and that no overflow will ever occur.
    /// 
    /// # Counterpart Method
    /// The method [midpoint_assign_uint()](struct@BigUInt#method.midpoint_assign_uint)
    /// is more efficient than this method `midpoint_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_assign_uint()](struct@BigUInt#method.midpoint_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("999998888877777666665555544444333332222211111").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "44444333332222211111555555555555555555555555555555555555555555555");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn midpoint_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** METHODS FOR EXPONENTIATION AND LOGARITHM ***/

    // fn checked_pow_uint<U>(&self, exp: U) -> Option<Self>
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result wrapped by `Some` of enum `Option`.
    /// The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the result of `self` raised to the power of `exp`, using
    ///   exponentiation of type `BigUInt` by squaring,
    ///   wrapping around at the boundary of the type `Self`,
    ///   wrapped by `Some` of enum `Option` if overflow does not occur.
    /// - If overflow occurs, it returns `None` of enum `Option`.
    /// - If both `self` and `rhs` are zero, the result is mathematically
    ///   undefined so this method returns `None`.
    /// 
    /// # Features
    /// - Checked wrapping (modular) exponentiation. 
    /// - If overflowing happens, it returns `None` of enum `Option`.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [checked_pow()](struct@BigUInt#method.checked_pow)
    /// is proper rather than this method `checked_pow_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1000000000000000000000000000000");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 100_u8;
    /// let res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Overflow");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = 30_u8;
    /// let res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "0");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 0_u8;
    /// let res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = 0_u8;
    /// let res = a_biguint.checked_pow_uint(exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Undefined");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_pow_uint<U>(&self, _exp: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }
 
    // pub fn unchecked_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result. The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurs, it will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`
    /// if overflow does not occur. If overflow occurs, it will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [unchecked_pow()](struct@BigUInt#method.unchecked_pow)
    /// is proper rather than this method `unchecked_pow_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 30_u8;
    /// let mut res = a_biguint.unchecked_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let mut exp = 0_u8;
    /// let mut res = a_biguint.unchecked_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::zero();
    /// let mut exp = 30_u8;
    /// let mut res = a_biguint.unchecked_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU32::from_uint(10_u8);
    /// let _exp = 100_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow_uint(_exp);
    /// 
    /// let _a_biguint = UU32::zero();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow_uint(_exp);
    /// ```
    pub fn unchecked_pow_uint<U>(&self, _exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the result. The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// It returns the maximum value instead of overflowing.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [saturating_pow()](struct@BigUInt#method.saturating_pow)
    /// is proper rather than this method `saturating_pow_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 30_u8;
    /// let mut res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = 100_u8;
    /// let res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let mut exp = 0_u8;
    /// let mut res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let mut exp = 30_u8;
    /// let mut res = a_biguint.saturating_pow_uint(exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = UU32::zero();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.saturating_pow_uint(_exp);
    /// ```
    pub fn saturating_pow_uint<U>(&self, _exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow_assign_uint<U>(&self, exp: U)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assign the result to `self` back.
    /// The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - `self` will be the maximum value instead of overflowing.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [saturating_pow_assign()](struct@BigUInt#method.saturating_pow_assign)
    /// is proper rather than this method `saturating_pow_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(1000000000000000000000000000000_u128);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint, UU32::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(100_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 0_u8;
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// a_biguint.saturating_pow_assign_uint(exp);
    /// println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _exp = 0_u8;
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// _a_biguint.saturating_pow_assign_uint(_exp);
    /// ```
    pub fn saturating_pow_assign_uint<U>(&mut self, _exp: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn iroot_uint<U>(&self, exp: U) -> Self
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self` and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned.
    /// 
    /// # Features
    /// If `exp` is greater than zero and `self` is greater than 1,
    /// the result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [iroot()](struct@BigUInt#method.iroot)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 8_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "100000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 65_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "9");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 212_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 213_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = u128::MAX;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = 6_u8;
    /// let res = a_biguint.iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.iroot_uint(_exp);
    /// 
    /// let _a_biguint = U256::one();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.iroot_uint(_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.iroot_uint(_exp);
    /// ```
    pub fn iroot_uint<U>(&self, _exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn iroot_assign_uint<U>(&mut self, exp: U)
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self` and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Features
    /// - If the exact value of `exp`-th root of `self` can be expressed with
    ///   `Self`-typed unsigned integer, it will be assigned to `self`.
    ///   Otherwise, the `Self`-typed biggest unsigned integer that is less
    ///   than the exact value of `exp`-th root of `self` will be assigned
    ///   to `self`.
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [iroot_assign()](struct@BigUInt#method.iroot_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 8_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "100000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 65_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "9");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 212_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 213_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = u128::MAX;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 6_u8;
    /// a_biguint.iroot_assign_uint(exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// _a_biguint.iroot_assign_uint(_exp);
    /// 
    /// let mut _a_biguint = U256::one();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// _a_biguint.iroot_assign_uint(_exp);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// _a_biguint.iroot_assign_uint(_exp);
    /// ```
    pub fn iroot_assign_uint<U>(&mut self, _exp: U)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_iroot_uint<U>(&self, exp: U) -> Option<Self>
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value, wrapped by `Some` of enum `Option`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self` and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned wrapped by `Some`
    /// of enum `Option`.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Features
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - If `exp` is `0`, this method returns `None`.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [checked_iroot()](struct@BigUInt#method.checked_iroot)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 8_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The third root of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "100000000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 65_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The square root of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "9");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 212_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The square root of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 213_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The square root of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = u128::MAX;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The square root of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = 6_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 0_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::one();
    /// let exp = 0_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = 0_u8;
    /// let res = a_biguint.checked_iroot_uint(exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_iroot_uint<U>(&self, _exp: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_iroot_uint<U>(&self, exp: U) -> Self
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self` and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned.
    ///
    /// # Features
    /// If `exp` is greater than zero and `self` is greater than 1,
    /// the result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [unchecked_root()](struct@BigUInt#method.unchecked_root)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 8_u8;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The third root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "100000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 65_u8;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "9");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 212_u8;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 213_u8;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = u128::MAX;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = 6_u8;
    /// let res = a_biguint.unchecked_iroot_uint(exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot_uint(_exp);
    /// 
    /// let _a_biguint = U256::one();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot_uint(_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot_uint(_exp);
    /// ```
    #[inline]
    pub fn unchecked_iroot_uint<U>(&self, _exp: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    /// Calculates the logarithm of the number with respect to `base`, rounded
    /// down, and returns the result wrapped with enum `Some` of `Option`.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the logarithm of the number with respect to `base`,
    ///   rounded down, wrapped with enum `Some` of `Option`.
    /// - It returns `None` if `self` is zero.
    /// - It returns `None` if `base` is either `0` or `1`.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [checked_ilog2()](struct@BigUInt#method.checked_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [checked_ilog10()](struct@BigUInt#method.checked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// let res = a_biguint.checked_ilog_uint(base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 10_u8;
    /// let res = a_biguint.checked_ilog_uint(base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "64");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 0_u8;
    /// let res = a_biguint.checked_ilog_uint(base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 1_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 0_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 1_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let base = 0_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let base = 1_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog_uint<U>(&self, _base: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog_uint<U>(&self, base: U) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`,and `u128`.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    /// 
    /// # Output
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [unchecked_ilog2()](struct@BigUInt#method.unchecked_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [unchecked_ilog10()](struct@BigUInt#method.unchecked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// let res = a_biguint.unchecked_ilog_uint(base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 10_u8;
    /// let res = a_biguint.unchecked_ilog_uint(base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "64");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::one();
    /// let base = 6_u8;
    /// let res = a_biguint.unchecked_ilog_uint(base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let _a_biguint = U256::one();
    /// let _base = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = 1_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = 6_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = 1_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::one();
    /// let _base = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// 
    /// let _a_biguint = U256::one();
    /// let _base = 1_u8;
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog_uint(_base);
    /// ```
    #[inline]
    pub fn unchecked_ilog_uint<U>(&self, _base: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_pow(&self, exp: &Self) -> Option<Self>
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    /// 
    /// # Output
    /// - It returns the result of `self` raised to the power of `exp`, using
    ///   exponentiation of type `BigUInt` by squaring,
    ///   wrapping around at the boundary of the type `Self`,
    ///   wrapped by `Some` of enum `Option` if overflow does not occur.
    /// - If overflow occurs, it returns `None` of enum `Option`.
    /// - If both `self` and `rhs` are zero, the result is mathematically
    ///   undefined so this method returns `None`.
    /// 
    /// # Features
    /// - Checked wrapping (modular) exponentiation. 
    /// - If overflowing happens, it returns `None` of enum `Option`.
    /// - If both `self` and `rhs` are zero, the result is mathematically
    ///   undefined so this method returns `None`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint) is a bit
    /// faster than this method `checked_pow()` when the exponent `exp` is
    /// primitive unsigned integral data type such as u8, u16, u32, u64, and
    /// u128. If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1000000000000000000000000000000");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Overflow");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "0");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::zero();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Undefined");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_pow(&self, _exp: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_pow(&self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If overflow occurs, it will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [pow_uint()](struct@BigUInt#method.pow_uint) is more
    /// efficient than this method `pow()` when the exponent `exp` is primitive
    /// unsigned integral data type such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](struct@BigUInt#method.pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = U256::from_uint(30_u8);
    /// let res = a_biguint.unchecked_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for 123456789012345678901234567890123456789 ** 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = U256::zero();
    /// let res = a_biguint.unchecked_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for 0 ** 123456789012345678901234567890123456789
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.unchecked_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_uint(10_u8);
    /// let _exp = U256::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow(&_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow(&_exp);
    /// ```
    pub fn unchecked_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow(&self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method [saturating_pow_uint()](struct@BigUInt#method.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](struct@BigUInt#method.pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.saturating_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for wrapping (modular) exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.saturating_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for 123456789012345678901234567890123456789 ** 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.saturating_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for 0 ** 123456789012345678901234567890123456789
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.saturating_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.saturating_pow(&_exp);
    /// ```
    pub fn saturating_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - `self` will be the maximum value instead of overflowing.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// The method [saturating_pow_assign_uint()](struct@BigUInt#method.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow_assign()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [saturating_pow_assign_uint()](struct@BigUInt#method.saturating_pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for wrapping (modular) exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(100_u8);
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for 123456789012345678901234567890123456789 ** 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::zero();
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for 0 ** 123456789012345678901234567890123456789
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// _a_biguint.saturating_pow_assign(&_exp);
    /// ```
    pub fn saturating_pow_assign(&mut self, _exp: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_iroot(&self, exp: &Self) -> Option<Self>
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value, wrapped by `Some` of enum `Option`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned wrapped by `Some`
    /// of enum `Option`.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Features
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - If `exp` is `0`, this method returns `None`.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [checked_iroot_uint()](struct@BigUInt#method.checked_iroot_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "100000000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "9");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::one();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_iroot(&self, _exp: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_iroot(&self, exp: &Self) -> Self
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `exp` is `0`, it will panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned.
    /// 
    /// # Features
    /// If `exp` is greater than zero and `self` is greater than 1,
    /// the result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [unchecked_iroot_uint()](struct@BigUInt#method.unchecked_iroot_uint)
    /// is a bit faster than this method `unchecked_iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [unchecked_iroot_uint()](struct@BigUInt#method.unchecked_iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "100000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "9");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// 
    /// let _a_biguint = U256::one();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// ```
    pub fn unchecked_iroot(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`, rounded
    /// down, and returns the result wrapped with enum `Some` of `Option`.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// - It returns the logarithm of the number with respect to `base`,
    ///   rounded down, wrapped with enum `Some` of `Option`.
    /// - It returns `None` if `self` is zero.
    /// - It returns `None` if `base` is either `0` or `1`.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [checked_ilog2()](#tymethod.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](#tymethod.checked_ilog10) can produce
    /// results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "64");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::zero();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::one();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::zero();
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::one();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog(&self, _base: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Output
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [unchecked_ilog2()](#tymethod.unchecked_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [unchecked_ilog10()](#tymethod.unchecked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.unchecked_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "2");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.unchecked_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "64");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.unchecked_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::from_uint(6_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// ```
    pub fn unchecked_ilog(&self, _base: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog2(&self) -> Option<Self>
    /// Calculates the base 2 logarithm of the number.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down,
    /// wrapped by `Some` of enum `Option` if `self` is not zero.
    /// It returns `None` if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [checked_ilog_uint()](#tymethod.checked_ilog_uint)
    /// can produce results of the base other than 2, and
    /// [checked_ilog10()](#tymethod.checked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The base 10 logarithm of {} is {}.", a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog2(&self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog2(&self) -> Self
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [unchecked_ilog_uint()](#tymethod.unchecked_ilog_uint)
    /// can produce results of the base other than 2, and
    /// [unchecked_ilog10()](#tymethod.unchecked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.unchecked_ilog2();
    /// println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.unchecked_ilog2();
    /// println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.unchecked_ilog2();
    /// println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog2();
    /// ```
    pub fn unchecked_ilog2(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog10(&self) -> Option<Self>
    /// Calculates the base 10 logarithm of the number.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down,
    /// wrapped by `Some` of enum `Option` if `self` is not zero.
    /// It returns `None` if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [checked_ilog_uint()](#tymethod.checked_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [checked_ilog2()](#tymethod.checked_ilog2)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.checked_ilog10();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 10 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "4");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", 12345_u32, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The base 10 logarithm of {} is {}.", a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog10(&self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog10(&self) -> Self
    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [unchecked_ilog_uint()](#tymethod.unchecked_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [unchecked_ilog2()](#tymethod.unchecked_ilog2)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(12345_u32);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Example
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog10();
    /// ```
    pub fn unchecked_ilog10(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_shift_left<U>(&self, _n: U) -> Option<Self>
    /// Shift left the field `number: [T;N]` to the left by `n`,
    /// and returns the result, wrapped by `some` of enum `Option`.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// - If n is less than the size of the type `T` * N * 8,
    ///   it returns the left-shifted version of `self`, which is shifted to the
    ///   left by `n` bits, wrapped by `some` of enum `Option`.
    /// - If `n` is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it returns `None`.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), true);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), true);
    ///             assert_eq!(r.is_right_carry(), false);
    ///        },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.checked_shift_left(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    pub fn checked_shift_left<U>(&self, _n: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_shift_left<U>(&self, _n: U) -> Self
    /// Shift left the field `number: [T;N]` to the left by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift left' means 'move left all bits'. So, if `10011010` is shifted
    /// left by 2, it will be `01101000`, for example.
    /// 
    /// # Output
    /// It returns the left-shifted version of `self`, which is shifted to the
    /// left by `n` bits.
    /// 
    /// # Left Carry
    /// 'A left-carry occurs' means that a bit `1` is pushed out
    /// during shift-left operation.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `n` is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it will panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
    /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), true);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
    /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.unchecked_shift_left(n);
    /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), true);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 256_u16;
    /// // It will panic!
    /// let _res = _a_biguint.unchecked_shift_left(_n);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 512_u16;
    /// // It will panic!
    /// let _res = _a_biguint.unchecked_shift_left(_n);
    /// ```
    #[inline]
    pub fn unchecked_shift_left<U>(&self, _n: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_shift_right<U>(&self, _n: U) -> Option<Self>
    /// Shift right the field `number: [T;N]` to the right by `n`,
    /// and returns the result, wrapped by `some` of enum `Option`.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits, wrapped by `some` of enum `Option`.
    /// If n is greater than or equal to the size of the type `T` * N * 8,
    /// all bits will be gone. So, it returns `None`.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), true);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///             assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), true);
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 256_u16;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 512_u16;
    /// let res = a_biguint.checked_shift_right(n);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
    ///         },
    ///     None => {
    ///             println!("All bits are gone!");
    ///             assert_eq!(res, None);
    ///         }
    /// }
    /// ```
    pub fn checked_shift_right<U>(&self, _n: U) -> Option<Self>
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_shift_right<U>(&self, _n: U) -> Self
    /// shifts the field `number: [T;N]` to the right by `n`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `n` indicates how many bits this method shift `self` left by,
    /// and is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    /// 
    /// # Features
    /// 'Shift right' means 'move right all bits'. So, if `10011010` is shifted
    /// right by 2, it will be `00100110`, for example.
    /// 
    /// # Output
    /// It returns the right-shifted version of `self`. which is shifted to the
    /// right by `n` bits.
    /// 
    /// # Right Carry
    /// 'A right-carry occurs' means that a bit `1` is pushed out
    /// during shift-right operation.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If n is greater than or equal to the size of the type `T` * N * 8,
    ///   all bits will be gone. So, it will panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 3_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
    /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    /// let n = 4_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
    /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    /// let n = 128_u8;
    /// let res = a_biguint.unchecked_shift_right(n);
    /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), true);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 256_u16;
    /// // It will panic!
    /// let res = _a_biguint.unchecked_shift_right(_n);
    /// 
    /// let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let _n = 512_u16;
    /// // It will panic!
    /// let res = _a_biguint.unchecked_shift_right(_n);
    /// ```
    #[inline]
    pub fn unchecked_shift_right<U>(&self, _n: U) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }
}