// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The module that contains a big unsigned integer struct
//! with user-defined fixed size and its methods.

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
/// Trait `BigUInt_Prime` is for BigUInt
///
/// # Quick start
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::BigUInt_Prime` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::BigUInt_Prime;
/// ```
/// If you import (use) `cryptocol::number::BigUInt_Prime`, all the methods of
/// `BigUInt_Prime` are available immediately and automagically, as if such
/// primitive data types had the methods from the begining.
/// 
/// ## Example 2
/// ```
/// use cryptocol::define_utypes_with;
/// use cryptocol::number::BigUInt_Prime;
/// define_utypes_with!(u32);
///
/// let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
/// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
/// let c_biguint = a_biguint.gcd(&b_biguint);
/// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
/// assert_eq!(c_biguint.to_string(), "27");
///
/// let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
/// let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
/// let c_biguint = a_biguint.lcm(&b_biguint);
/// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
/// assert_eq!(c_biguint.to_string(), "77779222242222466669444474444755552444414444166664222202222077777");
///
/// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
/// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
/// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
/// assert_eq!(b, true);
/// ```
#[allow(non_camel_case_types)]
pub trait BigUInt_Prime<T, const N: usize> : Clone + Sized //+ Display + + ToString
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd

{
    /*** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS ***/

    // fn gcd_uint<U>(&self, other: &U) -> Self
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
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Output
    /// It returns The greatest common diviser of `self` and `other`.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting greatest common diviser is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [gcd()](trait@BigUInt_Prime#tymethod.gcd)
    /// is proper rather than this method `gcd_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// let c_biguint = a_biguint.gcd_uint(b_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.gcd_uint)
    fn gcd_uint<U>(&self, other: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn gcd_assign_uint<U>(&mut self, other: U)
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
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting greatest common diviser is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [gcd_assign()](trait@BigUInt_Prime#tymethod.gcd_assign)
    /// is proper rather than this method `gcd_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u64);
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
    /// a_biguint.gcd_assign_uint(b_biguint);
    /// println!("After a_biguint.gcd_assign_uint(), a_biguint = {}.", a_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.gcd_assign_uint)
    fn gcd_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn lcm_uint<U>(&self, other: U) -> Self
    /// Calculates the least common multiple of `self` and `other`,
    /// and returns the result.
    /// If you would like to know greatest common divisor more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The least common multiple of `self` and `other` is calculated.
    /// `other` is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Output
    /// It returns The least common multiple of `self` and `other`.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting least common multiple is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [lcm_uint()](trait@BigUInt_Prime#tymethod.lcm_uint)
    /// is more efficient than this method `lcm()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [lcm_uint()](trait@BigUInt_Prime#tymethod.lcm_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("1111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = 77777666665555544444333332222211111_u128;
    /// let c_biguint = a_biguint.lcm_uint(b_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.lcm_uint)
    fn lcm_uint<U>(&self, other: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn lcm_assign_uint<U>(&mut self, other: U)
    /// Calculates the least common multiple of `self` and `other`,
    /// and assigns the result back to `self`.
    /// If you would like to know least common multiple more in detail,
    /// read [here](https://en.wikipedia.org/wiki/Least_common_multiple).
    /// 
    /// # Argument
    /// The least common multiple of `self` and `other` is calculated.
    /// `other` is a primitive unsigned integer such as `u8`, `u16`, `u32`,
    /// `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting least common multiple is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// If `other` is bigger than `u128`, the method
    /// [lcm_assign()](trait@BigUInt_Prime#tymethod.lcm_assign)
    /// is proper rather than this method `lcm_assign_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u32);
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
    /// a_biguint.lcm_assign_uint(b_biguint);
    /// println!("After a_biguint.lcm_assign_uint(), a_biguint = {}.", a_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.lcm_assign_uint)
    fn lcm_assign_uint<U>(&mut self, other: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn gcd(&self, other: &Self) -> Self
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
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Output
    /// It returns The greatest common diviser of `self` and `other`.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting greatest common diviser is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [gcd_uint()](trait@BigUInt_Prime#tymethod.gcd_uint)
    /// is more efficient than this method `gcd()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_uint()](trait@BigUInt_Prime#tymethod.gcd_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000123456789").unwrap();
    /// let c_biguint = a_biguint.gcd(&b_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.gcd)
    fn gcd(&self, other: &Self) -> Self;

    // fn gcd_assign(&mut self, other: &Self)
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
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting greatest common diviser is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [gcd_assign_uint()](trait@BigUInt_Prime#tymethod.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](trait@BigUInt_Prime#tymethod.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u64);
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
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.gcd_assign)
    fn gcd_assign(&mut self, other: &Self);

    // fn lcm(&self, other: &Self) -> Self
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
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Output
    /// It returns The least common multiple of `self` and `other`.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting least common multiple is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [lcm_uint()](trait@BigUInt_Prime#tymethod.lcm_uint)
    /// is more efficient than this method `lcm()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [lcm_uint()](trait@BigUInt_Prime#tymethod.lcm_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    /// let b_biguint = U256::from_string("77777666665555544444333332222211111").unwrap();
    /// let c_biguint = a_biguint.lcm(&b_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.lcm)
    fn lcm(&self, other: &Self) -> Self;

    // fn lcm_assign(&mut self, other: &Self)
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
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If either `self` or `other` is zero, it will panic.
    /// - If both `self` and `other` is zero, it will panic.
    /// 
    /// # Features
    /// Both `self` and `other` should natural numbers. So, if either `self`
    /// or `other` is zero, getting greatest common diviser is meaningless.
    /// In this case, this method will panic.
    /// 
    /// # Counterpart Method
    /// The method [gcd_assign_uint()](trait@BigUInt_Prime#tymethod.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](trait@BigUInt_Prime#tymethod.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u32);
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
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
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
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.lcm_assign)
    fn lcm_assign(&mut self, other: &Self);


    /*** METHODS FOR PRIME NUMBER TEST ***/

    // fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool
    /// Tests a `BigUInt`-type object to find whether or not it is a
    /// primne number.
    /// 
    /// # Arguments
    /// The argument `repetition` defines how many times it tests whether or
    /// not `self` is a prime number. Usually, `5` is given to repetition`
    /// in order to achieve 99.9% accuracy.
    /// 
    /// # Output
    /// It returns `true` if it is a primne number.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - It uses the method `test_miller_rabin()` which uses
    ///   [Miller Rabin algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).
    /// - If this test results in composite number, the tested number is surely
    ///   a composite number. If this test results in prime number,
    ///   the probability that the tested number is not a prime number is
    ///   (1/4) ^ `repeatition`.
    ///   So, if `repeatition` is two and it results in prime number the
    ///   probability that the tested number is not a prime number is
    ///   1/16 (= 1/4 * 1/4). Therefore, if you test any number with
    ///   `repeatition` (= 5) and they all result in a prime number,
    ///   it is 99.9% that the number is a prime number.
    /// - However, for performance, if the number is less than 10000,
    ///   it does not use Miller-Rabin alogrithm but deterministic algorithm
    ///   so that the argument `repetition` is meaningless.
    /// - If the number is less than u32::MAX (= 4294967295_u32),
    ///   3 is enough for `repetition` for 100% certainty for determination of
    ///   prime number. This method tests the number with 2, 7, and 61.
    /// - If the number is less than u64::MAX (= 18446744073709551615_u64),
    ///   7 is enough for `repetition` for 100% certainty for determination of
    ///   prime number. This method tests the number with 2, 325, 9375, 28178,
    ///   450775, 9780504, and 1795265022.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1 for prime numer case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// use cryptocol::number::BigUInt_Prime;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    /// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    /// assert_eq!(b, true);
    /// ```
    /// 
    /// # For more examples,
    /// click [here](./documentation/big_uint_prime/struct.BigUInt.html#method.is_prime_using_miller_rabin)
    fn is_prime_using_miller_rabin(&self, repetition: usize) -> bool;
}
