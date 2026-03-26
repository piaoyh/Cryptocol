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


use crate::number::SmallUInt;




/// # Introduction
/// Trait `BigUInt_Panic_Free` is for BigUInt
///
/// # Quick start
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::BigUInt_Panic_Free` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::BigUInt_Panic_Free;
/// ```
/// If you import (use) `cryptocol::number::BigUInt_Panic_Free`, all the methods
/// of `BigUInt_Panic_Free` are available immediately and automagically, as if
/// such primitive data types had the methods from the begining.
/// 
/// ## Example 2
/// ```
/// use cryptocol::define_utypes_with;
/// use cryptocol::number::BigUInt_Panic_Free;
/// define_utypes_with!(u32);
///
/// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
/// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
/// let one = U256::one();
/// let res = a_biguint.panic_free_modular_add(&one, &m);
/// println!("{} + {} = {}", a_biguint, one, res);
/// assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
///
/// let a_biguint = U256::from_uint(10_u8);
/// let exp = U256::from_uint(30_u8);
/// let res = a_biguint.panic_free_pow(&exp);
/// println!("{} ** {} = {}", a_biguint, exp, res);
/// assert_eq!(res.to_string(), "1000000000000000000000000000000");
///
/// let a_biguint = U256::max();
/// let num = U256::from(586478_u32);
/// let multiple = a_biguint.panic_free_next_multiple_of(&num);
/// println!("The next multiple of {} is {}", a_biguint, multiple);
/// assert_eq!(multiple.to_string(), "448670");
///
/// let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
/// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
/// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
/// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
/// assert_eq!(c_biguint.to_string(), "27");
/// ```
#[allow(non_camel_case_types)]
pub trait BigUInt_Panic_Free<T, const N: usize> : Clone + Sized //+ Display + + ToString
where T: SmallUInt
{
    /*** ADDITION ***/

    // fn panic_free_modular_add_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-sum (`self` + `rhs`) % `modulus` with wrapping
    /// (modular) addition at `modulus`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulus`.
    /// - Wrapping (modular) addition at `modulus`.
    /// - The differences between this method `panic_free_modular_add_uint()`
    ///   and the method `wrapping_add_uint()` are, first, where
    ///   wrapping around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_add_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_add_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_add()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add)
    /// is proper rather than this method `panic_free_modular_add_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 1_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_add_uint)
    fn panic_free_modular_add_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;


    // fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Calculates (`self` + `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulus`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulus`.
    /// - The differences between this method
    ///   `panic_free_modular_add_assign_uint()` and the method
    ///   `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_add_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_add_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_add_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 1_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_add_assign_uint)
    fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;


    // fn panic_free_modular_add(&self, rhs: &Self, modulus: &Self) -> Self
    /// Calculates (`self` + `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `Self`-typed.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-sum (`self` + `rhs`) % `modulus` with wrapping
    /// (modular) addition at `modulus`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulus`.
    /// - Wrapping (modular) addition at `modulus`.
    /// - The differences between this method `panic_free_modular_add()` and
    ///   the method `wrapping_add()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_add()` wraps wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus`, while the method `wrapping_add()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_add_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add_uint)
    /// is a bit faster than this method `panic_free_modular_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_add_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    /// let one = U256::one();
    /// let res = a_biguint.panic_free_modular_add(&one, &m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_add)
    fn panic_free_modular_add(&self, rhs: &Self, modulus: &Self) -> Self;


    // fn panic_free_modular_add_assign(&mut self, rhs: &Self, modulus: &Self)
    /// Calculates (`self` + `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be added to `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` + `rhs`),
    ///   and is of `Self`-typed.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulus`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulus`.
    /// - The differences between this method `panic_free_modular_add_assign()`
    ///   and the method `wrapping_add_assign()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_add_assign()` wraps around at maximum value.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus`, while the method `wrapping_add_assign()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [panic_free_modular_add_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add_assign_uint)
    /// is a bit faster than this method `panic_free_modular_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_add_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_add_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_modular_add_assign(&one, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign(&U256::one(), &m), a_biguint = {}", a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_add_assign)
    fn panic_free_modular_add_assign(&mut self, rhs: &Self, modulus: &Self);


            
    /*** SUBTRACTION ***/

    // fn panic_free_modular_sub_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Calculates (`self` - `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-difference (`self` - `rhs`) % `modulus` with
    /// wrapping (modular) subtraction at `modulus`.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulus`.
    /// - Wrapping (modular) subtraction at `modulus`.
    /// - The differences between this method `panic_free_modular_sub_uint()`
    ///   and the method `wrapping_sub_uint()` are, first, where
    ///   wrapping around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_sub_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_sub_uint()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_sub()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub)
    /// is proper rather than this method `panic_free_modular_sub_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 1_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_sub_uint)
    fn panic_free_modular_sub_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;


    // fn panic_free_modular_sub_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Calculates (`self` - `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulus`.
    /// - Wrapping (modular) subtraction at `modulus`.
    /// - The differences between this method
    ///   `panic_free_modular_sub_assign_uint()` and the method
    ///   `wrapping_sub_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_sub_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_sub_assign_uint()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   underflow occurred even once before this current operation or
    ///   `UNDERFLOW` flag is already set before this current operation,
    ///   the `UNDERFLOW` flag is not changed even if this current operation
    ///   does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_sub_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 1_u8;
    /// 
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_sub_assign_uint)
    fn panic_free_modular_sub_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;


    // fn panic_free_modular_sub(&self, rhs: &Self, modulus: &Self) -> Self
    /// Calculates (`self` - `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// -`rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-sum (`self` + `rhs`) % `modulus` with wrapping
    /// (modular) addition at `modulus`.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulus`.
    /// - Wrapping (modular) subtraction at `modulus`.
    /// - The differences between this method `panic_free_modular_sub()`
    ///   and the method `wrapping_sub()` are, first, where
    ///   wrapping around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_sub()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_sub()` sets `UNDERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_sub_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub_uint)
    /// is a bit faster than this method `panic_free_modular_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_sub_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub_uint).
    /// 
    /// # Example 1 for Normal Case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    /// let one = U256::one();
    /// let res = a_biguint.panic_free_modular_sub(&one, &m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_sub)
    fn panic_free_modular_sub(&self, _rhs: &Self, _modulus: &Self) -> Self;


    // fn panic_free_modular_sub_assign(&mut self, rhs: &Self, modulus: &Self)
    /// Calculates (`self` - `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulus`.
    /// - Wrapping (modular) subtraction at `modulus`.
    /// - The differences between this method `panic_free_modular_sub_assign()`
    ///    and the method `wrapping_sub_assign()` are, first, where wrapping
    ///   around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_sub_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_sub_assign()` sets `UNDERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   underflow occurred even once before this current operation or
    ///   `UNDERFLOW` flag is already set before this current operation,
    ///   the `UNDERFLOW` flag is not changed even if this current operation
    ///   does not cause underflow.
    /// 
    /// # Counterpart Method
    /// - The method
    /// [panic_free_modular_sub_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub_assign)
    /// is a bit faster than this method `panic_free_modular_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_sub_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_sub_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_modular_sub_assign(&rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_sub_assign)
    fn panic_free_modular_sub_assign(&mut self, rhs: &Self, modulus: &Self);



    /*** MULTIPLICATION ***/

    // fn panic_free_modular_mul_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Calculates (`self` * `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-product (`self` * `rhs`) % `modulus` with wrapping
    /// (modular) multiplication at `modulus`.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulus`.
    /// - Wrapping (modular) multiplication at `modulus`.
    /// - The differences of between this method `modular_mul_uint()` and the
    ///   method `wrapping_mul_uint()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_mul_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_mul_uint()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_mul()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul)
    /// is proper rather than this method `panic_free_modular_mul_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_mul_uint)
    fn panic_free_modular_mul_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;


    // fn panic_free_modular_mul_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Calculates (`self` * `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulus`.
    /// - Wrapping (modular) multiplication at `modulus`.
    /// - The differences between this method
    ///   `panic_free_modular_mul_assign_uint()` and the method
    ///   `wrapping_mul_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_mul_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_mul_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_mul_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_uint = 5_u8;
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
    /// a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_mul_assign_uint)
    fn panic_free_modular_mul_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;
    

    // pub fn panic_free_modular_mul(&self, rhs: &Self, modulus: &Self) -> Self
    /// Calculates (`self` * `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulus-product (`self` * `rhs`) % `modulus` with wrapping
    /// (modular) multiplication at `modulus`.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulus`.
    /// - Wrapping (modular) multiplication at `modulus`.
    /// - The differences of between this method `modular_mul()` and the
    ///   method `wrapping_mul()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_mul()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_mul()` sets `OVERFLOW`
    ///   flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_mul_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul_uint)
    /// is a bit faster than this method `panic_free_modular_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_mul_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// let mul_biguint = UU32::from_uint(5_u8);
    /// let res = a_biguint.panic_free_modular_mul(&mul_biguint, &m);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_mul)
    fn panic_free_modular_mul(&self, rhs: &Self, modulus: &Self) -> Self;


    // pub fn panic_free_modular_mul_assign(&self, rhs: &Self, modulus: &Self)
    /// Calculates (`self` * `rhs`) % `modulus`,
    /// wrapping around at `modulus` of the `Self` type,
    /// and then assigns the result back to `self`.
    /// 
    /// # Arguments
    /// -`rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` * `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulus`.
    /// - Wrapping (modular) multiplication at `modulus`.
    /// - The differences between this method
    ///   `panic_free_modular_mul_assign()` and the method
    ///   `wrapping_mul_assign()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `wrapping_mul_assign()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `wrapping_mul_assign()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the result value will have the value `0`.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `modulus` | result value (self) | flags       |
    /// |----------|---------------------|-------------|
    /// | 0 or 1   | 0                   | `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_mul_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul_assign_uint)
    /// is a bit faster than this method `panic_free_modular_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_mul_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_mul_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
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
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_mul_assign)
    fn panic_free_modular_mul_assign(&mut self, _rhs: &Self, _modulus: &Self);



    /*** DIVISION ***/
                
    // fn panic_free_divide_fully_uint<U>(&self, rhs: U) -> (Self, Self)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of a quotient and a remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns tuple of a quotient and a remainder.
    /// The quotient is of `Self` type, and the remainder is of the primitive
    /// unsigned integral data type such as `u8`, `u16`, `u32`, `u64`,
    /// and `u128`.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If 'self' is zero and `rhs` is non-zero,
    ///   this method returns (zero, zero).
    /// - If both `rhs` and 'self' are zero, the quotient will be zero,
    ///   and its flags `UNDEFINED` and `DIVIDED_BY_ZERO` will be set,
    ///   and the remainder will be zero,
    ///   and its flag `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and 'self' is non-zero, the quotient will have
    ///   the maximum value of `Self`, and its flags `INFINITY` and
    ///   `DIVIDED_BY_ZERO` will be set,
    ///   and the remainder` will be zero,
    ///   and its flag `DIVIDED_BY_ZERO` will be set.
    /// - In summary, the quotient, the remainder and their flags
    ///   will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` | flags of `quotient`            | `remainder` | flags of `remainder` |
    /// |-------|--------|------------|--------------------------------|-------------|----------------------|
    /// | 0     | 0      | 0          | `UNDEFINED`, `DIVIDED_BY_ZERO` | 0           | `DIVIDED_BY_ZERO`    |
    /// | 0     | != 0   | max        | `INFINITY`, `DIVIDED_BY_ZERO`  | 0           | `DIVIDED_BY_ZERO`    |
    /// 
    /// - This function is the base function for all the methods
    ///   panic_free_*_div_uint(), panic_free_*_div_assign_uint(),
    ///   panic_free_*_rem_uint(), and panic_free_*_rem_assign_uint().
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_divide_fully()](trait@BigUInt_Panic_Free#tymethod.panic_free_divide_fully)
    /// is proper rather than this method `panic_free_divide_fully_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_divide_fully_uint)
    fn panic_free_divide_fully_uint<U>(&self, rhs: U) -> (Self, Self)
    where U: SmallUInt;

    // fn panic_free_div_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type,
    /// and the quotient would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_div_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is zero and `self` is not zero, the quotient will have
    ///   maximum value of `BigUInt` and the flags of the quotient,
    ///   `INFINITY` and `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and `self` is zero, the quotient will have
    ///   value `zero` of `BigUInt` type and the flags of the quotient,
    ///   `DIVIDED_BY_ZERO` and `UNDEFINED` will be set.
    /// - In summary, the quotient and its flags will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` | flags of `quotient`            |
    /// |-------|--------|------------|--------------------------------|
    /// | 0     | 0      | 0          | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0     | != 0   | max        | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_uint)
    /// is a bit faster than this method `wrapping_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_uint).
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let quotient = dividend.panic_free_div_uint(divisor);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_div_uint)
    fn panic_free_div_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt;

    // fn panic_free_div_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_div_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is zero and `self` is not zero, the quotient will have
    ///   maximum value of `BigUInt` and the flags of `self`,
    ///   `INFINITY` and `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and `self` is zero, the quotient will have
    ///   value `zero` of `BigUInt` type and the flags of `self`,
    ///   `DIVIDED_BY_ZERO` and `UNDEFINED` will be set.
    /// - In summary, the quotient and its flags will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` (= `self`) | flags of `quotient`            |
    /// |-------|--------|-----------------------|--------------------------------|
    /// | 0     | 0      | 0                     | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0     | != 0   | max                   | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_div_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_assign)
    /// is proper rather than this method `panic_free_div_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let divisor = 87_u8;
    /// a_biguint.panic_free_div_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_div_assign_uint)
    fn panic_free_div_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt;

    // fn panic_free_modular_div_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and returns the quotient.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when (`self` % `modulus`) is divided by
    /// (`rhs` % `modulus`) if (`rhs` % `modulus`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally returns the quotient of `rd1` divided by `rd2`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is zero or multiple of `modulus` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is not zero, and `modulus` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0 (mod `modulus`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulus`)    | != 0 (mod `modulus`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_div()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div)
    /// is proper rather than this method `panic_free_modular_div_uint()`.
    /// 
    /// # Example 1 for a normal case for modulus >= 2 and dividend != 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.modular_div_uint(divisor, &modulus);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_div_uint)
    fn panic_free_modular_div_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;

    // fn panic_free_modular_div_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and assigns the quotient back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally assigns the quotient of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is zero or multiple of `modulus` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is not zero, and `modulus` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0 (mod `modulus`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulus`)    | != 0 (mod `modulus`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_div_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulus = UU32::from_uint(100_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulus);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_div_assign_uint)
    fn panic_free_modular_div_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;

    // fn panic_free_divide_fully(&self, rhs: &Self) -> (Self, Self)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of a quotient and a remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of a quotient and a remainder.
    /// Both the quotient and the remainder are of `BigUInt` type.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If 'self' is zero and `rhs` is non-zero,
    ///   this method returns (zero, zero).
    /// - If both `rhs` and 'self' are zero, the quotient will be zero,
    ///   and its flags `UNDEFINED` and `DIVIDED_BY_ZERO` will be set,
    ///   and the remainder will be zero,
    ///   and its flag `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and 'self' is non-zero, the quotient will have
    ///   the maximum value of `Self`, and its flags `INFINITY` and
    ///   `DIVIDED_BY_ZERO` will be set,
    ///   and the remainder` will be zero,
    ///   and its flag `DIVIDED_BY_ZERO` will be set.
    /// - In summary, the quotient, the remainder and their flags
    ///   will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` | flags of `quotient`            | `remainder` | flags of `remainder` |
    /// |-------|--------|------------|--------------------------------|-------------|----------------------|
    /// | 0     | 0      | 0          | `UNDEFINED`, `DIVIDED_BY_ZERO` | 0           | `DIVIDED_BY_ZERO`    |
    /// | 0     | != 0   | max        | `INFINITY`, `DIVIDED_BY_ZERO`  | 0           | `DIVIDED_BY_ZERO`    |
    /// 
    /// - This function is the base function for all the methods
    ///   panic_free_*_div(), panic_free_*_div_assign(),
    ///   panic_free_*_rem(), and panic_free_*_rem_assign().
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_divide_fully_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_divide_fully_uint)
    /// is a bit faster than this method `panic_free_divide_fully()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_divide_fully_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_divide_fully_uint).
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.panic_free_divide_fully)
    fn panic_free_divide_fully(&self, rhs: &Self) -> (Self, Self);

    // fn panic_free_div(&self, rhs: &Self) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a quotient of `BigUInt` type,
    /// and the quotient would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_div()` will panic if `rhs` is zero.
    /// - If `rhs` is zero and `self` is not zero, the quotient will have
    ///   maximum value of `BigUInt` and the flags of the quotient,
    ///   `INFINITY` and `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and `self` is zero, the quotient will have
    ///   value `zero` of `BigUInt` type and the flags of the quotient,
    ///   `DIVIDED_BY_ZERO` and `UNDEFINED` will be set.
    /// - In summary, the quotient and its flags will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` | flags of `quotient`            |
    /// |-------|--------|------------|--------------------------------|
    /// | 0     | 0      | 0          | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0     | != 0   | max        | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_uint)
    /// is a bit faster than this method `panic_free_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_uint).
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.panic_free_div(&divisor);
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.panic_free_div)
    fn panic_free_div(&self, rhs: &Self) -> Self;

    // fn panic_free_div_assign(&mut self, rhs: &Self)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_div_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is zero and `self` is not zero, the quotient will have
    ///   maximum value of `BigUInt` and the flags of `self`,
    ///   `INFINITY` and `DIVIDED_BY_ZERO` will be set.
    /// - If `rhs` is zero and `self` is zero, the quotient will have
    ///   value `zero` of `BigUInt` type and the flags of `self`,
    ///   `DIVIDED_BY_ZERO` and `UNDEFINED` will be set.
    /// - In summary, the quotient and its flags will be set as follows:
    /// 
    /// | `rhs` | `self` | `quotient` (= `self`) | flags of `quotient`            |
    /// |-------|--------|-----------------------|--------------------------------|
    /// | 0     | 0      | 0                     | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0     | != 0   | max                   | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_div_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_assign_uint)
    /// is a bit faster than this method `panic_free_div_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_div_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_div_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
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
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint.panic_free_div_assign(&divisor);
    /// println!("After a_biguint.panic_free_div_assign(&divisor),\na_biguint = {}", a_biguint);
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
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.panic_free_div_assign)
    fn panic_free_div_assign(&mut self, rhs: &Self);

    // fn panic_free_modular_div(&self, rhs: &Self, modulus: &Self) -> Self
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and returns the quotient.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when (`self` % `modulus`) is divided by
    /// (`rhs` % `modulus`) if (`rhs` % `modulus`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally returns the quotient of `rd1` divided by `rd2`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is zero or multiple of `modulus` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is not zero, and `modulus` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0 (mod `modulus`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulus`)    | != 0 (mod `modulus`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div_uint)
    /// is a bit faster than this method `modular_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_div_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.panic_free_modular_div)
    fn panic_free_modular_div(&self, rhs: &Self, modulus: &Self) -> Self;

    // fn panic_free_modular_div_assign(&mut self, rhs: &Self, modulus: &Self)
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and assigns the quotient back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally assigns the quotient of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is zero or multiple of `modulus` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus`, and `self` is not zero, and `modulus` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0 (mod `modulus`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulus`)    | != 0 (mod `modulus`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_div_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div_assign_uint)
    /// is a bit faster than this method `panic_free_modular_div_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_div_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_div_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
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
    /// let modulus = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_arithmetic/struct.BigUInt.html#method.panic_free_modular_div_assign)
    fn panic_free_modular_div_assign(&mut self, rhs: &Self, modulus: &Self);

    // fn panic_free_rem_uint<U>(&self, rhs: U) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type,
    /// and the remainder would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_rem_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is `zero`, `self` will be `zero` and the `DIVIDED_BY_ZERO` flag
    ///   of `self` will be set.
    /// - In summary, the remainder and its flags will be set as follows:
    /// 
    /// | `rhs` | `remainder` (= `self`) | flags of `remainder` |
    /// |-------|------------------------|----------------------|
    /// | 0     | 0                      | `DIVIDED_BY_ZERO`    |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_uint)
    /// is a bit faster than this method `wrapping_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let remainder = dividend.panic_free_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_rem_uint)
    fn panic_free_rem_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt;

    // fn panic_free_rem_assign_uint<U>(&mut self, rhs: U)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of primitive unsigned integral data type
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_rem_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is `zero`, the remainder is `zero` and the flag
    ///   `DIVIDED_BY_ZERO` of `remainder` will be set.
    /// - In summary, the remainder and its flags will be set as follows:
    /// 
    /// | `rhs` | `remainder` | flags of `remainder` |
    /// |-------|-------------|----------------------|
    /// | 0     |  0          | `DIVIDED_BY_ZERO`    |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_rem_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_assign)
    /// is proper rather than this method `panic_free_rem_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 87_u8;
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_rem_assign_uint)
    fn panic_free_rem_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt;

    // fn panic_free_modular_rem_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and returns the remainder.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the remainder of when (`self` % `modulus`) is divided by
    /// (`rhs` % `modulus`) if (`rhs` % `modulus`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally returns the remainder of `rd1` divided by `rd2`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the remainder will have the value `zero` and
    ///   `DIVIDED_BY_ZERO` flag of the remainder will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the remainder and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | remainder | flags                          |
    /// |----------|---------------------|-----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | 0         | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | 0         | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0         | `DIVIDED_BY_ZERO`              |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_rem()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem)
    /// is proper rather than this method `panic_free_modular_rem_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulus = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulus);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulus);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_rem_uint)
    fn panic_free_modular_rem_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;

    // fn panic_free_modular_rem_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and assigns the remainder back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of primitive unsigned integral data type
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally assigns the remainder of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus` then, `self` will have the value `zero` and its
    ///   `DIVIDED_BY_ZERO` flag will be set.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | remainder (= `self`) | flags                          |
    /// |----------|---------------------|----------------------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | 0                    | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | 0                    | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0                    | `DIVIDED_BY_ZERO`              |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_rem_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
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
    /// let divisor = 128_u8;
    /// let modulus = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulus);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_rem_assign_uint)
    fn panic_free_modular_rem_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;

    // fn panic_free_rem(&self, rhs: &Self) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Output
    /// It returns a remainder of `BigUInt` type,
    /// and the remainder would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_rem()` will panic if `rhs` is zero.
    /// - If `rhs` is `zero`, the remainder is `zero` and the flag
    ///   `DIVIDED_BY_ZERO` of `remainder` will be set.
    /// - In summary, the remainder and its flags will be set as follows:
    /// 
    /// | `rhs` | `remainder` | flags of `remainder` |
    /// |-------|-------------|----------------------|
    /// | 0     |  0          | `DIVIDED_BY_ZERO`    |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_uint)
    /// is a bit faster than this method `panic_free_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.panic_free_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_rem)
    fn panic_free_rem(&self, rhs: &Self) -> Self;

    // fn panic_free_rem_assign(&mut self, rhs: &Self)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - __It does not panic__ while the counterpart method
    ///   `wrapping_rem_uint()` will panic if `rhs` is zero.
    /// - If `rhs` is `zero`, `self` will be `zero` and the `DIVIDED_BY_ZERO` flag
    ///   of `self` will be set.
    /// - In summary, the remainder and its flags will be set as follows:
    /// 
    /// | `rhs` | `remainder` (= `self`) | flags of `remainder` |
    /// |-------|------------------------|----------------------|
    /// | 0     | 0                      | `DIVIDED_BY_ZERO`    |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_rem_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_assign_uint)
    /// is a bit faster than this method `panic_free_rem_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_rem_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_rem_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
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
    /// a_biguint.panic_free_rem_assign(&divisor);
    /// println!("After a_biguint.panic_free_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_rem_assign)
    fn panic_free_rem_assign(&mut self, rhs: &Self);

    // fn panic_free_modular_rem(&self, rhs: &Self, modulus: &Self) -> Self
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and returns the remainder.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the remainder of when (`self` % `modulus`) is divided by
    /// (`rhs` % `modulus`) if (`rhs` % `modulus`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally returns the remainder of `rd1` divided by `rd2`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, the remainder will have the value `zero` and
    ///   `DIVIDED_BY_ZERO` flag of the remainder will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the remainder and the flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | remainder | flags                          |
    /// |----------|---------------------|-----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | 0         | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | 0         | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0         | `DIVIDED_BY_ZERO`              |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem_uint)
    /// is a bit faster than this method `modular_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_rem_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem(&divisor, &modulus);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulus);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_rem)
    fn panic_free_modular_rem(&self, rhs: &Self, modulus: &Self) -> Self;

    // fn panic_free_modular_rem_assign(&self, rhs: &Self, modulus: &Self)
    /// Divides (`self` % `modulus`) by (`rhs` % `modulus`),
    /// and assigns the remainder back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` divides `self`, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the remainder of (`self` / `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulus`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulus`,
    ///   and then finally assigns the remainder of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulus` or `modulus` is zero or one.
    /// - If `modulus` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulus` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulus` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulus` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` flag will be set.
    /// - If `modulus` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulus` then, `self` will have the value `zero` and its
    ///   `DIVIDED_BY_ZERO` flag will be set.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulus` | `rhs`               | remainder (= `self`) | flags                          |
    /// |----------|---------------------|----------------------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulus`)    | 0                    | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulus`) | 0                    | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulus`)    | 0                    | `DIVIDED_BY_ZERO`              |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_rem_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem_assign_uint)
    /// is a bit faster than this method `panic_free_modular_rem_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_rem_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_rem_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let modulus = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign(&divisor, &modulus);
    /// println!("After a_biguint.modular_rem_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_rem_assign)
    fn panic_free_modular_rem_assign(&mut self, rhs: &Self, modulus: &Self);



    /*** METHODS FOR EXPONENTIATION AND LOGARITHM ***/

    // fn panic_free_pow_uint<U>(&self, exp: U) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return will have the value `0`.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `exp` | return value | flags       |
    /// |--------|-------|--------------|-------------|
    /// | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// - If `exp` is bigger than `u128`, the method
    ///   [panic_free_pow()](trait@BigUInt_Panic_Free#tymethod.panic_free_pow)
    ///   is proper rather than this method `panic_free_pow_uint()`.
    /// - If you need to know whether or not overflow occurs, use the method
    ///   [overflowing_pow_uint()](trait@BigUInt_Panic_Free#tymethod.overflowing_pow_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let res = a_biguint.panic_free_pow_uint(exp);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_pow_uint)
    fn panic_free_pow_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt;

    // fn panic_free_pow_assign_uint<U>(&self, exp: U) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and assign the result to `self` back.
    /// The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
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
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of `self`
    ///   will be set and the result value (= `self`) will have the value `0`.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `exp` | result value | flags       |
    /// |--------|-------|--------------|-------------|
    /// | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// - If `exp` is bigger than `u128`, the method
    ///   [panic_free_pow_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_pow_assign)
    ///   is proper rather than this method `panic_free_pow_assign_uint()`.
    /// - If you need to know whether or not overflow occurs, use the method
    ///   [overflowing_pow_assign_uint()](trait@BigUInt_Panic_Free#tymethod.overflowing_pow_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let exp = 10_u8;
    /// a_biguint.panic_free_pow_assign_uint(exp);
    /// println!("After a_biguint.panic_free_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "10000000000");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_pow_assign_uint)
    fn panic_free_pow_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt;

    // fn panic_free_modular_pow_uint<U>(&self, exp: U, modulus: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`,
    /// and returns the result. The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is a primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulus` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulus`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | `self` | `exp` | return value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, use the method
    /// [panic_free_modular_pow()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow)
    /// instead.
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = 30_u8;
    /// let modulus = U256::halfmax();
    /// let res = a_biguint.modular_pow_uint(exp, &modulus);
    /// println!("{} ** {} (mod {}) = {}", a_biguint, exp, modulus, res);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.carrying_add_assign_uint)
    fn panic_free_modular_pow_uint<U>(&self, exp: U, modulus: &Self) -> Self
    where U: SmallUInt;

    // fn panic_free_modular_pow_assign_uint<U>(&mut self, exp: U, modulus: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`,
    /// and assign the result to `self` back.
    /// The type `U` has the trait `SmallUInt`.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is a primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulus` of the `Self` type`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | `self` | `exp` | result value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [panic_free_modular_pow_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let exp = 30_u8;
    /// let modulus = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign_uint(exp, &modulus);
    /// println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulus, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_pow_assign_uint)
    fn panic_free_modular_pow_assign_uint<U>(&mut self, exp: U, modulus: &Self)
    where U: SmallUInt;

    // fn panic_free_pow(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of the return value will
    ///   be set.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return will have the value `0`.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `exp` | return value | flags       |
    /// |--------|-------|--------------|-------------|
    /// | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_pow_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_pow_uint)
    /// is more efficient than this method `panic_free_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_pow_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.panic_free_pow(&exp);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_pow)
    fn panic_free_pow(&self, exp: &Self) -> Self;

    // fn panic_free_pow_assign(&mut self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, the `OVERFLOW` flag of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of `self`
    ///   will be set and the result value (= `self`) will have the value `0`.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `exp` | result value | flags       |
    /// |--------|-------|--------------|-------------|
    /// | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_pow_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_pow_assign_uint)
    /// is more efficient than this method `panic_free_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number, use
    /// the method [pow_assign_uint()](trait@BigUInt_Panic_Free#tymethod.pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_pow_assign(&exp);
    /// println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_pow_assign)
    fn panic_free_pow_assign(&mut self, exp: &Self);

    // fn panic_free_modular_pow(&self, exp: &Self, modulus: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulus` of the `Self` type`.
    /// - If overflowing (wrapping around at `modulus`) happens,
    ///   the `OVERFLOW` flag of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | `self` | `exp` | return value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_modular_pow_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow_uint)
    /// is more efficient than this method `panic_free_modular_pow()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_modular_pow_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let modulus = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulus);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulus);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_pow)
    fn panic_free_modular_pow(&self, exp: &Self, modulus: &Self) -> Self;

    // fn panic_free_modular_pow_assign(&mut self, exp: &Self, modulus: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at `modulus` of the `Self` type`,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// - `exp` is the power to raise `self` to, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of (`self` ** `exp`),
    ///    and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulus` of the `Self` type`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// - If `modulus` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulus` | `self` | `exp` | result value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_modular_pow_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow_assign_uint)
    /// is more efficient than this method `panic_free_modular_pow_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_modular_pow_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_modular_pow_assign_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let modulus = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulus);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulus, a_biguint);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_pow_assign)
    fn panic_free_modular_pow_assign(&mut self, exp: &Self, modulus: &Self);


    // fn panic_free_iroot_uint<U>(&self, exp: U) -> Self
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
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - If `exp` is zero and `self` is either zero or one,
    ///   the return value will be zero and 
    ///   the flags `UNDEFINED` of the return value will be set.
    /// - If `exp` is zero and `self` is greater than one, the return value
    ///   will be the maximum and the flags `UNDEFINED`, and `INFINITY`
    ///   of the return value will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `exp` | `self` | return value | flags                   |
    /// |-------|--------|--------------|-------------------------|
    /// | 0     | 0 or 1 | 0            | `UNDEFINED`             |
    /// | 0     | >= 2   | max          | `INFINITY`, `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [panic_free_iroot()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = 8_u8;
    /// let res = a_biguint.panic_free_iroot_uint(exp);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_iroot_uint)
    fn panic_free_iroot_uint<U>(&self, exp: U) -> Self
    where U: SmallUInt;

    // fn panic_free_iroot_assign_uint<U>(&mut self, exp: U)
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self` and is a primitive
    /// unsigned integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - `self` will be the exp-th root of `self` or biggest value under the
    ///   exp-th root of `self`.
    /// - The result of this method is never greater than `self` and so
    ///   never causes overflow.
    /// - If `exp` is greater than zero and `self` is greater than 1, `self`
    ///   will never be greater than `self` and so it never causes overflow.
    /// - If `exp` is zero and `self` is either zero or one, `self` will be
    ///   zero and the flags `UNDEFINED` of the return value will be set.
    /// - If `exp` is zero and `self` is greater than one, `self` will be the
    ///   maximum and the flags `UNDEFINED` and `INFINITY` of `self` will be
    ///   set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `exp` | `self` | result | flags                   |
    /// |-------|--------|--------|-------------------------|
    /// | 0     | 0 or 1 | 0      | `UNDEFINED`             |
    /// | 0     | >= 2   | max    | `INFINITY`, `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [panic_free_iroot_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_iroot_assign_uint(exp);
    /// println!("After a_biguint.panic_free_iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_iroot_assign_uint)
    fn panic_free_iroot_assign_uint<U>(&mut self, exp: U)
    where U: SmallUInt;

    // fn panic_free_iroot(&self, exp: &Self) -> Self
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
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - If `exp` is zero and `self` is either zero or one,
    ///   the return value will be zero and 
    ///   the flags `UNDEFINED` of the return value will be set.
    /// - If `exp` is zero and `self` is greater than one, the return value
    ///   will be the maximum and the flags `UNDEFINED`, and `INFINITY`
    ///   of the return value will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `exp` | `self` | return value | flags                   |
    /// |-------|--------|--------------|-------------------------|
    /// | 0     | 0 or 1 | 0            | `UNDEFINED`             |
    /// | 0     | >= 2   | max          | `INFINITY`, `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_iroot_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot_uint)
    /// is a bit faster than this method `panic_free_iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [panic_free_iroot_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_iroot)
    fn panic_free_iroot(&self, exp: &Self) -> Self;

    // fn panic_free_iroot_assign(&mut self, exp: &Self)
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - `self` will be the exp-th root of `self` or biggest value under the
    ///   exp-th root of `self`.
    /// - The result of this method is never greater than `self` and so
    ///   never causes overflow.
    /// - If `exp` is greater than zero and `self` is greater than 1, `self`
    ///   will never be greater than `self` and so it never causes overflow.
    /// - If `exp` is zero and `self` is either zero or one, `self` will be
    ///   zero and the flags `UNDEFINED` of the return value will be set.
    /// - If `exp` is zero and `self` is greater than one, `self` will be the
    ///   maximum and the flags `UNDEFINED` and `INFINITY` of `self` will be
    ///   set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `exp` | `self` | result | flags                   |
    /// |-------|--------|--------|-------------------------|
    /// | 0     | 0 or 1 | 0      | `UNDEFINED`             |
    /// | 0     | >= 2   | max    | `INFINITY`, `UNDEFINED` |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_iroot_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot_assign_uint)
    /// is a bit faster than this method `panic_free_iroot_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [panic_free_iroot_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_iroot_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_iroot_assign)
    fn panic_free_iroot_assign(&mut self, exp: &Self);

    // fn panic_free_ilog_uint<U>(&self, base: U) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
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
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    /// 
    /// # Features
    /// - If `self` is zero, the return value will be zero and the flag
    ///   `UNDEFINED` of the return value will be set.
    /// - If `self` is one and `base` is either zero or one, the return
    ///   value will be zero and the flag `UNDEFINED` of the return
    ///   value will be set.
    /// - If `self` is greater than 1 and `base` is either zero or one, the return
    ///   value will be maximum value and the flags `UNDEFINED` and `INFINITY`
    ///   of the return value will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `self` | `base` | result | flags                   |
    /// |--------|--------|--------|-------------------------|
    /// | 0      | --     | 0      | `UNDEFINED`             |
    /// | 1      | 0 or 1 | 0      | `UNDEFINED`             |
    /// | >= 2   | 0 or 1 | max    | `UNDEFINED`, `INFINITY` |
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [panic_free_ilog2()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// let res = a_biguint.panic_free_ilog_uint(base);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog_uint)
    fn panic_free_ilog_uint<U>(&self, base: U) -> Self
    where U: SmallUInt;

    // fn panic_free_ilog_assign_uint<U>(&mut self, base: U)
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and assigns the result back to `self`.
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
    /// # Features
    /// - If `self` is zero, the result will be zero and the flag
    ///   `UNDEFINED` of `self` will be set.
    /// - If `self` is one and `base` is either zero or one, the result
    ///   will be zero and the flag `UNDEFINED` of `self` will be set.
    /// - If `self` is greater than 1 and `base` is either zero or one,
    ///   the result will be maximum value and the flags `UNDEFINED`
    ///   and `INFINITY` of `self` will be set.
    /// - In summary, the result and the flags of `self` will be set as follows:
    /// 
    /// | `self` | `base` | result | flags                   |
    /// |--------|--------|--------|-------------------------|
    /// | 0      | --     | 0      | `UNDEFINED`             |
    /// | 1      | 0 or 1 | 0      | `UNDEFINED`             |
    /// | >= 2   | 0 or 1 | max    | `UNDEFINED`, `INFINITY` |
    /// 
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [panic_free_ilog2_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog2_assign)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
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
    /// let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    /// a_biguint.panic_free_ilog_assign_uint(base);
    /// println!("After a_biguint.panic_free_ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog_assign_uint)
    fn panic_free_ilog_assign_uint<U>(&mut self, base: U)
    where U: SmallUInt;

    // fn panic_free_ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
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
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    ///
    /// # Features
    /// - If `self` is zero, the return value will be zero and the flag
    ///   `UNDEFINED` of the return value will be set.
    /// - If `self` is one and `base` is either zero or one, the return
    ///   value will be zero and the flag `UNDEFINED` of the return
    ///   value will be set.
    /// - If `self` is greater than 1 and `base` is either zero or one, the return
    ///   value will be maximum value and the flags `UNDEFINED` and `INFINITY`
    ///   of the return value will be set.
    /// - In summary, the return value and its flags will be set as follows:
    ///
    /// | `self` | `base` | result | flags                   |
    /// |--------|--------|--------|-------------------------|
    /// | 0      | --     | 0      | `UNDEFINED`             |
    /// | 1      | 0 or 1 | 0      | `UNDEFINED`             |
    /// | >= 2   | 0 or 1 | max    | `UNDEFINED`, `INFINITY` |
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [panic_free_ilog2()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.panic_free_ilog(&base);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog)
    fn panic_free_ilog(&self, base: &Self) -> Self;

    // fn panic_free_ilog_assign(&mut self, base: &Self)
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    ///
    /// # Features
    /// - If `self` is zero, the result will be zero and the flag
    ///   `UNDEFINED` of `self` will be set.
    /// - If `self` is one and `base` is either zero or one, the result
    ///   will be zero and the flag `UNDEFINED` of `self` will be set.
    /// - If `self` is greater than 1 and `base` is either zero or one,
    ///   the result will be maximum value and the flags `UNDEFINED`
    ///   and `INFINITY` of `self` will be set.
    /// - In summary, the result and the flags of `self` will be set as follows:
    ///
    /// | `self` | `base` | result | flags                   |
    /// |--------|--------|--------|-------------------------|
    /// | 0      | --     | 0      | `UNDEFINED`             |
    /// | 1      | 0 or 1 | 0      | `UNDEFINED`             |
    /// | >= 2   | 0 or 1 | max    | `UNDEFINED`, `INFINITY` |
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [panic_free_ilog2_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog2_assign)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
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
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog_assign)
    fn panic_free_ilog_assign(&mut self, base: &Self);

    // fn panic_free_ilog2(&self) -> Self
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Features
    /// If `self` is zero, the return value will be zero and the flag
    /// `UNDEFINED` of the return value will be set.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [ilog_uint()](trait@BigUInt_Panic_Free#tymethod.ilog_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10()](trait@BigUInt_Panic_Free#tymethod.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.panic_free_ilog2();
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog2)
    fn panic_free_ilog2(&self) -> Self;

    // fn panic_free_ilog2_assign(&mut self)
    /// Calculates the base 2 logarithm of the number, rounded down,
    /// and assigns back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// If `self` is zero, the result will be zero and the flag
    /// `UNDEFINED` of `self` will be set.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [ilog_assign_uint()](trait@BigUInt_Panic_Free#tymethod.ilog_assign_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10_assign()](trait@BigUInt_Panic_Free#tymethod.ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog2_assign)
    fn panic_free_ilog2_assign(&mut self);

    // fn panic_free_ilog10(&self) -> Self
    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// If `self` is zero, the return value will be zero and the flag
    /// `UNDEFINED` of the return value will be set.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 10;
    /// [painc_free_ilog_uint()](trait@BigUInt_Panic_Free#tymethod.painc_free_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [painc_free_ilog2()](trait@BigUInt_Panic_Free#tymethod.painc_free_ilog2)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.panic_free_ilog10();
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog10)
    fn panic_free_ilog10(&self) -> Self;

    // fn panic_free_ilog10_assign(&mut self)
    /// Calculates the base 10 logarithm of the number, rounded down,
    /// and assigns back to `self`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// If `self` is zero, the result will be zero and the flag
    /// `UNDEFINED` of `self` will be set.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [panic_free_ilog_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog_assign_uint)
    /// can produce results of the base other than 10, and
    /// [panic_free_ilog2_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_ilog2_assign)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_ilog10_assign();
    /// println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_ilog10_assign)
    fn panic_free_ilog10_assign(&mut self);



    /*** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS ***/

    // fn panic_free_gcd_uint<U>(&self, other: U) -> Self
    /// Calculates the greatest common divisor of `self` and `other`,
    /// and returns the result.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Greatest_common_divisor).
    /// 
    /// # Argument
    /// The greatest common diviser of `self` and `other` is calculated.
    /// `other` is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns The greatest common diviser of `self` and `other`.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting greatest common diviser is meaningless.
    ///   In this case, this method returns zero,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | return value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_gcd()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd)
    /// is proper rather than this method `panic_free_gcd_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// let c_biguint = a_biguint.panic_free_gcd_uint(b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "11111");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_gcd_uint)
    fn panic_free_gcd_uint<U>(&self, other: U) -> Self
    where U: SmallUInt;

    // fn panic_free_gcd_assign_uint<U>(&mut self, other: U)
    /// Calculates the greatest common divisor of `self` and `other`,
    /// and assigns the result back to `self`.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Greatest_common_divisor).
    /// 
    /// # Argument
    /// The greatest common diviser of `self` and `other` is calculated.
    /// `other` is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting greatest common diviser is meaningless.
    ///   In this case, this method assigns zero to `self`,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | result value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_gcd_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd_assign)
    /// is proper rather than this method `panic_free_gcd_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
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
    /// a_biguint.panic_free_gcd_assign_uint(b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "11111");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_gcd_assign_uint)
    fn panic_free_gcd_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt;

    // fn panic_free_lcm_uint<U>(&self, other: U) -> Self
    /// Calculates the least common multiple of `self` and `other`,
    /// and returns the result.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The least common multiple of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns The least common multiple of `self` and `other`.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting least common multiple is meaningless.
    ///   In this case, this method returns zero,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | return value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_lcm()](trait@BigUInt_Panic_Free#tymethod.panic_free_lcm)
    /// is proper rather than this method `panic_free_lcm_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// let c_biguint = a_biguint.panic_free_lcm_uint(b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "7777922224222246666944447444475555866662777741110777774888865555388888");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_lcm_uint)
    fn panic_free_lcm_uint<U>(&self, other: U) -> Self
    where U: SmallUInt;

    // fn panic_free_lcm_assign_uint<U>(&mut self, other: U)
    /// Calculates the least common multiple of `self` and `other`,
    /// and assigns the result back to `self`.
    /// If you would like to know least common multiple more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The least common multiple of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting least common multiple is meaningless.
    ///   In this case, this method assigns zero to `self`,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | result value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_lcm_assign()](trait@BigUInt_Panic_Free#tymethod.panic_free_lcm_assign)
    /// is proper rather than this method `panic_free_lcm_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
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
    /// a_biguint.panic_free_lcm_assign_uint(b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign_uint(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "7777922224222246666944447444475555866662777741110777774888865555388888");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_lcm_assign_uint)
    fn panic_free_lcm_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt;


    // fn panic_free_gcd(&self, other: &Self) -> Self
    /// Calculates the greatest common divisor of `self` and `other`,
    /// and returns the result.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Greatest_common_divisor).
    /// 
    /// # Argument
    /// The greatest common diviser of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns The greatest common diviser of `self` and `other`.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting greatest common diviser is meaningless.
    ///   In this case, this method returns zero,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | return value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_gcd_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd_uint)
    /// is more efficient than this method `panic_free_gcd()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [panic_free_gcd_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "27");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_gcd)
    fn panic_free_gcd(&self, other: &Self) -> Self;


    // fn panic_free_gcd_assign(&mut self, other: &Self)
    /// Calculates the greatest common divisor of `self` and `other`,
    /// and assigns the result back to `self`.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Greatest_common_divisor).
    /// 
    /// # Argument
    /// The greatest common diviser of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting greatest common diviser is meaningless.
    ///   In this case, this method assigns zero to `self`,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | result value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_gcd_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd_assign_uint)
    /// is more efficient than this method `panic_free_gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [panic_free_gcd_assign_uint()](trait@BigUInt_Panic_Free#tymethod.panic_free_gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "27");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_gcd_assign)
    fn panic_free_gcd_assign(&mut self, other: &Self);

    
    // fn panic_free_lcm(&self, other: &Self) -> Self
    /// Calculates the least common multiple of `self` and `other`,
    /// and returns the result.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The least common multiple of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns The least common multiple of `self` and `other`.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting greatest common diviser is meaningless.
    ///   In this case, this method returns zero,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the return value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | return value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [lcm_uint()](trait@BigUInt_Panic_Free#tymethod.lcm_uint)
    /// is more efficient than this method `lcm()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [lcm_uint()](trait@BigUInt_Panic_Free#tymethod.lcm_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    /// let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_lcm)
    fn panic_free_lcm(&self, other: &Self) -> Self;

    // fn panic_free_lcm_assign(&mut self, other: &Self)
    /// Calculates the greatest common divisor of `self` and `other`,
    /// and assigns the result back to `self`.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The greatest common diviser of `self` and `other` is calculated.
    /// `other` is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Both `self` and `other` should natural numbers. So, if either `self`
    ///   or `other` is zero, getting least common multiple is meaningless.
    ///   In this case, this method assigns zero to `self`,
    ///   and sets `UNDEFINED` flag of the return value.
    /// - If either `self` or `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - If both `self` and `other` is zero, the result value will be zero,
    ///   and its `UNDEFINED` flag will be set.
    /// - In summary, the result value and its flags will be set as follows:
    /// 
    /// | `self` | `other` | result value | flags       |
    /// |--------|---------|--------------|-------------|
    /// | 0      | >= 1    | 0            | `UNDEFINED` |
    /// | >= 1   | 0       | 0            | `UNDEFINED` |
    /// | 0      | 0       | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [gcd_assign_uint()](trait@BigUInt_Panic_Free#tymethod.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](trait@BigUInt_Panic_Free#tymethod.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_lcm_assign)
    fn panic_free_lcm_assign(&mut self, other: &Self);

    // fn panic_free_next_multiple_of_uint<U>(&self, rhs: U) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and returns the result.
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
    /// - It returns the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`.
    ///   However, if overflow occurs, it returns the value wrapped around.
    /// - If `rhs` is zero, the `UNDEFINED` flag will be set and it returns
    ///   `zero`.
    /// 
    /// # Features
    /// - The result will be the smallest value greater than or equal to self,
    ///   which is a multiple of `rhs`. However, if overflow occurs,
    ///   the result will be the value wrapped around.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_next_multiple_of()](#tymethod.panic_free_next_multiple_of)
    /// is proper rather than this method `panic_free_next_multiple_of_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = 586478_u32;
    /// let multiple = a_biguint.panic_free_next_multiple_of_uint(num);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_next_multiple_of_uint)
    fn panic_free_next_multiple_of_uint<U>(&self, rhs: U) -> Self
    where U: SmallUInt;

    // fn panic_free_next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is a primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`.
    ///   However, if overflow occurs, `self` will be the value wrapped around.
    /// - If `rhs` is zero, the `UNDEFINED` flag will be set and `self`
    ///   will be `zero`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_next_multiple_of_assign()](#tymethod.panic_free_next_multiple_of_assign)
    /// is proper rather than this method `panic_free_next_multiple_of_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_next_multiple_of_assign_uint)
    fn panic_free_next_multiple_of_assign_uint<U>(&mut self, rhs: U)
    where U: SmallUInt;
    
    // fn panic_free_modular_next_multiple_of_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulus`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is a primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulus`. So,
    ///   if overflow occurs, it returns the value wrapped around at `modulus`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulus`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of_uint()`
    ///   and the method `panic_free_next_multiple_of_uint()` are, first,
    ///   where wrapping around happens, and, second, when `OVERFLOW` flag is
    ///   set. First, this method wraps araound at `modulus` while the method
    ///   `panic_free_next_multiple_of_uint()` wraps araound at `maximum
    ///   value + 1`. Second, this method set `OVERFLOW` flag when wrapping
    ///   around happens at `modulus` while the method
    ///   `panic_free_next_multiple_of_uint()` sets the `OVERFLOW` flag
    ///   when wrapping around happens.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [next_multiple_of()](#tymethod.next_multiple_of)
    /// is proper rather than this method `next_multiple_of_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = 100_u8;
    /// let modulus = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulus);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_next_multiple_of_uint)
    fn panic_free_modular_next_multiple_of_uint<U>(&self, rhs: U, modulus: &Self) -> Self
    where U: SmallUInt;
    
    // fn panic_free_modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulus`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is a primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulus` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulus`.
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulus`. So, if
    ///   overflow occurs, `self` will be the value wrapped around at `modulus`.
    /// - If `rhs` is zero, it assigns `zero` to `self` back
    ///   and the `UNDEFINED` flag of `self` will be set.
    /// - If `modulus` is either `zero` or `one`, it assigns `zero`
    ///   to `self` back and the `UNDEFINED` flag of `self` will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of_assign_uint()`
    ///   and the method `panic_free_next_multiple_of_assign_uint()` are, first,
    ///   where wrapping around happens, and, second, when `OVERFLOW` flag is
    ///   set. First, this method wraps araound at `modulus` while the method
    ///   `panic_free_next_multiple_of_assign_uint()` wraps araound at `maximum
    ///   value + 1`. Second, this method set `OVERFLOW` flag when wrapping
    ///   around happens at `modulus` while the method
    ///   `panic_free_next_multiple_of_assign_uint()` sets the `OVERFLOW` flag
    ///   when wrapping around happens.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_next_multiple_of_assign()](#tymethod.panic_free_modular_next_multiple_of_assign)
    /// is proper rather than this method `panic_free_modular_next_multiple_of_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
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
    /// let modulus = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulus);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_next_multiple_of_assign_uint)
    fn panic_free_modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulus: &Self)
    where U: SmallUInt;

    // fn panic_free_next_multiple_of(&self, rhs: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and returns the result.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`.
    ///   However, if overflow occurs, it returns the value wrapped around.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// 
    /// # Features
    /// - The result will be the smallest value greater than or equal to self,
    ///   which is a multiple of `rhs`. However, if overflow occurs,
    ///   the result will be the value wrapped around.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_next_multiple_of_uint()](#tymethod.panic_free_next_multiple_of_uint)
    /// is a bit faster than this method `panic_free_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_next_multiple_of_uint()](#tymethod.panic_free_next_multiple_of_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(586478_u32);
    /// let multiple = a_biguint.panic_free_next_multiple_of(&num);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_next_multiple_of)
    fn panic_free_next_multiple_of(&self, rhs: &Self) -> Self;
    
    // fn panic_free_next_multiple_of_assign(&mut self, rhs: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is the base of multiple, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`.
    ///   However, if overflow occurs, `self` will be the value wrapped around.
    /// - If `rhs` is zero, the `UNDEFINED` flag will be set and `self`
    ///   will be `zero`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_next_multiple_of_assign_uint()](#tymethod.panic_free_next_multiple_of_assign_uint)
    /// is a bit faster than this method `panic_free_next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_next_multiple_of_assign_uint()](#tymethod.panic_free_next_multiple_of_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = UU32::from(586478_u32);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_next_multiple_of_assign(&num);
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
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_next_multiple_of_assign)
    fn panic_free_next_multiple_of_assign(&mut self, rhs: &Self);

    // fn panic_free_modular_next_multiple_of(&self, rhs: &Self, modulus: &Self) -> Self
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulus`,
    /// and returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulus`. So,
    ///   if overflow occurs, it returns the value wrapped around at `modulus`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// 
    /// # Feature
    /// - Wrapping (modular) arround at `modulus`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulus` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of()` and the method
    ///   `panic_free_next_multiple_of()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulus` while the method
    ///   `panic_free_next_multiple_of()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulus` while the method `panic_free_next_multiple_of()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_next_multiple_of_uint()](#tymethod.panic_free_modular_next_multiple_of_uint)
    /// is a bit faster than this method `panic_free_modular_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_next_multiple_of_uint()](#tymethod.panic_free_modular_next_multiple_of_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(100_u8);
    /// let modulus = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulus);
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_next_multiple_of)
    fn panic_free_modular_next_multiple_of(&self, rhs: &Self, modulus: &Self) -> Self;    
    
    // fn panic_free_modular_next_multiple_of_assign(&mut self, rhs: &Self, modulus: &Self)
    /// Calculates the smallest value greater than or equal to `self`,
    /// which is a multiple of `rhs`, wrapping around at `modulus`,
    /// and assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the base of multiple, and is of `&Self` type.
    /// - `modulus` is the divisor to divide the result of the calculation of
    ///   the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, and is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulus`.
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulus`. So, if
    ///   overflow occurs, `self` will be the value wrapped around at `modulus`.
    /// - If `rhs` is zero, it assigns `zero` to `self` back
    ///   and the `UNDEFINED` flag of `self` will be set.
    /// - If `modulus` is either `zero` or `one`, it assigns `zero`
    ///   to `self` back and the `UNDEFINED` flag of `self` will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of_assign()`
    ///   and the method `panic_free_next_multiple_of_assign()` are, first,
    ///   where wrapping around happens, and, second, when `OVERFLOW` flag is
    ///   set. First, this method wraps araound at `modulus` while the method
    ///   `panic_free_next_multiple_of_assign()` wraps araound at `maximum
    ///   value + 1`. Second, this method set `OVERFLOW` flag when wrapping
    ///   around happens at `modulus` while the method
    ///   `panic_free_next_multiple_of_assign()` sets the `OVERFLOW` flag
    ///   when wrapping around happens.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_next_multiple_of_assign_uint()](#tymethod.panic_free_modular_next_multiple_of_assign_uint)
    /// is a bit faster than this method `panic_free_modular_next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_next_multiple_of_assign_uint()](#tymethod.panic_free_modular_next_multiple_of_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// use cryptocol::number::BigUInt_Panic_Free;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// 
    /// let num = UU32::from(100_u8);
    /// let modulus = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulus);
    /// println!("After a_biguint.modular_next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123456800");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_panic_free/struct.BigUInt.html#method.panic_free_modular_next_multiple_of_assign)
    fn panic_free_modular_next_multiple_of_assign(&mut self, rhs: &Self, modulus: &Self);
}