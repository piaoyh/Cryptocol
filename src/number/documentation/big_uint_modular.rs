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
/// examples were moved to big_uint_arithmetic_uint.rs.
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
    /*** ADDITION ***/

    // pub fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulo`.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_uint()` and the
    ///   method `wrapping_add_uint()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_uint()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_add()](struct@BigUInt#method.modular_add)
    /// is proper rather than this method `modular_add_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let rhs = 1_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for mormal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 2_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max().wrapping_sub_uint(2_u8);
    /// let m = U256::max();
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}", a_biguint, rhs, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "3");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "3");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_add_uint(rhs, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::zero();
    /// let _rhs = 3_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_add_uint(_rhs, &_m);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::one();
    /// let _rhs = 3_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_add_uint(_rhs, &_m);
    /// ```
    pub fn modular_add_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>

    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulo`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_assign_uint()` and
    ///   the method `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation, the
    ///   `OVERFLOW` flag is not changed even if this current operation does
    ///    not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_add_assign()](struct@BigUInt#method.modular_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 1_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 2_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.modular_add_assign_uint(1_u8, &m);
    /// println!("After a_biguint.modular_add_assign_uint(1_u8, &m), a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "750");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "750");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "150");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::zero();
    /// let _rhs = 1_u8;
    /// // It will panic.
    /// _a_biguint.modular_add_assign_uint(_rhs, &_m);
    /// 
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::one();
    /// let _rhs = 1_u8;
    /// // It will panic.
    /// _a_biguint.modular_add_assign_uint(_rhs, &_m);
    /// ```
    pub fn modular_add_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulo`.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add()` and the method
    ///   `wrapping_add()` are, first, where wrapping around happens,
    ///   and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_add_uint()](struct@BigUInt#method.modular_add_uint)
    /// is a bit faster than this method `modular_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_add_uint()](struct@BigUInt#method.modular_add_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let one = U256::one();
    /// let res = a_biguint.modular_add(&one, &m);
    /// println!("{} + {} = {}", a_biguint, one, res);
    /// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let two = U256::from_uint(2_u8);
    /// let res = a_biguint.modular_add(&two, &m);
    /// println!("{} + {} = {}", a_biguint, two, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let three = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_add(&three, &m);
    /// println!("{} + {} = {}", a_biguint, three, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max().wrapping_sub_uint(2_u8);
    /// let m = U256::max();
    /// let three = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_add(&three, &m);
    /// println!("{} + {} = {}", a_biguint, three, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let three = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_add(&three, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, three, res, m);
    /// assert_eq!(res.to_string(), "3");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let three = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_add(&three, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, three, res, m);
    /// assert_eq!(res.to_string(), "3");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// let res = a_biguint.modular_add(&zero, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let multiple_of_modulo = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// let res = a_biguint.modular_add(&zero, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// let res = a_biguint.modular_add(&zero, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let multiple_of_modulo = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let multiple_of_modulo = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    /// println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    /// assert_eq!(res.to_string(), "0");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::zero();
    /// let _rhs = U256::from_uint(3_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_add(&_rhs, &_m);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::one();
    /// let _rhs = U256::from_uint(3_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_add(&_rhs, &_m);
    /// ```
    pub fn modular_add(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Calculates (`self` + `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` typed.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulo`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `modular_add_assign_uint()` and
    ///   the method `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method set `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_assign()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_add_assign_uint()](struct@BigUInt#method.modular_add_assign_uint)
    /// is a bit faster than this method `modular_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_add_assign_uint()](struct@BigUInt#method.modular_add_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    /// let one = U256::one();
    /// a_biguint.modular_add_assign(&one, &m);
    /// println!("After a_biguint.modular_add_assign_uint(&U256::one(), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// println!("Originally, b_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    /// let two = U256::from_uint(2_u8);
    /// a_biguint.modular_add_assign(&two, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(2_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();    
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    /// let three = U256::from_uint(3_u8);
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "4");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::max().wrapping_sub_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::max();
    /// let three = U256::from_uint(3_u8);
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let three = U256::from_uint(3_u8);
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let three = U256::from_uint(3_u8);
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let three = U256::from_uint(3_u8);
    /// a_biguint.modular_add_assign(&three, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// a_biguint.modular_add_assign(&zero, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let multiple_of_modulo = U256::from_uint(250_u8);
    /// a_biguint.modular_add_assign(&multiple_of_modulo, &m);
    /// println!("After a_biguint.modular_add_assign(& U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// a_biguint.modular_add_assign(&zero, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let zero = U256::zero();
    /// a_biguint.modular_add_assign(&zero, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(150_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let multiple_of_modulo = U256::from_uint(250_u8);
    /// a_biguint.modular_add_assign(&multiple_of_modulo, &m);
    /// println!("After a_biguint.modular_add_assign(&U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::zero();
    /// let _rhs = U256::one();
    /// // It will panic.
    /// _a_biguint.modular_add_assign(&_rhs, &_m);
    /// 
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::one();
    /// let _rhs = U256::one();
    /// // It will panic.
    /// _a_biguint.modular_add_assign(&_rhs, &_m);
    /// ```
    pub fn modular_add_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** Subtraction ***/

    // pub fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-difference (`self` - `rhs`) % `modulo` with
    /// wrapping (modular) subtraction at `modulo`.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulo`.
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub_uint()` and the
    ///   method `wrapping_sub_uint()` are, first, where wrapping around
    ///   happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub_uint()` sets `UNDERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_sub()](struct@BigUInt#method.modular_sub)
    /// is proper rather than this method `modular_sub_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 1_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 2_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::max();
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res, U256::max().wrapping_sub_uint(1_u8));
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "247");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "247");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for  op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Pacnic Example
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::zero();
    /// let _rhs = 3_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_sub_uint(_rhs, &_m);
    /// 
    /// let _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::one();
    /// let _rhs = 3_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_sub_uint(_rhs, &_m);
    /// ```
    pub fn modular_sub_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulo`.
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub_assign_uint()` and
    ///   the method `wrapping_sub_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub_assign_uint()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   underflow occurred even once before this current operation or
    ///   `UNDERFLOW` flag is already set before this current operation, the
    ///   `UNDERFLOW` flag is not changed even if this current operation does
    ///    not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_sub_assign()](struct@BigUInt#method.modular_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 1_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    ///    
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 2_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    ///    
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 3_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084090");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::max();
    /// let rhs = 3_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint, U256::max().wrapping_sub_uint(1_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "247");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "247");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(150_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// a_biguint.modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::zero();
    /// let _rhs = 1_u8;
    /// // It will panic.
    /// _a_biguint.modular_sub_assign_uint(_rhs, &_m);
    /// 
    /// let mut _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::one();
    /// let _rhs = 1_u8;
    /// // It will panic.
    /// _a_biguint.modular_sub_assign_uint(_rhs, &_m);
    /// ```
    pub fn modular_sub_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type..
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-difference (`self` - `rhs`) % `modulo` with
    /// wrapping (modular) subtraction at `modulo`.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulo`.
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub()` and the
    ///   method `wrapping_sub()` are, first, where wrapping around
    ///   happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub()` sets `UNDERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_sub_uint()](struct@BigUInt#method.modular_sub_uint)
    /// is a bit faster than this method `modular_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_sub_uint()](struct@BigUInt#method.modular_sub_uint).
    /// 
    /// # Example 1 for Normal Case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// let one = U256::one();
    /// let res = a_biguint.modular_sub(&one, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, one, res, m);
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
    /// # Example 2 for Normal Case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// let one = U256::one();
    /// let res = a_biguint.modular_sub(&one, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, one, res, m);
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
    /// # Example 3 for Normal Case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// let three = U256::from_uint(4_u8);
    /// let res = a_biguint.modular_sub(&three, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, three, res, m);
    /// assert_eq!(res.to_string(), "9999999999999999999999999999999999999999999999999999999999999999998");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::max();
    /// let rhs = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res, U256::max().wrapping_sub_uint(1_u8));
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "247");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::from_uint(3_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "247");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let rhs = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "6");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = U256::from_uint(250_u8);
    /// let res = a_biguint.modular_sub(&rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Pacnic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::zero();
    /// let _rhs = U256::from_uint(3_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_sub(&_rhs, &_m);
    /// 
    /// let _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::one();
    /// let _rhs = U256::from_uint(3_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_sub(&_rhs, &_m);
    /// ```
    pub fn modular_sub(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulo`.
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method `modular_sub_assign()` and
    ///   the method `wrapping_sub_assign()` are, first, where wrapping
    ///   around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub_assign()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   underflow occurred even once before this current operation or
    ///   `UNDERFLOW` flag is already set before this current operation, the
    ///   `UNDERFLOW` flag is not changed even if this current operation does
    ///    not cause underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_sub_assign_uint()](struct@BigUInt#method.modular_sub_assign_uint)
    /// is a bit faster than this method `modular_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_sub_assign_uint()](struct@BigUInt#method.modular_sub_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = UU32::one();
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 2 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    ///    
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = UU32::from_uint(2_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 3 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    ///    
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = UU32::from_uint(3_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084090");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for modulo == Self::max()
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::max();
    /// let rhs = U256::from_uint(3_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint, U256::max().wrapping_sub_uint(1_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::from_uint(3_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "247");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::from_uint(3_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "247");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let rhs = U256::from_uint(250_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(250_u8);
    /// let rhs = U256::zero();
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(150_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = U256::from_uint(50_u8);
    /// let rhs = U256::from_uint(250_u8);
    /// a_biguint.modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::zero();
    /// let _rhs = U256::one();
    /// // It will panic.
    /// _a_biguint.modular_sub_assign(&_rhs, &_m);
    /// 
    /// let mut _a_biguint = U256::from_uint(2_u8);
    /// let _m = U256::one();
    /// let _rhs = U256::one();
    /// // It will panic.
    /// _a_biguint.modular_sub_assign(&_rhs, &_m);
    /// ```
    pub fn modular_sub_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** Multiplication ***/

    // pub fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` * `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-product (`self` * `rhs`) % `modulo` with wrapping
    /// (modular) multiplication at `modulo`.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulo`.
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences of between this method `modular_mul_uint()` and the
    ///   method `wrapping_mul_uint()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_mul_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_mul_uint()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_mul()](struct@BigUInt#method.modular_mul)
    /// is proper rather than this method `modular_mul_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {}", a_biguint, mul_uint, res);
    /// assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_uint = 248_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {}", a_biguint, mul_uint, res);
    /// assert_eq!(res.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let _m = UU32::zero();
    /// let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _mul_uint = 248_u8;
    /// // It will panic!
    /// let res = _a_biguint.modular_mul_uint(_mul_uint, &_m);
    /// 
    /// let _m = UU32::one();
    /// let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _mul_uint = 248_u8;
    /// // It will panic!
    /// let res = _a_biguint.modular_mul_uint(_mul_uint, &_m);
    /// ```
    pub fn modular_mul_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` * `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `product` divided
    ///   by `modulo` to `self` back.
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences between this method `modular_mul_assign_uint()` and
    ///   the method `wrapping_mul_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_mul_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_mul_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation, the
    ///   `OVERFLOW` flag is not changed even if this current operation does
    ///    not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_mul_assign()](struct@BigUInt#method.modular_mul_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mul_uint = 5_u8;
    /// a_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mul_uint = 248_u8;
    /// a_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mul_uint = 248_u16;
    /// a_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mul_uint = 2_u16;
    /// a_biguint.modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "35149539482914268500351723679771158582906673069252814597151206317181518258");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "750");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "750");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "150");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _m = UU32::zero();
    /// let _mul_uint = 248_u8;
    /// // It will panic!
    /// _a_biguint.modular_mul_assign_uint(_mul_uint, &_m);
    /// 
    /// let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _m = UU32::one();
    /// let _mul_uint = 248_u8;
    /// // It will panic!
    /// _a_biguint.modular_mul_assign_uint(_mul_uint, &_m);
    /// ```
    pub fn modular_mul_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates (`self` * `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Output
    /// It returns the modulo-product (`self` * `rhs`) % `modulo` with wrapping
    /// (modular) multiplication at `modulo`.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulo`.
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences between this method `modular_mul()` and the method
    ///   `wrapping_mul()` are, first, where wrapping around happens,
    ///   and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_mul()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_mul()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_mul_uint()](struct@BigUInt#method.modular_mul_uint)
    /// is a bit faster than this method `modular_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_mul_uint()](struct@BigUInt#method.modular_mul_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_biguint = UU32::from_uint(123456789_u32);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "8622247606403727534023749230384750061554739874487486410968923457264899031");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for modulo == maximum
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::max();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_biguint = UU32::from_uint(123456789_u32);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "111970462099597246185125739952117562742423650866418469977837510261574559319010");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_biguint = UU32::zero();
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_biguint = UU32::zero();
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_biguint = UU32::zero();
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// let res = a_biguint.modular_mul(&mul_biguint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    /// assert_eq!(res.to_string(), "0");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let _m = UU32::zero();
    /// let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _mul_biguint = UU32::from_uint(248_u8);
    /// // It will panic!
    /// let res = _a_biguint.modular_mul(&_mul_biguint, &_m);
    /// 
    /// let _m = UU32::one();
    /// let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let _mul_biguint = UU32::from_uint(248_u8);
    /// // It will panic!
    /// let res = _a_biguint.modular_mul(&_mul_biguint, &_m);
    /// ```
    pub fn modular_mul(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_mul_assign(&self, rhs: &Self, modulo: &Self)
    /// Calculates (`self` * `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either `zero` or `one`, this method will panic.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `product` divided
    ///   by `modulo` to `self` back.
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences between this method `modular_mul_assign()` and
    ///   the method `wrapping_mul_assign()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_mul_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_mul_assign()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_mul_assign_uint()](struct@BigUInt#method.modular_mul_assign_uint)
    /// is a bit faster than this method `modular_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_mul_assign_uint()](struct@BigUInt#method.modular_mul_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After a_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_biguint = UU32::from_uint(123456789_u32);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8622247606403727534023749230384750061554739874487486410968923457264899031");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for modulo == maximum
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::max();
    /// let mul_biguint = UU32::from_uint(123456789_u32);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "111970462099597246185125739952117562742423650866418469977837510261574559319010");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(4321000_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::zero();
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::zero();
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(4321000_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::zero();
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(4321000_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let mul_biguint = UU32::from_uint(4321000_u32);
    /// a_biguint.modular_mul_assign(&mul_biguint, &m);
    /// println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// assert_eq!(_a_biguint.is_overflow(), false);
    /// assert_eq!(_a_biguint.is_underflow(), false);
    /// assert_eq!(_a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(_a_biguint.is_infinity(), false);
    /// assert_eq!(_a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let _m = UU32::zero();
    /// let _mul_biguint = UU32::from_uint(248_u8);
    /// // It will panic!
    /// _a_biguint.modular_mul_assign(&_mul_biguint, &_m);
    /// 
    /// let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// assert_eq!(_a_biguint.is_overflow(), false);
    /// assert_eq!(_a_biguint.is_underflow(), false);
    /// assert_eq!(_a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(_a_biguint.is_infinity(), false);
    /// assert_eq!(_a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let _m = UU32::one();
    /// let _mul_biguint = UU32::from_uint(248_u8);
    /// // It will panic!
    /// _a_biguint.modular_mul_assign(&_mul_biguint, &_m);
    /// ```
    pub fn modular_mul_assign(&mut self, _rhs: &Self, _modulo: &Self)
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
    /// use cryptocol::number::BigUInt_Modular;
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
    /// use cryptocol::number::BigUInt_Modular;
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
    /// use cryptocol::number::BigUInt_Modular;
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
    /// use cryptocol::number::BigUInt_Modular;
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


    /*** Division ***/

    // pub fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and returns the quotient.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Output
    /// It returns the quotient of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    /// and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    /// and then finally returns the quotient of `rd1` divided by `rd2`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_div()](struct@BigUInt#method.modular_div)
    /// is proper rather than this method `modular_div_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "3");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = 3_u8;
    /// let res = dividend.modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_uint(750_u16);
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = 3_u8;
    /// let res = dividend.modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    /// assert_eq!(res.to_string(), "0");
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
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// // op2 == 0
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // op2 == multiple of modulo
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == 0
    /// let _a_biguint = U256::zero();
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == 0
    /// let _a_biguint = U256::from_uint(750_u16);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == multiple of modulo
    /// let _a_biguint = U256::zero();
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == multiple of modulo
    /// let _a_biguint = U256::from_uint(150_u8);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// let res = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // modulo == 0
    /// let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _rhs = 128_u8;
    /// let _m = U256::zero();
    /// // It will panic!
    /// let quotient = _a_biguint.modular_div_uint(_rhs, &_m);
    /// 
    /// // modulo == 1
    /// let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _rhs = 128_u8;
    /// let _m = U256::one();
    /// // It will panic!
    /// let quotient = _a_biguint.modular_div_uint(_rhs, &_m);
    /// ```
    pub fn modular_div_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and assigns the quotient back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the quotient of `rd1` divided by `rd2`.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_div_assign()](struct@BigUInt#method.modular_div_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let divisor = 3_u8;
    /// let modulo = U256::from_uint(250_u8);
    /// a_biguint.modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = 3_u8;
    /// a_biguint.modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// // op2 == 0
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // op2 == multiple of modulo
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == 0
    /// let mut _a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = 0_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == multiple of modulo
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == multiple of modulo
    /// let mut _a_biguint = U256::from_uint(150_u8);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = 250_u8;
    /// // It will panic.
    /// _a_biguint.modular_div_assign_uint(_rhs, &_m);
    /// 
    /// // modulo == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = 128_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_div_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = 128_u8;
    /// let _modulo = U256::one();
    /// // It will panic!
    /// _a_biguint.modular_div_assign_uint(_divisor, &_modulo);
    /// ```
    pub fn modular_div_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_div(&self, _rhs: &Self, _modulo: &Self) -> Self
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and returns the quotient.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Output
    /// It returns the quotient of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    /// and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    /// and then finally returns the quotient of `rd1` divided by `rd2`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_div_uint()](struct@BigUInt#method.modular_div_uint)
    /// is a bit faster than this method `modular_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_div_uint()](struct@BigUInt#method.modular_div_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.modular_div(&divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    /// assert_eq!(quotient.to_string(), "3");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = U256::from_uint(3_u8);
    /// let res = dividend.modular_div(&divisor, &modulo);
    /// println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_uint(750_u16);
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = U256::from_uint(3_u8);
    /// let res = dividend.modular_div(&divisor, &modulo);
    /// println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    /// assert_eq!(res.to_string(), "0");
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
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == 0
    /// let _a_biguint = U256::zero();
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == 0
    /// let _a_biguint = U256::from_uint(750_u16);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == multiple of modulo
    /// let _a_biguint = U256::zero();
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == multiple of modulo
    /// let _a_biguint = U256::from_uint(150_u8);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_div(&_rhs, &_m);
    /// 
    /// // modulo == 0
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.modular_div(&_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::one();
    /// // It will panic!
    /// let quotient = _dividend.modular_div(&_divisor, &_modulo);
    /// ```
    pub fn modular_div(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_div_assign(&self, _rhs: &Self, _modulo: &Self)
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and assigns the quotient back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the quotient of `rd1` divided by `rd2`.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_div_assign_uint()](struct@BigUInt#method.modular_div_assign_uint)
    /// is a bit faster than this method `modular_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_div_assign_uint()](struct@BigUInt#method.modular_div_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = UU32::from_uint(128_u8);
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_div_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let divisor = UU32::from_uint(3_u8);
    /// let modulo = U256::from_uint(250_u8);
    /// a_biguint.modular_div_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let modulo = U256::from_uint(250_u8);
    /// let divisor = U256::from_uint(3_u8);
    /// a_biguint.modular_div_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// // op2 == 0
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // op2 == multiple of modulo
    /// let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == 0
    /// let mut _a_biguint = U256::from_uint(750_u16);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(250_u8);
    /// let _rhs = U256::zero();
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // op1 == 0 and op2 == multiple of modulo
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // op1 == multiple of modulo and op2 == multiple of modulo
    /// let mut _a_biguint = U256::from_uint(150_u8);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _m = U256::from_uint(50_u8);
    /// let _rhs = U256::from_uint(250_u8);
    /// // It will panic.
    /// _a_biguint.modular_div_assign(&_rhs, &_m);
    /// 
    /// // modulo == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_div_assign(&_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::one();
    /// // It will panic!
    /// _a_biguint.modular_div_assign(&_divisor, &_modulo);
    /// ```
    pub fn modular_div_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }


    // pub fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and returns the remainder.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Output
    /// It returns the remainder of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    /// and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    /// and then finally returns the remainder of `rd1` divided by `rd2`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_rem()](struct@BigUInt#method.modular_rem)
    /// is proper rather than this method `modular_rem_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2 for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Example 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_uint(200_u8);
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == 0    
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == 0
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    /// let _dividend = U256::zero();
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    /// let _dividend = U256::from_uint(200_u8);
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    /// let _dividend = U256::from_uint(200_u8);
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo == 0
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 128_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 128_u8;
    /// let _modulo = U256::one();
    /// // It will panic!
    /// let quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
    /// ```
    pub fn modular_rem_uint<U>(&self, _rhs: U, _modulo: &Self) -> U
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and assigns the remainder back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the remainder of `rd1` divided by `rd2`.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [modular_rem_assign()](struct@BigUInt#method.modular_rem_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 2 for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(200_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == 0
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    /// let mut _a_biguint = U256::from_uint(200_u8);
    /// let _divisor = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    /// let mut _a_biguint = U256::from_uint(200_u8);
    /// let _divisor = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 128_u8;
    /// let _modulo = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 128_u8;
    /// let _modulo = U256::one();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    /// ```
    pub fn modular_rem_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_rem(&self, _rhs: &Self, _modulo: &Self) -> Self
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and returns the remainder.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either `zero` or multiple of `modulo`, it will panic.
    /// - If `modulo` is either `zero` or `one`, it will panic.
    /// 
    /// # Output
    /// It returns the remainder of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    /// and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    /// and then finally returns the remainder of `rd1` divided by `rd2`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_rem_uint()](struct@BigUInt#method.modular_rem_uint)
    /// is a bit faster than this method `modular_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_rem_uint()](struct@BigUInt#method.modular_rem_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem(&divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem(&divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = U256::from_uint(200_u8);
    /// let divisor = U256::from_uint(128_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.modular_rem(&divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == 0
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == 0
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    /// let _dividend = U256::zero();
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    /// let _dividend = U256::from_uint(200_u8);
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    /// let _dividend = U256::from_uint(200_u8);
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo == 0
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::one();
    /// // It will panic!
    /// let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    /// ```
    pub fn modular_rem(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_rem_assign(&self, _rhs: &Self, _modulo: &Self)
    /// Divides (`self` % `modulo`) by (`rhs` % `modulo`),
    /// and assigns the remainder back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `rhs` is either zero or multiple of `modulo`, it will panic.
    /// - If `modulo` is either zero or one, it will panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the remainder of `rd1` divided by `rd2`.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_rem_assign_uint()](struct@BigUInt#method.modular_rem_assign_uint)
    /// is a bit faster than this method `modular_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_rem_assign_uint()](struct@BigUInt#method.modular_rem_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = UU32::from_uint(128_u8);
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 2 for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = UU32::from_uint(128_u8);
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_uint(200_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = UU32::from_uint(128_u8);
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.modular_rem_assign(&divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    /// let mut _a_biguint = U256::from_uint(200_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    /// let mut _a_biguint = U256::from_uint(200_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo == 0
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// 
    /// // modulo == 1
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = U256::from_uint(128_u8);
    /// let _modulo = U256::one();
    /// // It will panic!
    /// _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    /// ```
    pub fn modular_rem_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** MULTIPLE OPERATIONS ***/

    // pub fn modular_next_multiple_of_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is a primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is `zero`.
    /// - This function will panic if `modulo` is either `zero` or `one`.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`. So,
    /// if overflow occurs, it returns the value wrapped around at `modulo`.
    /// 
    /// # Feature
    /// - Wrapping (modular) arround at `modulo`.
    /// - The differences between this method `modular_next_multiple_of_uint()`
    ///   and the method `next_multiple_of_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set. First, this
    ///   method wraps around at `modulo` while the method
    ///   `next_multiple_of_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method set `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `next_multiple_of_uint()` sets the
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [modular_next_multiple_of()](struct@BigUInt#method.modular_next_multiple_of)
    /// is proper rather than this method `modular_next_multiple_of_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = 100_u8;
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.modular_next_multiple_of_uint(num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "123456789012345678901234567890123456800");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let num = 100_u8;
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.modular_next_multiple_of_uint(num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "1");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = 0_u8;
    /// let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = 100_u8;
    /// let _modulo = U256::zero();
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = 100_u8;
    /// let _modulo = U256::one();
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);
    /// ```
    pub fn modular_next_multiple_of_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is a primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is `zero`.
    /// - This function will panic if `modulo` is either `zero` or `one`.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulo`.
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulo`. So, if
    ///   overflow occurs, `self` will be the value wrapped around at `modulo`.
    /// - The differences between this method
    ///   `modular_next_multiple_of_assign_uint()`
    ///   and the method `next_multiple_of_assign_uint()` are, first, where
    ///   wrapping around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps araound at `modulo` while the method
    ///   `next_multiple_of_assign_uint()` wraps araound at `maximum value + 1`.
    ///   Second, this method set `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `next_multiple_of_assign_uint()` sets the
    ///   `OVERFLOW` flag when wrapping around happens.
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
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let num = 100_u8;
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.modular_next_multiple_of_assign_uint(num, &modulo);
    /// println!("After a_biguint.modular_next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123456800");
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
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let num = 100_u8;
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.modular_next_multiple_of_assign_uint(num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _num = 0_u8;
    /// let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    /// _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _num = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _num = 100_u8;
    /// let _modulo = U256::zero();
    /// _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _num = 100_u8;
    /// let _modulo = U256::one();
    /// _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);
    /// ```
    pub fn modular_next_multiple_of_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is `zero`.
    /// - This function will panic if `modulo` is either `zero` or `one`.
    /// 
    /// # Output
    /// It returns the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`. So,
    /// if overflow occurs, it returns the value wrapped around at `modulo`.
    /// 
    /// # Feature
    /// - Wrapping (modular) arround at `modulo`.
    /// - The differences between this method `modular_next_multiple_of()` and
    ///   the method `next_multiple_of()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `next_multiple_of()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `next_multiple_of()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_next_multiple_of_uint()](struct@BigUInt#method.modular_next_multiple_of_uint)
    /// is a bit faster than this method `modular_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_next_multiple_of_uint()](struct@BigUInt#method.modular_next_multiple_of_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "123456789012345678901234567890123456800");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::max();
    /// let num = U256::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8); println!("modulo = {}", modulo);
    /// let multiple = a_biguint.modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "1");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// // rhs == 0
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::zero();
    /// let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);
    /// 
    /// // rhs == multiple of modulo
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::from(200_u8);
    /// let _modulo = U256::from(100_u8);
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);
    /// 
    /// // modulo == 0
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::from(100_u8);
    /// let _modulo = U256::zero();
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);
    /// 
    /// // modulo == 1
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::from(100_u8);
    /// let _modulo = U256::one();
    /// // It will panic.
    /// let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);
    /// ```
    pub fn modular_next_multiple_of(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulo`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `rhs` is `zero`.
    /// - This function will panic if `modulo` is either `zero` or `one`.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulo`.
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulo`. So, if
    ///   overflow occurs, `self` will be the value wrapped around at `modulo`.
    /// - The differences between this method
    ///   `modular_next_multiple_of_assign()` and method
    ///   `next_multiple_of_assign()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `next_multiple_of_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `next_multiple_of_assign()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [modular_next_multiple_of_assign_uint()](struct@BigUInt#method.modular_next_multiple_of_assign_uint)
    /// is a bit faster than this method `modular_next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [modular_next_multiple_of_assign_uint()](struct@BigUInt#method.modular_next_multiple_of_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
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
    /// let num = UU32::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.modular_next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123456800");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
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
    /// let num = UU32::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u8);
    /// 
    /// // rhs == 0
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _num = UU32::zero();
    /// let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    /// _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);
    /// 
    /// // rhs == multiple of modulo
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _num = UU32::from(200_u8);
    /// let _modulo = UU32::from(100_u8);
    /// _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);
    /// 
    /// // modulo == 0
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _num = UU32::from(100_u8);
    /// let _modulo = UU32::zero();
    /// _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);
    /// 
    /// // modulo == 1
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _num = UU32::from(100_u8);
    /// let _modulo = UU32::one();
    /// _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);
    /// ```
    pub fn modular_next_multiple_of_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`,
    /// and returns the result. The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is a primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either zero or one, this method will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulo` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulo`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, use the method
    /// [modular_pow()](struct@BigUInt#method.modular_pow) instead.
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let modulo = U256::halfmax();
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    /// # Example 2 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = 100_u8;
    /// let modulo = U256::halfmax();
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    /// assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = 0_u8;
    /// let modulo = U256::halfmax();
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
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
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = 30_u8;
    /// let modulo = U256::halfmax();
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    /// # Example 6 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(300_u16);
    /// let exp = 30_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let res = a_biguint.modular_pow_uint(exp, &modulo);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u32);
    /// 
    /// // self == 0 and exp == 0 and modulo != 0
    /// let _a_biguint = UU32::zero();
    /// let _exp = 0_u8;
    /// let _modulo = U256::halfmax();
    /// // It will panic.
    /// let res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self == 0 and exp == multiple of modulo and modulo != 0
    /// let _a_biguint = UU32::zero();
    /// let _exp = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == 0 and modulo != 0
    /// let _a_biguint = UU32::from_uint(300_u16);
    /// let _exp = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// let _a_biguint = UU32::from_uint(300_u16);
    /// let _exp = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 0
    /// let _a_biguint = U256::from_uint(10_u8);
    /// let _exp = 100_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 1
    /// let _a_biguint = U256::from_uint(10_u8);
    /// let _exp = 100_u8;
    /// let _modulo = U256::one();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// 
    /// // self == 0 and exp == 0 and modulo == 0
    /// let _a_biguint = U256::zero();
    /// let _exp = 0_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow_uint(_exp, &_modulo);
    /// ```
    pub fn modular_pow_uint<U>(&self, _exp: U, _modulo: &Self) -> Self
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`,
    /// and assign the result to `self` back.
    /// The type `U` has the trait `SmallUInt`.
    ///
    /// # Arguments
    /// - `exp` is the power to raise `self` to and is a primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either zero or one, this method will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulo` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulo`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [modular_pow_assign()](struct@BigUInt#method.modular_pow_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
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
    /// let exp = 30_u8;
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(1000000000000000000000000000000_u128);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 100_u8;
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 0_u8;
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
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
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 6 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 7 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_uint(300_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = 30_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.modular_pow_assign_uint(exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u64);
    /// 
    /// // self == 0 and exp == 0 and modulo != 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 0_u8;
    /// let _modulo = U256::halfmax();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self == 0 and exp == multiple of modulo and modulo != 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == 0 and modulo != 0
    /// let mut _a_biguint = U256::from_uint(300_u16);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 0_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// let mut _a_biguint = U256::from_uint(300_u16);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 200_u8;
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 0
    /// let mut _a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 100_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 1
    /// let mut _a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 100_u8;
    /// let _modulo = U256::one();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// 
    /// // self == 0 and exp == 0 and modulo == 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _exp = 0_u8;
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign_uint(_exp, &_modulo);
    /// ```
    pub fn modular_pow_assign_uint<U>(&mut self, _exp: U, _modulo: &Self)
    where U: TraitsBigUInt<U>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_pow(&self, exp: &Self, modulo: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either zero or one, this method will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulo` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulo`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// 
    /// # Counterpart Method
    /// The method [modular_pow_uint()](struct@BigUInt#method.modular_pow_uint)
    /// is more efficient than this method `modular_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [modular_pow_uint()](struct@BigUInt#method.modular_pow_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.modular_pow(&exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
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
    /// # Example 2 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.modular_pow(&exp, &modulo);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulo, res);
    /// assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::zero();
    /// let modulo = U256::halfmax();
    /// let res = a_biguint.modular_pow(&exp, &modulo);
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
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(200_u8);
    /// let modulo = UU32::from_uint(100_u8);
    /// let res = a_biguint.modular_pow(&exp, &modulo);
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
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.modular_pow(&exp, &modulo);
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
    /// # Example 6 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(300_u16);
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// let res = a_biguint.modular_pow(&exp, &modulo);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// // self == 0 and exp == 0 and modulo != 0
    /// let _a_biguint = UU32::zero();
    /// let _exp = UU32::zero();
    /// let _modulo = UU32::halfmax();
    /// // It will panic.
    /// let res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self == 0 and exp == multiple of modulo and modulo != 0
    /// let _a_biguint = UU32::zero();
    /// let _exp = UU32::from_uint(200_u8);
    /// let _modulo = UU32::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == 0 and modulo != 0
    /// let _a_biguint = UU32::from_uint(300_u16);
    /// let _exp = UU32::zero();
    /// let _modulo = UU32::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// let _a_biguint = UU32::from_uint(300_u16);
    /// let _exp = UU32::from_uint(200_u8);
    /// let _modulo = UU32::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 0
    /// let _a_biguint = UU32::from_uint(10_u8);
    /// let _exp = UU32::from_uint(100_u8);
    /// let _modulo = UU32::zero();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 1
    /// let _a_biguint = UU32::from_uint(10_u8);
    /// let _exp = UU32::from_uint(100_u8);
    /// let _modulo = UU32::one();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// 
    /// // self == 0 and exp == 0 and modulo == 0
    /// let _a_biguint = UU32::zero();
    /// let _exp = UU32::zero();
    /// let _modulo = UU32::zero();
    /// // It will panic!
    /// let _res = _a_biguint.modular_pow(&_exp, &_modulo);
    /// ```
    pub fn modular_pow(&self, _exp: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulo` of the `Self` type`,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is of `&Self` type.
    /// - `modulo` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If `modulo` is either zero or one, this method will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulo` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulo`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method [modular_pow_assign_uint()](struct@BigUInt#method.modular_pow_assign_uint)
    /// is more efficient than this method `modular_pow_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [modular_pow_assign_uint()](struct@BigUInt#method.modular_pow_assign_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 2 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(1000000000000000000000000000000_u128);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(100_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
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
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(200_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 6 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_uint(300_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.modular_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// use cryptocol::number::BigUInt_Modular;
    /// define_utypes_with!(u128);
    /// 
    /// // self == 0 and exp == 0 and modulo != 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::zero();
    /// let _modulo = U256::halfmax();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self == 0 and exp == multiple of modulo and modulo != 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == 0 and modulo != 0
    /// let mut _a_biguint = U256::from_uint(300_u16);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::zero();
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// let mut _a_biguint = U256::from_uint(300_u16);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::from_uint(200_u8);
    /// let _modulo = U256::from_uint(100_u8);
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 0
    /// let mut _a_biguint = U256::from_uint(10_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::from_uint(100_u8);
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self != 0 and exp != 0 and modulo == 1
    /// let mut _a_biguint = U256::from_uint(10_u8);
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::from_uint(100_u8);
    /// let _modulo = U256::one();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// 
    /// // self == 0 and exp == 0 and modulo == 0
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _exp = U256::zero();
    /// let _modulo = U256::zero();
    /// // It will panic!
    /// _a_biguint.modular_pow_assign(&_exp, &_modulo);
    /// ```
    pub fn modular_pow_assign(&mut self, _exp: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}