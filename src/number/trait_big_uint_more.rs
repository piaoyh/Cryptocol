// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]


use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::SmallUInt;




/// # Introduction
/// Trait `BigUInt_More` is for BigUInt
///
/// # Quick start
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::BigUInt_More` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::BigUInt_More;
/// ```
/// If you import (use) `cryptocol::number::BigUInt_More`, all the methods of
/// `BigUInt_More` are available immediately and automagically, as if such
/// primitive data types had the methods from the begining.
/// 
/// ## Example 2
/// ```
/// // to do
/// ```
#[allow(non_camel_case_types)]
pub trait BigUInt_More<T, const N: usize> : Clone + Sized //+ Display + + ToString
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
/*** ADDITION UINT ***/

    // fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
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
    /// [checked_add()](#tymethod.checked_add)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.checked_add_uint)
    fn checked_add_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_add_uint<U>(&self, rhs: U) -> Self
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
    /// [unchecked_add()](#tymethod.unchecked_add)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.unchecked_add_uint)
    fn unchecked_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_add_uint<U>(&self, rhs: U) -> Self
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
    /// [saturating_add()](#tymethod.saturating_add)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_add_uint)
    fn saturating_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_add_assign_uint<U>(&mut self, rhs: T)
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
    /// [saturating_add_assign()](#tymethod.saturating_add_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_add_assign_uint)
    fn saturating_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_add_uint<U>(&self, rhs: U) -> Self
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
    /// [safe_add()](#tymethod.safe_add)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_add_uint)
    fn safe_add_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_add_assign_uint<U>(&mut self, rhs: U)
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
    /// [safe_add_assign()](#tymethod.safe_add_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_add_assign_uint)
    fn safe_add_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** SUBTRACTION UINT ***/
    
    // fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
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
    /// [checked_sub()](#tymethod.checked_sub)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.checked_sub_uint)
    fn checked_sub_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
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
    /// [unchecked_sub()](#tymethod.unchecked_sub)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.unchecked_sub_uint)
    fn unchecked_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_sub_uint<U>(&self, rhs: U) -> Self
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
    /// [saturating_sub()](#tymethod.saturating_sub)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_sub_uint)
    fn saturating_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_sub_assign_uint<U>(&mut self, rhs: T)
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
    /// [saturating_sub_assign()](#tymethod.saturating_sub_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_sub_assign_uint)
    fn saturating_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_sub_uint<U>(&self, rhs: U) -> Self
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
    /// [safe_sub()](#tymethod.safe_sub)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_sub_uint)
    fn safe_sub_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_sub_assign_uint<U>(&mut self, rhs: U)
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
    /// [safe_sub_assign()](#tymethod.safe_sub_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_sub_assign_uint)
    fn safe_sub_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** MULTIPLICATION UINT ***/
    
    // fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
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
    /// [checked_mul()](#tymethod.checked_mul)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.checked_mul_uint)
    fn checked_mul_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
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
    /// [unchecked_mul()](#tymethod.unchecked_mul)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.unchecked_mul_uint)
    fn unchecked_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_mul_uint<U>(&self, rhs: U) -> Self
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
    /// [saturating_mul()](#tymethod.saturating_mul)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_mul_uint)
    fn saturating_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
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
    /// [saturating_mul_assign()](#tymethod.saturating_mul_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_mul_assign_uint)
    fn saturating_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_mul_uint<U>(& self, rhs: U) -> Self
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
    /// [safe_mul()](#tymethod.safe_mul)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_mul_uint)
    fn safe_mul_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn safe_mul_assign_uint<U>(&mut self, rhs: U)
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
    /// [safe_mul_assign()](#tymethod.safe_mul_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.safe_mul_assign_uint)
    fn safe_mul_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** DIVISION UINT ***/
    
    // fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
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
    /// [checked_div()](#tymethod.checked_div)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.checked_div_uint)
    fn checked_div_uint<U>(&self, rhs: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_div_uint<U>(&self, rhs: U) -> Self
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
    /// [unchecked_div()](#tymethod.unchecked_div)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.unchecked_div_uint)
    fn unchecked_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
            self.checked_div_uint(rhs).unwrap()
    }
    
    // fn saturating_div_uint<U>(&self, rhs: U) -> Self
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
    /// [saturating_div()](#tymethod.saturating_div)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_div_uint)
    fn saturating_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_div_assign_uint<U>(&mut self, rhs: U)
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
    /// [saturating_div_assign()](#tymethod.saturating_div_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_div_assign_uint)
    fn saturating_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn checked_rem_uint<U>(&self, rhs: U) -> Option<Self>
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
    /// [checked_rem()](#tymethod.checked_rem)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.checked_rem_uint)
    fn checked_rem_uint<U>(&self, rhs: U) -> Option<U>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_rem_uint<U>(&self, rhs: U) -> Self
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
    /// [unchecked_rem()](#tymethod.unchecked_rem)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.unchecked_rem_uint)
    fn unchecked_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_rem_uint<U>(&self, rhs: U) -> Self
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
    /// [saturating_rem()](#tymethod.saturating_rem)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_rem_uint)
    fn saturating_rem_uint<U>(&self, rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
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
    /// [saturating_rem_assign()](#tymethod.saturating_rem_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_more/struct.BigUInt.html#method.saturating_rem_assign_uint)
    fn saturating_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** ADDITION BIGUINT ***/
    
    // fn checked_add(&self, rhs: &Self) -> Option<Self>
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
    /// [checked_add_uint()](#tymethod.checked_add_uint)
    /// is a bit faster than this method `checked_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_add_uint()](#tymethod.checked_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.checked_add)
    fn checked_add(&self, rhs: &Self) -> Option<Self>;
    
    // fn unchecked_add(&self, rhs: &Self) -> Self
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
    /// [unchecked_add_uint()](#tymethod.unchecked_add_uint)
    /// is a bit faster than this method `unchecked_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_add_uint()](#tymethod.unchecked_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.unchecked_add)
    fn unchecked_add(&self, rhs: &Self) -> Self;
    
    // fn saturating_add(&self, rhs: &Self) -> Self
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
    /// [saturating_add_uint()](#tymethod.saturating_add_uint)
    /// is a bit faster than this method `saturating_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_uint()](#tymethod.saturating_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_add)
    fn saturating_add(&self, rhs: &Self) -> Self;
    
    // fn saturating_add_assign(&mut self, rhs: &Self)
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
    /// [saturating_add_assign_uint()](#tymethod.saturating_add_assign_uint)
    /// is a bit faster than this method `saturating_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_add_assign_uint()](#tymethod.saturating_add_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_add_assign)
    fn saturating_add_assign(&mut self, rhs: &Self);
    
    // fn safe_add(& self, rhs: &Self) -> Self
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
    /// [safe_add_uint()](#tymethod.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](#tymethod.safe_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_add)
    fn safe_add(&self, rhs: &Self) -> Self;
    
    // fn safe_add_assign(&mut self, rhs: &Self)
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
    /// [safe_add_assign_uint()](#tymethod.safe_add_assign_uint)
    /// is a bit faster than this method `safe_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_assign()](#tymethod.safe_add_assign).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_add_assign)
    fn safe_add_assign(&mut self, rhs: &Self);
    
    
    
    /*** SUBTRACTION BIGUINT ***/
    
    // fn checked_sub(&self, rhs: &Self) -> Option<Self>
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
    /// [checked_sub_uint()](#tymethod.checked_sub_uint)
    /// is a bit faster than this method `checked_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_sub_uint()](#tymethod.checked_sub_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.checked_sub)
    fn checked_sub(&self, rhs: &Self) -> Option<Self>;
    
    // fn unchecked_sub(&self, rhs: &Self) -> Self
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
    /// [unchecked_sub_uint()](#tymethod.unchecked_sub_uint)
    /// is a bit faster than this method `unchecked_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_sub_uint()](#tymethod.unchecked_sub_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.unchecked_sub)
    fn unchecked_sub(&self, rhs: &Self) -> Self;
    
    // fn saturating_sub(&self, rhs: &Self) -> Self
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
    /// [saturating_sub_uint()](#tymethod.saturating_sub_uint)
    /// is a bit faster than this method `saturating_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub_uint()](#tymethod.saturating_sub_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_sub)
    fn saturating_sub(&self, rhs: &Self) -> Self;
    
    // fn saturating_sub_assign(&mut self, rhs: &Self)
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
    /// [saturating_sub_assign_uint()](#tymethod.saturating_sub_assign_uint)
    /// is a bit faster than this method `saturating_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_sub_assign_uint()](#tymethod.saturating_sub_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_sub_assign)
    fn saturating_sub_assign(&mut self, rhs: &Self);
    
    // fn safe_sub(&self, rhs: &Self) -> Self
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
    /// [safe_sub_uint()](#tymethod.safe_sub_uint)
    /// is a bit faster than this method `safe_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_sub_uint()](#tymethod.safe_sub_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_sub)
    fn safe_sub(&self, rhs: &Self) -> Self;
    
    // fn safe_sub_assign(&mut self, rhs: &Self)
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
    /// [safe_sub_assign_uint()](#tymethod.safe_sub_assign_uint)
    /// is a bit faster than this method `safe_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_sub_assign_uint()](#tymethod.safe_sub_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_sub_assign)
    fn safe_sub_assign(&mut self, rhs: &Self);
    
    
    
    /*** MULTIPLICATION BIGUINT ***/
    
    // fn checked_mul(&self, rhs: &Self) -> Option<Self>
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
    /// [checked_mul_uint()](#tymethod.checked_mul_uint)
    /// is a bit faster than this method `checked_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_mul_uint()](#tymethod.checked_mul_uint).
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
    ///         },
    ///     None => { println!("Overflow happend!"); },
    /// }
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.checked_mul)
    fn checked_mul(&self, rhs: &Self) -> Option<Self>;
    
    // fn unchecked_mul(&self, rhs: &Self) -> Self
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
    /// [unchecked_mul_uint()](#tymethod.unchecked_mul_uint)
    /// is a bit faster than this method `unchecked_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_mul_uint()](#tymethod.unchecked_mul_uint).
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
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.unchecked_mul)
    fn unchecked_mul(&self, rhs: &Self) -> Self
    {
            self.checked_mul(rhs).unwrap()
    }
    
    // fn saturating_mul(&self, rhs: &Self) -> Self
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
    /// [saturating_mul_uint()](#tymethod.saturating_mul_uint)
    /// is a bit faster than this method `saturating_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_uint()](#tymethod.saturating_mul_uint).
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
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.carrying_add_assign_uint)
    fn saturating_mul(&self, rhs: &Self) -> Self;
    
    // fn saturating_mul_assign(&mut self, rhs: &Self)
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
    /// [saturating_mul_assign_uint()](#tymethod.saturating_mul_assign_uint)
    /// is a bit faster than this method `saturating_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_mul_assign_uint()](#tymethod.saturating_mul_assign_uint).
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
    /// 
    /// a_biguint.saturating_mul_assign(&b_biguint);
    /// println!("After a_biguint.saturating_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_mul_assign)
    fn saturating_mul_assign(&mut self, rhs: &Self);
    
    // fn safe_mul(& self, rhs: &Self) -> Self
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
    /// [safe_add_uint()](#tymethod.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](#tymethod.safe_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_mul)
    fn safe_mul(&self, rhs: &Self) -> Self;
    
    // fn safe_mul_assign(&mut self, rhs: &Self)
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
    /// [safe_add_uint()](#tymethod.safe_add_uint)
    /// is a bit faster than this method `safe_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [safe_add_uint()](#tymethod.safe_add_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.safe_mul_assign)
    fn safe_mul_assign(&mut self, rhs: &Self);
    
    
    
    /*** DIVISION BIGUINT ***/
    
    // fn checked_div(&self, rhs: &Self) -> Option<Self>
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
    /// [checked_div_uint()](#tymethod.checked_div_uint)
    /// is a bit faster than this method `checked_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_div_uint()](#tymethod.checked_div_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.checked_div)
    fn checked_div(&self, rhs: &Self) -> Option<Self>;
    
    // fn unchecked_div(&self, rhs: &Self) -> Self
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
    /// [unchecked_div_uint()](#tymethod.unchecked_div_uint)
    /// is a bit faster than this method `unchecked_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_div_uint()](#tymethod.unchecked_div_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.unchecked_div)
    fn unchecked_div(&self, rhs: &Self) -> Self;
    
    // fn saturating_div(&self, rhs: &Self) -> Self
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
    /// [saturating_div_uint()](#tymethod.saturating_div_uint)
    /// is a bit faster than this method `saturating_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_uint()](#tymethod.saturating_div_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_div)
    fn saturating_div(&self, rhs: &Self) -> Self;
    
    // fn saturating_div_assign(&mut self, rhs: &Self)
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
    /// [saturating_div_assign_uint()](#tymethod.saturating_div_assign_uint)
    /// is a bit faster than this method `saturating_div_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_div_assign_uint()](#tymethod.saturating_div_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_div_assign)
    fn saturating_div_assign(&mut self, rhs: &Self);
    
    // fn checked_rem(&self, rhs: &Self) -> Option<Self>
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
    /// [checked_rem_uint()](#tymethod.checked_rem_uint)
    /// is a bit faster than this method `checked_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [checked_rem_uint()](#tymethod.checked_rem_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.checked_rem)
    fn checked_rem(&self, rhs: &Self) -> Option<Self>;
    
    // fn unchecked_rem(&self, rhs: &Self) -> Self
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
    /// [unchecked_rem_uint()](#tymethod.unchecked_rem_uint)
    /// is a bit faster than this method `unchecked_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [unchecked_rem_uint()](#tymethod.unchecked_rem_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.unchecked_rem)
    fn unchecked_rem(&self, rhs: &Self) -> Self;
    
    // fn saturating_rem(&self, rhs: &Self) -> Self
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
    /// [saturating_rem_uint()](#tymethod.saturating_rem_uint)
    /// is a bit faster than this method `saturating_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_uint()](#tymethod.saturating_rem_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_rem)
    fn saturating_rem(&self, rhs: &Self) -> Self;
    
    // fn saturating_rem_assign(&mut self, rhs: &Self)
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
    /// [saturating_rem_assign_uint()](#tymethod.saturating_rem_assign_uint)
    /// is a bit faster than this method `saturating_rem_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [saturating_rem_assign_uint()](#tymethod.saturating_rem_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.saturating_rem_assign)
    fn saturating_rem_assign(&mut self, rhs: &Self);
    
    
    
    /*** MULTIPLE UINT ***/
    
    // fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [next_multiple_of()](#tymethod.next_multiple_of)
    ///   is proper rather than this method `next_multiple_of_uint()`.
    /// - In order to use any one of
    ///   [modular_next_multiple_of_uint()](trait.BigUInt_Modular.html#tymethod.modular_next_multiple_of_uint),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.next_multiple_of_uint)
    fn next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [next_multiple_of_assign()](#tymethod.next_multiple_of_assign)
    ///   is proper rather than this method `next_multiple_of_assign_uint()`.
    /// - In order to use any one of
    ///   [modular_next_multiple_of_assign_uint()](trait.BigUInt_Modular.html#tymethod.modular_next_multiple_of_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.next_multiple_of_assign_uint)
    fn next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn is_multiple_of_uint<U>(&self, rhs: U) -> bool
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
    /// [is_multiple_of()](#tymethod.is_multiple_of)
    /// is proper rather than this method `is_multiple_of_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = 100_u8;
    /// let ans = a_biguint.is_multiple_of_uint(rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.is_multiple_of_uint)
    fn is_multiple_of_uint<U>(&self, rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** MULTIPLE BIGUINT ***/
    
    // fn next_multiple_of(&self, rhs: &Self) -> Self
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
    /// - The method
    ///   [next_multiple_of_uint()](#tymethod.next_multiple_of_uint)
    ///   is a bit faster than this method `next_multiple_of()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [next_multiple_of_uint()](#tymethod.next_multiple_of_uint).
    /// - In order to use any one of
    ///   [modular_next_multiple_of()](trait.BigUInt_Modular.html#tymethod.modular_next_multiple_of),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.next_multiple_of)
    fn next_multiple_of(&self, rhs: &Self) -> Self;
    
    // fn next_multiple_of_assign(&mut self, rhs: &Self)
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
    /// - The method
    ///   [next_multiple_of_assign_uint()](#tymethod.next_multiple_of_assign_uint)
    ///   is a bit faster than this method `next_multiple_of_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [next_multiple_of_assign_uint()](#tymethod.next_multiple_of_assign_uint).
    /// - In order to use any one of
    ///   [modular_next_multiple_of_assign()](trait.BigUInt_Modular.html#tymethod.modular_next_multiple_of_assign),
    ///   you need to import (use) the trait `BigUInt_Modular`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.next_multiple_of_assign)
    fn next_multiple_of_assign(&mut self, rhs: &Self);
    
    // fn is_multiple_of(&self, rhs: &Self) -> bool
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
    /// [is_next_multiple_of_uint()](#tymethod.is_next_multiple_of_uint)
    /// is a bit faster than this method `is_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [is_next_multiple_of_uint()](#tymethod.is_next_multiple_of_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.is_multiple_of)
    fn is_multiple_of(&self, rhs: &Self) -> bool;
    
    
    
    /*** MIDDLE POINT ****/
    
    // fn midpoint_uint<U>(&self, rhs: U) -> Self
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
    /// [midpoint()](#tymethod.midpoint)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.midpoint_uint)
    fn midpoint_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn midpoint_assign_uint<U>(&mut self, rhs: U)
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
    /// [midpoint_assign()](#tymethod.midpoint_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.midpoint_assign_uint)
    fn midpoint_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn midpoint(&self, rhs: &Self) -> Self
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
    /// The method [midpoint_uint()](#tymethod.midpoint_uint)
    /// is more efficient than this method `midpoint()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_uint()](#tymethod.midpoint_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.midpoint)
    fn midpoint(&self, rhs: &Self) -> Self;
    
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
    /// The method [midpoint_assign_uint()](#tymethod.midpoint_assign_uint)
    /// is more efficient than this method `midpoint_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_assign_uint()](#tymethod.midpoint_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.midpoint_assign)
    fn midpoint_assign(&mut self, rhs: &Self);
    
    
    
    /*** EXPONENTIATION AND LOGARITHM UINT ***/
    
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
    /// [checked_pow()](#tymethod.checked_pow)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_pow_uint)
    fn checked_pow_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_pow_uint<U>(&self, exp: U) -> Self
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
    /// [unchecked_pow()](#tymethod.unchecked_pow)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.unchecked_pow_uint)
    fn unchecked_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_pow_uint<U>(&self, exp: U) -> Self
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
    /// [saturating_pow()](#tymethod.saturating_pow)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.saturating_pow_uint)
    fn saturating_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn saturating_pow_assign_uint<U>(&self, exp: U)
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
    /// [saturating_pow_assign()](#tymethod.saturating_pow_assign)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.saturating_pow_assign_uint)
    fn saturating_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn checked_iroot_uint<U>(&self, exp: U) -> Option<Self>
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
    /// [checked_iroot()](#tymethod.checked_iroot)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.checked_iroot_uint)
    fn checked_iroot_uint<U>(&self, exp: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_iroot_uint<U>(&self, exp: U) -> Self
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
    /// [unchecked_root()](#tymethod.unchecked_root)
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.unchecked_iroot_uint)
    fn unchecked_iroot_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
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
    /// [checked_ilog2()](#tymethod.checked_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [checked_ilog10()](#tymethod.checked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigInt_More;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.checked_ilog_uint)
    fn checked_ilog_uint<U>(&self, base: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_ilog_uint<U>(&self, base: U) -> Self
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation_uint/struct.BigUInt.html#method.unchecked_ilog_uint)
    fn unchecked_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    
    
    /*** EXPONENTIATION AND LOGARITHM BIGUINT ***/
    
    // fn checked_pow(&self, exp: &Self) -> Option<Self>
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
    /// [checked_pow_uint()](#tymethod.checked_pow_uint) is a bit
    /// faster than this method `checked_pow()` when the exponent `exp` is
    /// primitive unsigned integral data type such as u8, u16, u32, u64, and
    /// u128. If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [checked_pow_uint()](#tymethod.checked_pow_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_pow)
    fn checked_pow(&self, exp: &Self) -> Option<Self>;
    
    // fn unchecked_pow(&self, exp: &Self) -> Self
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
    /// The method [pow_uint()](#tymethod.pow_uint) is more
    /// efficient than this method `pow()` when the exponent `exp` is primitive
    /// unsigned integral data type such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](#tymethod.pow_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.unchecked_pow)
    fn unchecked_pow(&self, exp: &Self) -> Self;
    
    // fn saturating_pow(&self, exp: &Self) -> Self
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
    /// The method [saturating_pow_uint()](#tymethod.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](#tymethod.pow_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.saturating_pow)
    fn saturating_pow(&self, exp: &Self) -> Self;
    
    // fn saturating_pow_assign(&mut self, exp: &Self)
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
    /// The method [saturating_pow_assign_uint()](#tymethod.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow_assign()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [saturating_pow_assign_uint()](#tymethod.saturating_pow_assign_uint).
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.saturating_pow_assign)
    fn saturating_pow_assign(&mut self, exp: &Self);
    
    // fn checked_iroot(&self, exp: &Self) -> Option<Self>
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
    /// [checked_iroot_uint()](#tymethod.checked_iroot_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_iroot)
    fn checked_iroot(&self, exp: &Self) -> Option<Self>;
    
    // fn unchecked_iroot(&self, exp: &Self) -> Self
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
    /// [unchecked_iroot_uint()](#tymethod.unchecked_iroot_uint)
    /// is a bit faster than this method `unchecked_iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [unchecked_iroot_uint()](#tymethod.unchecked_iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.unchecked_iroot)
    fn unchecked_iroot(&self, exp: &Self) -> Self;
    
    // fn checked_ilog(&self, base: &Self) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_ilog)
    fn checked_ilog(&self, base: &Self) -> Option<Self>;
    
    // fn unchecked_ilog(&self, base: &Self) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    fn unchecked_ilog(&self, base: &Self) -> Self;
    
    // fn checked_ilog2(&self) -> Option<Self>
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.checked_ilog2)
    fn checked_ilog2(&self) -> Option<Self>;
    
    // fn unchecked_ilog2(&self) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.unchecked_ilog2)
    fn unchecked_ilog2(&self) -> Self;
    
    // fn checked_ilog10(&self) -> Option<Self>
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    fn checked_ilog10(&self) -> Option<Self>;
    
    // fn unchecked_ilog10(&self) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    fn unchecked_ilog10(&self) -> Self;
    
    // fn checked_shift_left<U>(&self, n: U) -> Option<Self>
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.checked_shift_left)
    fn checked_shift_left<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_shift_left<U>(&self, n: U) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.unchecked_shift_left)
    fn unchecked_shift_left<U>(&self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn checked_shift_right<U>(&self, n: U) -> Option<Self>
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.checked_shift_right)
    fn checked_shift_right<U>(&self, n: U) -> Option<Self>
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn unchecked_shift_right<U>(&self, n: U) -> Self
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_basic_operation/struct.BigUInt.html#method.unchecked_shift_right)
    fn unchecked_shift_right<U>(&self, n: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

}