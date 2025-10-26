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
/// examples were moved to big_uint_other_calculation.rs.
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
    /***** METHODS FOR EXPONENTIATION AND LOGARITHM WITH BIGUINT *****/

    // pub fn next_power_of_two(&self) -> Self
    /// Returns the smallest power of two greater than or equal to `self`.
    /// 
    /// # Output
    /// It returns the smallest power of two greater than or equal to `self`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// When the return value overflows
    /// (i.e., `self > (1 << (size_of::<T>() * N - 1))`),
    /// it returns the value wrapped to `zero`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.next_power_of_two();
    /// println!("The next power of two is {}.", res);
    /// assert_eq!(res.to_string(), "170141183460469231731687303715884105728");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.next_power_of_two();
    /// println!("The next power of two is {}.", res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for Minimum
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.next_power_of_two();
    /// println!("The next power of two is {}.", res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn next_power_of_two(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn next_power_of_two_assign(&mut self)
    /// Finds the smallest power of two greater than or equal to `self`,
    /// and assigns the result to `self` back.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - When the result overflows
    ///   (i.e., `self > (1 << (size_of::<T>() * N - 1))`),
    ///   it `self` will be the value wrapped to `zero`.
    /// - It assigns to `self` the smallest power of two greater than
    ///   or equal to `self`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.next_power_of_two_assign();
    /// println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "170141183460469231731687303715884105728");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.next_power_of_two_assign();
    /// println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for Minimum
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
    /// a_biguint.next_power_of_two_assign();
    /// println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn next_power_of_two_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }
    
    // pub fn is_power_of_two(&self) -> bool
    /// Returns true if and only if self == 2 ** k for some k.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.is_power_of_two();
    /// println!("Is {} the power of two? - {}.", a_biguint, res);
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("170141183460469231731687303715884105728").unwrap();
    /// let res = a_biguint.is_power_of_two();
    /// println!("Is {} the power of two? - {}.", a_biguint, res);
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3 for Maximum
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::max();
    /// let res = a_biguint.is_power_of_two();
    /// println!("Is {} the power of two? - {}.", a_biguint, res);
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 4 for Minimum
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.is_power_of_two();
    /// println!("Is {} the power of two? - {}.", a_biguint, res);
    /// assert_eq!(res, true);
    /// ```
    pub fn is_power_of_two(&self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn pow(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result.
    /// 
    /// # Arguments.
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow() internally.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.pow(&exp);
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.is_overflow(), true);
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.pow(&exp);
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.pow(&exp);
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
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.pow(&_exp);
    /// ```
    pub fn pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn pow_assign(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and assign the result to `self` back.
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
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow_assign() internally.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method [pow_assign_uint()](struct@BigUInt#method.pow_assign_uint)
    /// is more efficient than this method `pow_assign()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number, use
    /// the method [pow_assign_uint()](struct@BigUInt#method.pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// a_biguint.pow_assign(&exp);
    /// println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// a_biguint.pow_assign(&exp);
    /// println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// a_biguint.pow_assign(&exp);
    /// println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// a_biguint.pow_assign(&exp);
    /// println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// _a_biguint.pow_assign(&_exp);
    /// ```
    pub fn pow_assign(&mut self, _exp: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_pow(&mut self, exp: &Self) -> Self
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
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
    /// 
    /// # Counterpart Method
    /// The method [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint)
    /// is more efficient than this method `wrapping_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [wrapping_pow_uint()](struct@BigUInt#method.wrapping_pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.wrapping_pow(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.wrapping_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.is_overflow(), true);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.wrapping_pow(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.wrapping_pow(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.wrapping_pow(&_exp);
    /// ```
    pub fn wrapping_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and assign the result to `self` back.
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
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - It calls wrapping_pow() internally.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint)
    /// is more efficient than this method `wrapping_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number, use
    /// the method [wrapping_pow_assign_uint()](struct@BigUInt#method.wrapping_pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_pow_assign(&exp);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_pow_assign(&exp);
    /// println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_pow_assign(&exp);
    /// println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// a_biguint.wrapping_pow_assign(&exp);
    /// println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// _a_biguint.wrapping_pow_assign(&_exp);
    /// ```
    pub fn wrapping_pow_assign(&mut self, _exp: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_pow(&self, exp: &Self) -> (Self, bool)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring, 
    /// wrapping around at the boundary of the
    /// type `Self`, and returns a tuple of the result along with
    /// a boolean indicating whether an overflow would occur.
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
    /// It returns a tuple of the result of raising `self` to the power of `exp`,
    /// using exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self` along with a boolean
    /// indicating whether an arithmetic overflow would occur.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
    /// - If overflowing did not happen in the current operation, the second
    ///   element of the output tuple will be false even if the `OVERFLOW` flag
    ///   of `self` was already set because of previous operation of `self`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint)
    /// is a bit faster than this method `overflowing_pow()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [overflowing_pow_uint()](struct@BigUInt#method.overflowing_pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let (res, overflow) = a_biguint.overflowing_pow(&exp);
    /// println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let (res, overflow) = a_biguint.overflowing_pow(&exp);
    /// println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(res.is_overflow(), true);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let (res, overflow) = a_biguint.overflowing_pow(&exp);
    /// println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let (res, overflow) = a_biguint.overflowing_pow(&exp);
    /// println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let (res, overflow) = _a_biguint.overflowing_pow(&_exp);
    /// ```
    pub fn overflowing_pow(&self, _exp: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_pow_assign(&mut self, exp: &Self) -> bool
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring, 
    /// wrapping around at the boundary of the type `Self`, and
    /// assigns the result to `self` back, and
    /// returns a boolean indicating whether an overflow would occur.
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
    /// It returns bool indicating whether an overflow happened.
    /// It returns `true` if overflow happened. Otherwise, it returns `false`.
    /// 
    /// # Argument
    /// The argument `exp` is of `&Self` type.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - If overflowing did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint)
    /// is a bit faster than this method `overflow_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [overflow_pow_assign_uint()](struct@BigUInt#method.overflow_pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_pow_assign(&exp);
    /// println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_pow_assign(&exp);
    /// println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_pow_assign(&exp);
    /// println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    /// assert_eq!(overflow, false);
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
    /// # Example 3 for 0 ** 123456789012345678901234567890123456789
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_pow_assign(&exp);
    /// println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    /// assert_eq!(overflow, false);
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// let overflow = _a_biguint.overflowing_pow_assign(&_exp);
    /// ```
    pub fn overflowing_pow_assign(&mut self, _exp: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn iroot(&self, exp: &Self) -> Self
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
    /// [iroot_uint()](struct@BigUInt#method.iroot_uint)
    /// is a bit faster than this method `iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [iroot_uint()](struct@BigUInt#method.iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.iroot(&exp);
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
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.iroot(&_exp);
    /// 
    /// let _a_biguint = U256::one();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.iroot(&_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.iroot(&_exp);
    /// ```
    pub fn iroot(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn iroot_assign(&mut self, exp: &Self)
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
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
    /// [iroot_assign_uint()](struct@BigUInt#method.iroot_assign_uint)
    /// is a bit faster than this method `iroot_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [iroot_assign_uint()](struct@BigUInt#method.iroot_assign_uint).
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
    /// let exp = U256::from_uint(8_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// let exp = U256::from_uint(65_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// let exp = U256::from_uint(212_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// let exp = U256::from_uint(213_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let exp = U256::from_uint(6_u8);
    /// a_biguint.iroot_assign(&exp);
    /// println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// _a_biguint.iroot_assign(&_exp);
    /// 
    /// let mut _a_biguint = U256::one();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// _a_biguint.iroot_assign(&_exp);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// _a_biguint.iroot_assign(&_exp);
    /// ```
    pub fn iroot_assign(&mut self, _exp: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn isqrt(&self) -> Self
    /// Calculates the square root of `self`, rounded down,
    /// and returns the result value.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// If the exact value of the square root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of the square root of `self` will be returned.
    /// 
    /// # Features
    /// The result of this method is never greater than `self`.
    /// So, this method never causes overflow.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let res = a_biguint.isqrt();
    /// println!("The square root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.isqrt();
    /// println!("The second root of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn isqrt(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn isqrt_assign(&mut self)
    /// Calculates the square root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - If the exact value of the square root of `self` can be expressed with
    ///   `Self`-typed unsigned integer, it will be assigned to `self`.
    ///   Otherwise, the `Self`-typed biggest unsigned integer that is less
    ///   than the exact value of the second root of `self` will be assigned
    ///   to `self`.
    /// - The result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// a_biguint.isqrt_assign();
    /// println!("After a_biguint.isqrt_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// a_biguint.isqrt_assign();
    /// println!("After a_biguint.isqrt_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn isqrt_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ilog2(&self) -> Self
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [ilog_uint()](struct@BigUInt#method.ilog_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.ilog2();
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.ilog2();
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
    /// define_utypes_with!(u16);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.ilog2();
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
    /// define_utypes_with!(u16);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog2();
    /// ```
    pub fn ilog2(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ilog2_assign(&mut self)
    /// Calculates the base 2 logarithm of the number, rounded down,
    /// and assigns back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [ilog_assign_uint()](struct@BigUInt#method.ilog_assign_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10_assign()](struct@BigUInt#method.ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(64_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog2_assign();
    /// println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(70_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog2_assign();
    /// println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
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
    /// define_utypes_with!(u64);
    ///
    /// let mut a_biguint = U256::from_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog2_assign();
    /// println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// // It will panic.
    /// _a_biguint.ilog2_assign();
    /// ```
    pub fn ilog2_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ilog10(&self) -> Self
    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 10;
    /// [ilog_uint()](struct@BigUInt#method.ilog_uint)
    /// can produce results of the base other than 10, and
    /// [ilog2()](struct@BigUInt#method.ilog2)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(12345_u32);
    /// let res = a_biguint.ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
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
    /// define_utypes_with!(u64);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
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
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog10();
    /// ```
    pub fn ilog10(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ilog10_assign(&mut self)
    /// Calculates the base 10 logarithm of the number, rounded down,
    /// and assigns back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [ilog_assign_uint()](struct@BigUInt#method.ilog_assign_uint)
    /// can produce results of the base other than 10, and
    /// [ilog2_assign()](struct@BigUInt#method.ilog2_assign)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10000_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog10_assign();
    /// println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4");
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
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(12345_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog10_assign();
    /// println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4");
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
    /// define_utypes_with!(u128);
    ///
    /// let mut a_biguint = U256::from_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.ilog10_assign();
    /// println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// // It will panic.
    /// _a_biguint.ilog10_assign();
    /// ```
    pub fn ilog10_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}