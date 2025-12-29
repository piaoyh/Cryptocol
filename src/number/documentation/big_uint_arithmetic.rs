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
/// examples were moved to big_uint_arithmetic.rs.
pub struct BigUInt<T, const N: usize>
where T: TraitsBigUInt<T>
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}

impl<T, const N: usize> BigUInt<T, N>
where T: TraitsBigUInt<T>,
    Self: Sized + Clone + Display + Debug + ToString
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Rem<Output = Self> + RemAssign
        + Shl<i32, Output = Self> + ShlAssign<i32>
        + Shr<i32, Output = Self> + ShrAssign<i32>
        + BitAnd<Self, Output = Self> + BitAndAssign
        + BitOr<Self, Output = Self> + BitOrAssign
        + BitXor<Self, Output = Self> + BitXorAssign
        + Not<Output = Self>
        + From<T> + FromStr + From<[T; N]> + From<u32>
{

    
    /***** ARITHMATIC OPERATIONS WITH BIGUINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add(&self, rhs: &Self, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of an addition result `self` + `rhs` + `carry`
    /// along with a carry-out bit.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// - `carry` is of `bool` type so that `1` may be added to `self`
    ///   if `carry` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// "ternary addition" of two `Self`-typed operands and a carry-in bit, and
    /// returns an tuple of an addition result in `Self` type and a carry-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - This allows chaining together multiple additions to create even a
    ///   wider addition. This can be thought of as a big integer
    ///   "full adder", in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    ///   `overflowing_add()`, and the output carry reflects current
    ///   overflow.
    /// - The output carry is equal to the `OVERFLOW` flag of the return value.
    /// - If overflow happened, the flag `OVERFLOW` of the return value will
    ///   be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [carrying_add_uint()](struct@BigUInt#method.carrying_add_uint)
    /// is a bit faster than this method `carrying_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [carrying_add_uint()](struct@BigUInt#method.carrying_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    /// let a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// let (c_biguint_lo, carry) = a_biguint_lo.carrying_add(&b_biguint_lo, false);
    /// let (c_biguint_hi, overflow) = a_biguint_hi.carrying_add(&b_biguint_hi, carry);
    /// println!("{}:{} + {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    /// assert_eq!(overflow, false);
    /// assert_eq!(c_biguint_hi.is_overflow(), false);
    /// assert_eq!(c_biguint_hi.is_underflow(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    /// assert_eq!(carry, true);
    /// assert_eq!(c_biguint_lo.is_overflow(), true);
    /// assert_eq!(c_biguint_lo.is_underflow(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// ``` 
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// let a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    /// let a_biguint_lo = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    /// let b_biguint_hi = U256::zero();
    /// let b_biguint_lo = U256::one();
    /// 
    /// let (c_biguint_lo, carry) = a_biguint_lo.carrying_add(&b_biguint_lo, false);
    /// let (c_biguint_hi, overflow) = a_biguint_hi.carrying_add(&b_biguint_hi, carry);
    /// println!("{}:{} + {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// 
    /// assert_eq!(c_biguint_hi.to_string(), "0");
    /// assert_eq!(overflow, true);
    /// assert_eq!(c_biguint_hi.is_overflow(), true);
    /// assert_eq!(c_biguint_hi.is_underflow(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(carry, true);
    /// assert_eq!(c_biguint_lo.is_overflow(), true);
    /// assert_eq!(c_biguint_lo.is_underflow(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// ```
    pub fn carrying_add(&self, _rhs: &Self, _carry: bool) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn carrying_add_assign(&self, rhs: &Self, carry: bool) -> bool
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the addition result `self` + `rhs` + `carry` to `self` back,
    /// and return the resulting carry.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// - `carry` is of `bool` type so that `1` may be added to `self`
    ///   if `carry` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the output carry. It performs "ternary addition" of two
    /// `Self`-typed operands and a carry-in bit, and returns a carry-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - This allows chaining together multiple additions to create even a
    ///   wider addition. This can be thought of as a big integer "full adder",
    ///   in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    ///   `overflowing_add_assign()`, and the output carry reflect current
    ///   overflow.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [carrying_add_assign_uint()](struct@BigUInt#method.carrying_add_assign_uint)
    /// is a bit faster than this method `carrying_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [carrying_add_assign_uint()](struct@BigUInt#method.carrying_add_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    /// let mut a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// println!("Originally, a_biguint_hi = {}\na_biguint_lo = {}\nb_biguint_hi = {}\nb_biguint_lo = {}", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// print!("Operation is: {}:{} + {}:{} ", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// 
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// 
    /// let carry = a_biguint_lo.carrying_add_assign(&b_biguint_lo, false);
    /// let overflow = a_biguint_hi.carrying_add_assign(&b_biguint_hi, carry);
    /// 
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// println!("After a_biguint_lo.carrying_add_assign(&b_biguint_lo, false), a_biguint_lo = {}", a_biguint_lo);
    /// println!("After a_biguint_hi.carrying_add_assign(&b_biguint_hi, {}), a_biguint_hi = {}", carry, a_biguint_hi);
    /// 
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_biguint_lo.is_overflow(), true);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    /// let mut a_biguint_lo = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    /// let b_biguint_hi = U256::zero();
    /// let b_biguint_lo = U256::one();
    /// println!("Originally, a_biguint_hi = {}\na_biguint_lo = {}\nb_biguint_hi = {}\nb_biguint_lo = {}", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// print!("Operation is: {}:{} + {}:{} ", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    /// 
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// 
    /// let carry = a_biguint_lo.carrying_add_assign(&b_biguint_lo, false);
    /// let overflow = a_biguint_hi.carrying_add_assign(&b_biguint_hi, carry);
    /// 
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("carry = {}, overflow = {}", carry, overflow);
    /// println!("After a_biguint_lo.carrying_add_assign(&b_biguint_lo, false), a_biguint_lo = {}", a_biguint_lo);
    /// println!("After a_biguint_hi.carrying_add_assign(&b_biguint_hi, {}), a_biguint_hi = {}", carry, a_biguint_hi);
    /// 
    /// assert_eq!(a_biguint_lo.to_string(), "0");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_biguint_lo.is_overflow(), true);
    /// assert_eq!(a_biguint_lo.is_underflow(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_hi.to_string(), "0");
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint_hi.is_overflow(), true);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_hi.is_overflow(), false);
    /// assert_eq!(b_biguint_hi.is_underflow(), false);
    /// assert_eq!(b_biguint_hi.is_infinity(), false);
    /// assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_hi.is_undefined(), false);
    /// assert_eq!(b_biguint_hi.is_left_carry(), false);
    /// assert_eq!(b_biguint_hi.is_right_carry(), false);
    /// 
    /// assert_eq!(b_biguint_lo.is_overflow(), false);
    /// assert_eq!(b_biguint_lo.is_underflow(), false);
    /// assert_eq!(b_biguint_lo.is_infinity(), false);
    /// assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint_lo.is_undefined(), false);
    /// assert_eq!(b_biguint_lo.is_left_carry(), false);
    /// assert_eq!(b_biguint_lo.is_right_carry(), false);
    /// ```
    pub fn carrying_add_assign(&mut self, _rhs: &Self, _carry: bool) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_add(&self, rhs: &Self) -> Self
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` + `rhs` with wrapping (modular) addition.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_add_uint()](struct@BigUInt#method.wrapping_add_uint)
    /// is a bit faster than this method `wrapping_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_add_uint()](struct@BigUInt#method.wrapping_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let res = a_biguint.wrapping_add(&one_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let b_biguint = U512::max();
    /// let one_biguint = U512::one();
    /// let res = b_biguint.wrapping_add(&one_biguint);
    /// println!("{} + {} = {}", b_biguint, one_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
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
    /// define_utypes_with!(u32);
    /// 
    /// let c_biguint = U512::zero();
    /// let one_biguint = U512::one();
    /// let res = c_biguint.wrapping_add(&one_biguint);
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
    pub fn wrapping_add(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_add_assign(&mut self, rhs: &Self)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the addition result `self` + `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_add_assign_uint()](struct@BigUInt#method.wrapping_add_assign_uint)
    /// is a bit faster than this method `wrapping_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_add_assign_uint()](struct@BigUInt#method.wrapping_add_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
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
    /// a_biguint.wrapping_add_assign(&one_biguint);
    /// println!("After a_biguint.wrapping_add_assign(&U512::one()), a_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::max();
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
    /// a_biguint.wrapping_add_assign(&one_biguint);
    /// println!("After a_biguint.wrapping_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_add_assign(&one_biguint);
    /// println!("After a_biguint.wrapping_add_assign(&U512::one()),\ta_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn wrapping_add_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_add(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the addition result `self` + `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the addition `self` + `rhs` along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an overflow
    /// would have occurred, then the wrapped (modular) value is returned.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happens, the second element of the output tuple will
    ///   be true and the `OVERFLOW` flag of the return value will be set.
    /// - The second element of the output tuple reflects only
    ///   the current overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_add_uint()](struct@BigUInt#method.overflowing_add_uint)
    /// is a bit faster than this method `overflowing_add()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_add_uint()](struct@BigUInt#method.overflowing_add_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let one_biguint = U512::one();
    /// let (res, overflow) = a_biguint.overflowing_add(&one_biguint);
    /// println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, res, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let b_biguint = U512::max();
    /// let one_biguint = U512::one();
    /// let (res, overflow) = b_biguint.overflowing_add(&one_biguint);
    /// println!("{} + {} = {}, overflow = {}", b_biguint, one_biguint, res, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
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
    /// define_utypes_with!(u128);
    /// 
    /// let c_biguint = U512::max();
    /// let two_biguint = U512::from_uint(2_u8);
    /// let (res, overflow) = c_biguint.overflowing_add(&two_biguint);
    /// println!("{} + {} = {}, overflow = {}", c_biguint, two_biguint, res, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_add(&self, _rhs: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_add_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - If overflow did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - The output overflow reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_add_assign_uint()](struct@BigUInt#method.overflowing_add_assign_uint)
    /// is a bit faster than this method `overflowing_add_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_add_assign_uint()](struct@BigUInt#method.overflowing_add_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
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
    /// let overflow = a_biguint.overflowing_add_assign(&one_biguint);
    /// println!("After a_biguint.overflowing_add_assign(&U512::one()), a_biguint = {}, overflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::max();
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
    /// let mut overflow = a_biguint.overflowing_add_assign(&one_biguint);
    /// println!("After a_biguint.overflowing_add_assign(&U512::one()),\ta_biguint = {}, overflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// overflow = a_biguint.overflowing_add_assign(&one_biguint);
    /// println!("After a_biguint.overflowing_add_assign(&U512::one()),\ta_biguint = {}, overflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_add_assign(&mut self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Subtraction ***/

    // pub fn borrowing_sub(&self, rhs: &Self, borrow: bool) -> (Self, bool)
    /// Calculates `self` - `rhs` - `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of a subtraction result `self` - `rhs` - `carry`
    /// along with a borrow-out bit.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// - `borrow` is of `bool` type so that `1` may be subtracted from `self`
    ///   if `borrow` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing the subtraction result and the borrow-out
    /// bit. It performs "ternary subtraction" of one `Self`-typed operand,
    /// a primitive unsigned integer, and a borrow-in bit, and returns an tuple
    /// of an subtraction result in `Self` type and a borrow-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - This allows chaining together multiple subtraction to create even a
    ///   wider subtraction. This can be thought of as a big integer
    ///   "full subtracter", in the electronics sense.
    /// - If the input borrow is `false`, this method is equivalent to
    ///   `overflowing_sub()`, and the output borrow reflects current underflow.
    /// - The output borrow is equal to the `UNDERFLOW` flag
    ///   of the return value.
    /// - If underflow happened, the flag `UNDERFLOW` of the return value will
    ///   be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [borrowing_sub_uint()](struct@BigUInt#method.borrowing_sub_uint)
    /// is a bit faster than this method `borrowing_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [borrowing_sub_uint()](struct@BigUInt#method.borrowing_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    /// let a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// let (c_biguint_lo, borrow) = a_biguint_lo.borrowing_sub(&b_biguint_lo, false);
    /// let (c_biguint_hi, underflow) = a_biguint_hi.borrowing_sub(&b_biguint_hi, borrow);
    /// 
    /// println!("{}:{} - {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, overflow = {}", borrow, underflow);
    /// assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    /// assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    /// assert_eq!(borrow, true);
    /// assert_eq!(c_biguint_lo.is_underflow(), true);
    /// assert_eq!(c_biguint_lo.is_overflow(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(underflow, false);
    /// assert_eq!(c_biguint_hi.is_underflow(), false);
    /// assert_eq!(c_biguint_hi.is_overflow(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_hi = U256::zero();
    /// let a_biguint_lo = U256::zero();
    /// let b_biguint_hi = U256::zero();
    /// let b_biguint_lo = U256::one();
    ///
    /// let (c_biguint_lo, borrow) = a_biguint_lo.borrowing_sub(&b_biguint_lo, false);
    /// let (c_biguint_hi, underflow) = a_biguint_hi.borrowing_sub(&b_biguint_hi, borrow);
    /// 
    /// println!("{}:{} - {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, underflow = {}", borrow, underflow);
    /// 
    /// assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(borrow, true);
    /// assert_eq!(c_biguint_lo.is_underflow(), true);
    /// assert_eq!(c_biguint_lo.is_overflow(), false);
    /// assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_lo.is_infinity(), false);
    /// assert_eq!(c_biguint_lo.is_undefined(), false);
    /// assert_eq!(c_biguint_lo.is_left_carry(), false);
    /// assert_eq!(c_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(underflow, true);
    /// assert_eq!(c_biguint_hi.is_underflow(), true);
    /// assert_eq!(c_biguint_hi.is_overflow(), false);
    /// assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint_hi.is_infinity(), false);
    /// assert_eq!(c_biguint_hi.is_undefined(), false);
    /// assert_eq!(c_biguint_hi.is_left_carry(), false);
    /// assert_eq!(c_biguint_hi.is_right_carry(), false);
    /// ```
    pub fn borrowing_sub(&self, _rhs: &Self, _borrow: bool) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn borrowing_sub_assign(&self, rhs: &Self, borrow: bool) -> bool
    /// Calculates `self` - `rhs` - `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the subtraction result `self` - `rhs` - `carry`
    /// to `self` back,
    /// and return the resulting borrow.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// - `borrow` is of `bool` type so that `1` may be subtracted from `self`
    ///   if `borrow` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the output borrow. It performs "ternary subtraction" of two
    /// `Self`-typed operands, and a borrow-in bit,
    /// and returns a borrow-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - This allows chaining together multiple subtraction to create even a
    ///   wider subtraction. This can be thought of as a big integer
    ///   "full subtracter", in the electronics sense.
    /// - If the input borrow is false, this method is equivalent to
    ///   `overflowing_sub_assign()`, and the output borrow reflects
    ///   the current underflow.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [carrying_sub_assign_uint()](struct@BigUInt#method.carrying_sub_assign_uint)
    /// is a bit faster than this method `carrying_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, you are highly encouraged to use the method
    /// [carrying_sub_assign_uint()](struct@BigUInt#method.carrying_sub_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    /// let mut a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    /// let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    /// let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    /// 
    /// print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    /// let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, underflow = {}", borrow, underflow);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    /// assert_eq!(borrow, true);
    /// assert_eq!(a_biguint_lo.is_underflow(), true);
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint_hi.is_underflow(), false);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_hi = U256::zero();
    /// let mut a_biguint_lo = U256::zero();
    /// let b_biguint_hi = U256::zero();
    /// let b_biguint_lo = U256::one();
    /// 
    /// print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    /// let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, underflow = {}", borrow, underflow);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(borrow, true);
    /// assert_eq!(a_biguint_lo.is_underflow(), true);
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// assert_eq!(underflow, true);
    /// 
    /// assert_eq!(a_biguint_hi.is_underflow(), true);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// 
    /// print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    /// let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    /// println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    /// println!("borrow = {}, underflow = {}", borrow, underflow);
    /// 
    /// assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    /// assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFE");
    /// assert_eq!(borrow, false);
    /// assert_eq!(a_biguint_lo.is_underflow(), true);
    /// assert_eq!(a_biguint_lo.is_overflow(), false);
    /// assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_lo.is_infinity(), false);
    /// assert_eq!(a_biguint_lo.is_undefined(), false);
    /// assert_eq!(a_biguint_lo.is_left_carry(), false);
    /// assert_eq!(a_biguint_lo.is_right_carry(), false);
    /// 
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint_hi.is_underflow(), true);
    /// assert_eq!(a_biguint_hi.is_overflow(), false);
    /// assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_hi.is_infinity(), false);
    /// assert_eq!(a_biguint_hi.is_undefined(), false);
    /// assert_eq!(a_biguint_hi.is_left_carry(), false);
    /// assert_eq!(a_biguint_hi.is_right_carry(), false);
    /// ```
    pub fn borrowing_sub_assign(&mut self, _rhs: &Self, _borrow: bool) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_sub(&self, rhs: &Self) -> Self
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `self` - `rhs` with wrapping (modular) subtraction.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_sub_uint()](struct@BigUInt#method.wrapping_sub_uint)
    /// is a bit faster than this method `wrapping_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_sub_uint()](struct@BigUInt#method.wrapping_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub(&U512::one());
    /// println!("{} - 1 = {}", a_biguint, res);
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
    /// define_utypes_with!(u32);
    /// 
    /// let b_biguint = U512::zero();
    /// let res = b_biguint.wrapping_sub(&U512::one());
    /// println!("{} - 1 = {}", b_biguint, res);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_underflow(), true);
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
    /// define_utypes_with!(u32);
    /// 
    /// let c_biguint = U512::max();
    /// let res = c_biguint.wrapping_sub(&U512::one());
    /// println!("{} - 1 = {}", c_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn wrapping_sub(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_sub_assign(&mut self, rhs: &Self)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the subtraction result `self` - `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_sub_assign_uint()](struct@BigUInt#method.wrapping_sub_assign_uint)
    /// is a bit faster than this method `wrapping_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_sub_assign_uint()](struct@BigUInt#method.wrapping_sub_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_sub_assign(&U512::one());
    /// println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U512::zero());
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_sub_assign(&U512::one());
    /// println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(a_biguint.is_underflow(), true);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_sub_assign(&U512::one());
    /// println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn wrapping_sub_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_sub(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the subtraction result `self` - `rhs` along with
    /// a boolean indicating whether an arithmetic underflow would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the subtraction `self` - `rhs` along with a
    /// boolean indicating whether an arithmetic underflow would occur.
    /// If an underflow would have occurred, then the wrapped (modular) value
    /// is returned.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happens, the second element of the output tuple will
    ///   be true and the `UNDERFLOW` flag of the return value will be set.
    /// - The second element of the output tuple reflects only
    ///   the current underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_sub_uint()](struct@BigUInt#method.overflowing_sub_uint)
    /// is a bit faster than this method `overflowing_sub()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_sub_uint()](struct@BigUInt#method.overflowing_sub_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let (res, underflow) = a_biguint.overflowing_sub(&U512::one());
    /// println!("{} - 1 = {}, underflow = {}", a_biguint, res, underflow);
    /// assert_eq!(underflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let b_biguint = U512::zero();
    /// let (res, underflow) = b_biguint.overflowing_sub(&U512::one());
    /// println!("{} - 1 = {}, underflow = {}", b_biguint, res, underflow);
    /// assert_eq!(underflow, true);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_sub(&self, _rhs: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_sub_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the subtraction result `self` - `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic underflow
    /// would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic underflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - If underflow did not happen in the current operation, the output
    ///   will be false even if the `UNDERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - The output reflects only the current underflow.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_sub_assign_uint()](struct@BigUInt#method.overflowing_sub_assign_uint)
    /// is a bit faster than this method `overflowing_sub_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_sub_assign_uint()](struct@BigUInt#method.overflowing_sub_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// let underflow = a_biguint.overflowing_sub_assign(&U512::one());
    /// println!("After a_biguint.overflowing_sub_assign(&U512::one()), a_biguint = {}, underflow = {}", a_biguint, underflow);
    /// assert_eq!(underflow, false);
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
    /// define_utypes_with!(u8);
    /// 
    /// let mut b_biguint = U512::zero();
    /// println!("Originally, b_biguint = {}", b_biguint);
    /// assert_eq!(b_biguint.is_underflow(), false);
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = b_biguint.overflowing_sub_assign(&U512::one());
    /// println!("After b_biguint.overflowing_sub_assign(&U512::one()),\tb_biguint = {}, underflow = {}", b_biguint, underflow);
    /// assert_eq!(underflow, true);
    /// assert_eq!(b_biguint, U512::max());
    /// assert_eq!(b_biguint.is_underflow(), true);
    /// assert_eq!(b_biguint.is_overflow(), false);
    /// assert_eq!(b_biguint.is_divided_by_zero(), false);
    /// assert_eq!(b_biguint.is_infinity(), false);
    /// assert_eq!(b_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_sub_assign(&mut self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Multiplication ***/

    // pub fn carrying_mul(&self, rhs: &Self, carry: Self) -> (Self, Self)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple the low-order (wrapping) bits and the high-order
    /// (overflow) bits of the result of the calculation
    /// `self` * `rhs` + `carry`.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// - `carry` is of `Self` type
    ///   so that `carry` may be added to `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # 
    /// It returns the multiplication result `self` * `rhs` + `carry` in the
    /// form of a tuple of the low-order (wrapping) bits and the
    /// high-order (overflow) bits of the result as two separate values,
    /// in the order (`low`, `high`).
    /// 
    /// # Features
    /// - It performs "long multiplication" which takes in an extra amount
    ///   to add, and returns the result in a tuple containing a low-order
    ///   part and a high-order part of it. This allows for chaining together
    ///   multiple multiplications to create "bigger integers" which represent
    ///   larger values.
    /// - If the high-order part of the return value is not zero, the
    ///   `OVERFLOW` flag of the low-order part will be set though the output
    ///   tuple is free from overflow.
    /// - If the input carry is `0`, this method is equivalent to
    ///   `widening_mul()`.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    ///   [widening_mul()](struct@BigUInt#method.widening_mul) instead.
    /// - The value of the first field in the returned tuple matches
    ///   what you’d get by combining the methods
    ///   [wrapping_mul()](struct@BigUInt#method.wrapping_mul) and
    ///   [wrapping_add()](struct@BigUInt#method.wrapping_add):
    ///   `self.wrapping_mul(rhs).wrapping_add(carry)`. So,
    ///   `self.carrying_mul(rhs, carry).0` == `self.wrapping_mul(rhs).wrapping_add(carry)`
    /// - The method
    ///   [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint)
    ///   is a bit faster than this method `carrying_mul()`. If `rhs` is
    ///   primitive unsigned integral data type such as u8, u16, u32, u64, and
    ///   u128, use the method
    ///   [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_low = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_biguint_high = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_biguint = UU32::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    /// let (res_biguint_low, res_biguint_high) = a_biguint_low.carrying_mul(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// let (res_biguint_high, res_biguint_higher) = a_biguint_high.carrying_mul(&b_biguint, res_biguint_high);
    /// 
    /// println!("{}:{} X {} = {}:{}:{}", a_biguint_high, a_biguint_low, b_biguint, res_biguint_higher, res_biguint_high, res_biguint_low);
    /// assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    /// assert_eq!(res_biguint_high.is_overflow(), true);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint_low = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let a_biguint_high = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let b_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let (res_biguint_low, res_biguint_high) = a_biguint_low.carrying_mul(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// let (res_biguint_high, res_biguint_higher) = a_biguint_high.carrying_mul(&b_biguint, res_biguint_high);
    /// 
    /// println!("{}:{} X {} = {}:{}:{}", a_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), a_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// assert_eq!(res_biguint_high.is_overflow(), true);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string(), "1");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    pub fn carrying_mul(&self, _rhs: &Self, _carry: Self) -> (Self, Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn carrying_mul_assign(&mut self, rhs: &Self, carry: Self) -> Self
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the low-order (wrapping) bits of the result
    /// `self` * `rhs` + `carry` back to `self`,
    /// and returns the high-order (overflow) bits of the result.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// - `carry` is of `Self` type
    ///   so that `carry` may be added to `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result
    /// `self` * `rhs` + `carry`.
    /// 
    /// # Features
    /// - It performs "long multiplication" which takes in an extra amount
    ///   to add, and assigns the low-order part the result to `self` back,
    ///   and returns the high-order part of the result.
    /// - If the return value which is the high-order part of the result is
    ///   not zero, the `OVERFLOW` flag of `self` will be set
    ///   though the result is free from overflow because the `OVERFLOW` flag
    ///   is of `self`, and not of the result of the multiplication.
    /// - If the input carry is `0`, this method is equivalent to
    ///   `widening_mul_assign()`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    ///   [widening_mul_assign()](struct@BigUInt#method.widening_mul_assign)
    ///   instead.
    /// - The value of `self` after calculation matches what you’d get by
    ///   combining the mehtods
    ///   [wrapping_mul()](struct@BigUInt#method.wrapping_mul) and
    ///   [wrapping_add_assign()](struct@BigUInt#method.wrapping_add_assign_uint):
    ///   `self.wrapping_mul(rhs).wrapping_add_assign(carry)`.
    /// - The method
    ///   [carrying_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint)
    ///   is a bit faster than this method `carrying_mul_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128,
    ///   use the method
    ///   [carrying_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_low = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint_high = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_biguint = U256::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    /// println!("Originally, a_biguint_low = {}\na_biguint_high = {}", a_biguint_low, a_biguint_high);
    /// assert_eq!(a_biguint_low.is_overflow(), false);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.is_overflow(), false);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_higher = a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high);
    /// println!("After a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero()),\na_biguint_low = {}", a_biguint_low);
    /// println!("After a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high),\na_biguint_high = {}", a_biguint_high);
    /// println!("res_biguint_higher = {}", res_biguint_higher);
    /// assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    /// assert_eq!(a_biguint_high.is_overflow(), true);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    /// assert_eq!(a_biguint_low.is_overflow(), true);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint_low = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let mut a_biguint_high = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// let b_biguint = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    /// println!("Originally, a_biguint_low = {}\na_biguint_high = {}", a_biguint_low, a_biguint_high);
    /// assert_eq!(a_biguint_low.is_overflow(), false);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.is_overflow(), false);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero());
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// let res_biguint_higher = a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high);
    /// println!("After a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero()),\na_biguint_low = {}", a_biguint_low);
    /// println!("After a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high),\na_biguint_high = {}", a_biguint_high);
    /// println!("res_biguint_higher = {}", res_biguint_higher);
    /// assert_eq!(res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    /// assert_eq!(res_biguint_higher.is_overflow(), false);
    /// assert_eq!(res_biguint_higher.is_underflow(), false);
    /// assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_higher.is_infinity(), false);
    /// assert_eq!(res_biguint_higher.is_undefined(), false);
    /// assert_eq!(res_biguint_higher.is_left_carry(), false);
    /// assert_eq!(res_biguint_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    /// assert_eq!(a_biguint_high.is_overflow(), true);
    /// assert_eq!(a_biguint_high.is_underflow(), false);
    /// assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_high.is_infinity(), false);
    /// assert_eq!(a_biguint_high.is_undefined(), false);
    /// assert_eq!(a_biguint_high.is_left_carry(), false);
    /// assert_eq!(a_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint_low.to_string(), "1");
    /// assert_eq!(a_biguint_low.is_overflow(), true);
    /// assert_eq!(a_biguint_low.is_underflow(), false);
    /// assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint_low.is_infinity(), false);
    /// assert_eq!(a_biguint_low.is_undefined(), false);
    /// assert_eq!(a_biguint_low.is_left_carry(), false);
    /// assert_eq!(a_biguint_low.is_right_carry(), false);
    /// ```
    pub fn carrying_mul_assign(&mut self, _rhs: &Self, _carry: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn widening_mul(&self, rhs: &Self) -> (Self, Self)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple the low-order (wrapping) bits and the high-order
    /// (overflow) bits of the result of the calculation  `self` * `rhs`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns the multiplication result `self` * `rhs` in the form of a
    /// tuple of the low-order (wrapping) bits and the high-order
    /// (overflow) bits of the result as two separate values,
    /// in the order (`low`, `high`).
    /// 
    /// # Features
    /// - It performs "long multiplication", and returns the result in a tuple
    ///   containing a low-order part and a high-order part of it.
    /// - If the high-order part of the return value is not zero, the
    ///   `OVERFLOW` flag of the low-order part will be set though the output
    ///   tuple is free from overflow.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want to
    ///   use [carrying_mul()](struct@BigUInt#method.carrying_mul)
    ///   instead.
    /// - The value of the first field in the returned tuple matches what
    ///   you’d get the method
    ///   [wrapping_mul()](struct@BigUInt#method.wrapping_mul).
    ///   `self.widening_mul(rhs).0` == `self.wrapping_mul(rhs)`.
    /// - The method
    ///   [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint)
    ///   is a bit faster than this method `widening_mul()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8,
    ///   u16, u32, u64, and u128, use the method
    ///   [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    /// let (res_biguint_low, res_biguint_high) = a_biguint.widening_mul(&b_biguint);
    /// 
    /// println!("{} X {} = {}:{}", a_biguint, b_biguint, res_biguint_high, res_biguint_low);
    /// assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(reres_biguint_highs.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::max();
    /// let b_biguint = U256::max();
    /// let (res_biguint_low, res_biguint_high) = a_biguint.widening_mul(&b_biguint);
    /// 
    /// println!("{} X {} = {}:{}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap(), "1");
    /// assert_eq!(res_biguint_low.is_overflow(), true);
    /// assert_eq!(res_biguint_low.is_underflow(), false);
    /// assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_low.is_infinity(), false);
    /// assert_eq!(res_biguint_low.is_undefined(), false);
    /// assert_eq!(res_biguint_low.is_left_carry(), false);
    /// assert_eq!(res_biguint_low.is_right_carry(), false);
    /// ```
    pub fn widening_mul(&self, _rhs: &Self) -> (Self, Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn widening_mul_assign(&mut self, rhs: &Self) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the low-order (wrapping) bits of the result `self` * `rhs`,
    /// and returns the high-order (overflow) bits of the result.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.bit.
    /// 
    /// # Features
    /// - It performs "long multiplication",
    ///   and assigns the low-order part the result to `self` back,
    ///   and returns the high-order part of it.
    /// - If the return value which is the high-order part of the result is
    ///   not zero, the `OVERFLOW` flag of `self` will be set
    ///   though the result is free from overflow because the `OVERFLOW` flag
    ///   is of `self`, and not of the result of the multiplication.
    /// 
    /// # Counterpart Methods
    /// - If you also need to add a carry to the wide result, then you want to
    ///   use
    ///   [carrying_mul_assign()](struct@BigUInt#method.carrying_mul_assign)
    ///   instead.
    /// - The value of `self` after calculation matches what you’d get the
    ///   method [wrapping_mul()](struct@BigUInt#method.wrapping_mul)
    ///   so `self` == `self.wrapping_mul(rhs)`.
    /// - The method
    ///   [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint)
    ///   is a bit faster than this method `widening_mul_assign()`.
    ///   If `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint.widening_mul_assign(&b_biguint);
    /// println!("After a_biguint.widening_mul_assign(&b_biguint),\na_biguint = {}\nres_biguint_high = {}", a_biguint, res_biguint_high);
    /// assert_eq!(a_biguint.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::max();
    /// let b_biguint = U256::max();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let res_biguint_high = a_biguint.widening_mul_assign(&b_biguint);
    /// println!("After a_biguint.widening_mul_assign(&b_biguint),\na_biguint = {}\nres_biguint_high = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    /// assert_eq!(res_biguint_high.is_overflow(), false);
    /// assert_eq!(res_biguint_high.is_underflow(), false);
    /// assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    /// assert_eq!(res_biguint_high.is_infinity(), false);
    /// assert_eq!(res_biguint_high.is_undefined(), false);
    /// assert_eq!(res_biguint_high.is_left_carry(), false);
    /// assert_eq!(res_biguint_high.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn widening_mul_assign(&mut self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_mul(&self, _rhs: &Self) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the multiplication result `self` * `rhs` with wrapping
    /// (modular) multiplication.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of the return value
    ///   will be set.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint)
    /// is a bit faster than this method `wrapping_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.wrapping_mul(&b_biguint);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.wrapping_mul(&b_biguint);
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn wrapping_mul(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_mul_assign(&mut self, _rhs: &Self)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns a multiplication result `self` * `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_mul_assign_uint()](struct@BigUInt#method.wrapping_mul_assign_uint)
    /// is a bit faster than this method `wrapping_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_mul_assign_uint()](struct@BigUInt#method.wrapping_mul_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_mul_assign(&b_biguint);
    /// println!("After a_biguint.wrapping_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_mul_assign(&b_biguint);
    /// println!("After c_biguint.wrapping_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn wrapping_mul_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_mul(&self, rhs: &Self) -> (Self, bool)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the multiplication result `self` * `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns a tuple of the multiplication result `self` * `rhs` along
    /// with a boolean indicating whether an arithmetic overflow would
    /// occur. If an overflow would have occurred,
    /// then the wrapped (modular) value is returned.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication .
    /// - If overflow happens, the second element of the output tuple will
    ///   be true and the `OVERFLOW` flag of the return value will be set.
    /// - The second element of the output tuple reflects only
    ///   the current overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_mul_uint()](struct@BigUInt#method.overflowing_mul_uint)
    /// is a bit faster than this method `overflowing_mul()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_mul_uint()](struct@BigUInt#method.overflowing_mul_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let (res, overflow) = a_biguint.overflowing_mul(&b_biguint);
    /// println!("{} X {} = {}, {}", a_biguint, b_biguint, res, overflow);
    /// assert_eq!(overflow, false);
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let (res, overflow) = a_biguint.overflowing_mul(&b_biguint);
    /// println!("{} X {} = {}, {}", a_biguint, b_biguint, res, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_mul(&self, _rhs: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_mul_assign(&mut self, rhs: &Self) -> bool
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the multiplication result `self` * `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
    /// 
    /// # Arguments
    /// `rhs` is to be multiplied to `self`, and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - Wrapping (modular) multiplication.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - If overflow did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - The output reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_mul_assign_uint()](struct@BigUInt#method.overflowing_mul_assign_uint)
    /// is a bit faster than this method `overflowing_mul_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_mul_assign_uint()](struct@BigUInt#method.overflowing_mul_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
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
    /// let overflow = a_biguint.overflowing_mul_assign(&b_biguint);
    /// println!("After a_biguint.overflowing_mul_assign(&b_biguint), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
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
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    ///  
    /// let overflow = a_biguint.overflowing_mul_assign(&b_biguint);
    /// println!("After c_biguint.overflowing_mul_assign(&b_biguint), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_mul_assign(&mut self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** Division ***/

    // pub fn divide_fully(&self, rhs: &Self) -> (Self, Self)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of a quotient and a remainder.
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
    /// It returns a tuple of a quotient and a remainder.
    /// Both the quotient and the remainder are of `BigUInt` type.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function is the base function for all the methods *_div(),
    ///   *_div_assign(), *_rem(), and *_rem_assign().
    /// 
    /// # Counterpart Method
    /// The method
    /// [divide_fully_uint()](struct@BigUInt#method.divide_fully_uint)
    /// is a bit faster than this method `divide_fully()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [divide_fully_uint()](struct@BigUInt#method.divide_fully_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (quotient, remainder) = dividend.divide_fully(&divisor);
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (quotient, remainder) = dividend.divide_fully(&divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
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
    /// define_utypes_with!(u8);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let (quotient, remainder) = _dividend.divide_fully(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let (quotient, remainder) = _dividend.divide_fully(&_divisor);
    /// ```
    pub fn divide_fully(&self, _rhs: &Self) -> (Self, Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_div(&self, _rhs: &Self) -> Self
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
    /// It returns a quotient of `BigUInt` type,
    /// and the quotient would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_div_uint()](struct@BigUInt#method.wrapping_div_uint)
    /// is a bit faster than this method `wrapping_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_div_uint()](struct@BigUInt#method.wrapping_div_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.wrapping_div(&divisor);
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
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.wrapping_div(&divisor);
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
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.wrapping_div(&_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// let quotient = _dividend.wrapping_div(&_divisor);
    /// ```
    pub fn wrapping_div(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_div_assign(&mut self, _rhs: &Self)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back.
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
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_div_assign_uint()](struct@BigUInt#method.wrapping_div_assign_uint)
    /// is a bit faster than this method `wrapping_div_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_div_assign_uint()](struct@BigUInt#method.wrapping_div_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_div_assign(&divisor);
    /// println!("After a_biguint.wrapping_div_assign(&divisor),\na_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.wrapping_div_assign(&divisor);
    /// println!("After a_biguint.wrapping_div_assign(&divisor),\na_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_div_assign(&_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = UU32::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_div_assign(&_divisor);
    /// ```
    pub fn wrapping_div_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_div(&self, _rhs: &Self) -> (Self, bool)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of the quotient of `self` / `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// It returns a tuple of the quotient of `BigUInt` type as a result of
    /// `self` / `rhs` along with a boolean indicating whether an arithmetic
    /// overflow would occur. But the quotient would never overflow.
    /// So, the second element of the output tuple is always `false`.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - The quotient would never overflow.
    /// - The second element of the output tuple reflects only
    ///   the current overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_div_uint()](struct@BigUInt#method.overflowing_div_uint)
    /// is a bit faster than this method `overflowing_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_div_uint()](struct@BigUInt#method.overflowing_div_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let (quotient, overflow) = dividend.overflowing_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let (quotient, overflow) = dividend.overflowing_div(&divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// let (quotient, overflow) = _dividend.overflowing_div(&_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// let (quotient, overflow) = _dividend.overflowing_div(&_divisor);
    /// ```
    pub fn overflowing_div(&self, _rhs: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_div_assign(&mut self, _rhs: &Self) -> bool
    /// Divides `self` by `rhs`,
    /// and assigns the quotient of `self` / `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// It returns true if an arithmetic overflow would occur.
    /// But the quotient would never overflow.
    /// So, it always returns `false`.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - The quotient would never overflow.
    /// - The output will be `false` even if the `OVERFLOW` flag of `self`
    ///   was already set because of previous operation of `self`.
    /// - The output reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_div_assign_uint()](struct@BigUInt#method.overflowing_div_assign_uint)
    /// is a bit faster than this method `overflowing_div_assign_uint()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_div_assign_uint()](struct@BigUInt#method.overflowing_div_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_div_assign(&divisor);
    /// println!("After a_biguint.overflowing_div_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_div_assign(&divisor);
    /// println!("After a_biguint.overflowing_div_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_div_assign(&_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_div_assign(&_divisor);
    /// ```
    pub fn overflowing_div_assign(&mut self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_div(&self, _rhs: &Self, _modulus: &Self) -> Self
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
    /// [panic_free_modular_div_uint()](struct@BigUInt#method.panic_free_modular_div_uint)
    /// is a bit faster than this method `modular_div()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_div_uint()](struct@BigUInt#method.panic_free_modular_div_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for modulus >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
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
    /// # Example 3 for modulus >= 2 and dividend == multiple of modulus and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(10000_u16);
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
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
    /// # Example 4 for modulus >= 2 and divisor == 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), true);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for modulus >= 2 and divisor == multiple of modulus and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(200_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient, U256::max());
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), true);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for modulus >= 2 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for modulus >= 2 and divisor == 0 and dividend == multiple of modulus
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(30000_u16);
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for modulus >= 2 and divisor == multiple of modulus and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(200_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for modulus >= 2 and divisor == multiple of modulus and dividend == multiple of modulus
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(30000_u16);
    /// let divisor = U256::from_uint(200_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for modulus == 0 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::zero();
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for modulus == 1 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::one();
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulus == 0 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::zero();
    /// let modulus = U256::zero();
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 13 for modulus == 1 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::zero();
    /// let modulus = U256::one();
    /// let quotient = dividend.panic_free_modular_div(&divisor, &modulus);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulus);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// for modulus in [U256::zero(), U256::one()]
    /// {
    ///     let op1 = U256::zero();
    ///     let op2 = U256::zero();
    ///     let res = op1.panic_free_modular_div(&op2, &modulus);
    ///     println!("{} / {} = {} (mod {})", op1, op2, res, modulus);
    ///     assert_eq!(res.to_string(), "0");
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), true);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), true);
///         assert_eq!(quotient.is_left_carry(), false);
///         assert_eq!(quotient.is_right_carry(), false);
    ///     
    ///     for dividend in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
    ///     {
    ///         let rhs = U256::zero();
    ///         let res = dividend.panic_free_modular_div(&rhs, &modulus);
    ///         println!("{} / {} = {} (mod {})", dividend, rhs, res, modulus);
    ///         assert_eq!(res.to_string(), "0");
    ///         assert_eq!(res.is_overflow(), false);
    ///         assert_eq!(res.is_underflow(), false);
    ///         assert_eq!(res.is_divided_by_zero(), true);
    ///         assert_eq!(res.is_infinity(), true);
    ///         assert_eq!(res.is_undefined(), true);
    ///         assert_eq!(res.is_left_carry(), false);
    ///         assert_eq!(res.is_right_carry(), false);
    /// 
    ///         for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
    ///         {
    ///             let res = dividend.panic_free_modular_div(&divisor, &modulus);
    ///             println!("{} / {} = {} (mod {})", dividend, divisor, res, modulus);
    ///             assert_eq!(res.to_string(), "0");
    ///             assert_eq!(res.is_overflow(), false);
    ///             assert_eq!(res.is_underflow(), false);
    ///             assert_eq!(res.is_divided_by_zero(), false);
    ///             assert_eq!(res.is_infinity(), false);
    ///             assert_eq!(res.is_undefined(), true);
    ///             assert_eq!(res.is_left_carry(), false);
    ///             assert_eq!(res.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_div(&self, _rhs: &Self, _modulus: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_div_assign(&mut self, _rhs: &Self, _modulus: &Self)
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
    /// [panic_free_modular_div_assign_uint()](struct@BigUInt#method.panic_free_modular_div_assign_uint)
    /// is a bit faster than this method `panic_free_modular_div_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_div_assign_uint()](struct@BigUInt#method.panic_free_modular_div_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for Normal case for modulus >= 2 and self == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let modulus = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
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
    /// # Example 3 for Normal case for modulus >= 2 and self == multiple of modulus and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(10000_u16);
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
    /// # Example 4 for modulus >= 2 and self != 0 and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for modulus >= 2 and self != 0 and divisor == multiple of modulus
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for modulus >= 2 and self == 0 and divisor == 0
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
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for modulus >= 2 and self == 0 and divisor == multiple of modulus
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
    /// let divisor = U256::from_uint(200_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for modulus >= 2 and self == multiple of modulus and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(30000_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = U256::zero();
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for modulus >= 2 and self == multiple of modulus and divisor == multiple of modulus
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_uint(30000_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = U256::from_uint(200_u8);
    /// let modulus = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for modulus == 0 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::zero();
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for modulus == 1 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = U256::from_uint(128_u8);
    /// let modulus = U256::one();
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulus == 0 and divisor == 0 and dividend == 0
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
    /// let divisor = U256::zero();
    /// let modulus = U256::zero();
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 13 for modulus == 1 and divisor == 0 and dividend == 0
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
    /// let divisor = U256::zero();
    /// let modulus = U256::one();
    /// a_biguint.panic_free_modular_div_assign(&divisor, &modulus);
    /// println!("After a_biguint.panic_free_modular_div_assign({}, {}), a_biguint = {}", divisor, modulus, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// for modulus in [U256::zero(), U256::one()]
    /// {
    ///     let mut dividend = U256::zero();
    ///     println!("Originally, op1 = {}", dividend);
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), false);
    ///     assert_eq!(dividend.is_left_carry(), false);
    ///     assert_eq!(dividend.is_right_carry(), false);
    /// 
    ///     let divisor = U256::zero();
    ///     dividend.panic_free_modular_div_assign(&divisor, &modulus);
    ///     println!("After a_biguint.panic_free_modular_div_assign({}, {}), op1 = {}", divisor, modulus, dividend);
    ///     assert_eq!(dividend.to_string(), "0");
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), true);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), true);
    ///     assert_eq!(dividend.is_left_carry(), false);
    ///     assert_eq!(dividend.is_right_carry(), false);
    ///     
    ///     for op in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
    ///     {
    ///         let mut dividend = op.clone();
    ///         println!("Originally, dividend = {}", dividend);
    ///         assert_eq!(dividend.is_overflow(), false);
    ///         assert_eq!(dividend.is_underflow(), false);
    ///         assert_eq!(dividend.is_infinity(), false);
    ///         assert_eq!(dividend.is_undefined(), false);
    ///         assert_eq!(dividend.is_divided_by_zero(), false);
    ///         assert_eq!(dividend.is_left_carry(), false);
    ///         assert_eq!(dividend.is_right_carry(), false);
    /// 
    ///         let divisor = U256::zero();
    ///         dividend.panic_free_modular_div_assign(&divisor, &modulus);
    ///         println!("After op1.panic_free_modular_div_assign({}, {}), dividend = {}", divisor, modulus, dividend);
    ///         assert_eq!(dividend.to_string(), "0");
    ///         assert_eq!(dividend.is_overflow(), false);
    ///         assert_eq!(dividend.is_underflow(), false);
    ///         assert_eq!(dividend.is_divided_by_zero(), true);
    ///         assert_eq!(dividend.is_infinity(), true);
    ///         assert_eq!(dividend.is_undefined(), true);
    ///         assert_eq!(dividend.is_left_carry(), false);
    ///         assert_eq!(dividend.is_right_carry(), false);
    /// 
    ///         for divisor in [U256::from_uint(3_u8), U256::from_uint(50_u8)]
    ///         {
    ///             let mut dividend = op.clone();
    ///             println!("Originally, dividend = {}", dividend);
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(dividend.is_left_carry(), false);
    ///             assert_eq!(dividend.is_right_carry(), false);
    /// 
    ///             dividend.panic_free_modular_div_assign(&divisor, &modulus);
    ///             println!("After dividend.panic_free_modular_div_assign({}, {}), dividend = {}", divisor, modulus, dividend);
    ///             assert_eq!(dividend.to_string(), "0");
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), true);
    ///             assert_eq!(dividend.is_left_carry(), false);
    ///             assert_eq!(dividend.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_div_assign(&mut self, _rhs: &Self, _modulus: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    } 

    // pub fn wrapping_rem(&self, _rhs: &Self) -> Self
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
    /// It returns a remainder of `BigUInt` type,
    /// and the remainder would never overflow. 
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint)
    /// is a bit faster than this method `wrapping_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_rem_uint()](struct@BigUInt#method.wrapping_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.wrapping_rem(&divisor);
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
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let remainder = dividend.wrapping_rem(&divisor);
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
    /// define_utypes_with!(u16);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let remainder = _dividend.wrapping_rem(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let remainder = _dividend.wrapping_rem(&_divisor);
    /// ```
    pub fn wrapping_rem(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_rem_assign(&mut self, _rhs: &Self)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back.
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
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function exists, so that all operations are accounted for
    ///   in the wrapping operations.
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// The method
    /// [wrapping_rem_assign_uint()](struct@BigUInt#method.wrapping_rem_assign_uint)
    /// is a bit faster than this method `wrapping_rem_assign()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [wrapping_rem_assign_uint()](struct@BigUInt#method.wrapping_rem_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// a_biguint.wrapping_rem_assign(&divisor);
    /// println!("After a_biguint.wrapping_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint.wrapping_rem_assign(&divisor);
    /// println!("After a_biguint.wrapping_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_rem_assign(&_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_rem_assign(&_divisor);
    /// ```
    pub fn wrapping_rem_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_rem(&self, _rhs: &Self) -> (Self, bool)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of the remainder of `self` / `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// It returns a tuple of the remainder of `BigUInt` type as a result of
    /// `self` % `rhs` along with a boolean indicating whether an arithmetic
    /// overflow would occur. But the remainder would never overflow.
    /// So, the second element of the output tuple is always `false`.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - The remainder would never overflow.
    /// - The second element of the output tuple reflects only
    ///   the current overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_rem_uint()](struct@BigUInt#method.overflowing_rem_uint)
    /// is a bit faster than this method `overflowing_rem()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_rem_uint()](struct@BigUInt#method.overflowing_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (remainder, overflow) = dividend.overflowing_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let (remainder, overflow) = dividend.overflowing_rem(&divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let (remainder, overflow) = _dividend.overflowing_rem(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// let (remainder, overflow) = _dividend.overflowing_rem(&_divisor);
    /// ```
    pub fn overflowing_rem(&self, _rhs: &Self) -> (Self, bool)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_rem_assign(&mut self, _rhs: &Self) -> bool
    /// Divides `self` by `rhs`,
    /// and assigns the remainder of `self` / `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// It returns true if an arithmetic overflow would occur.
    /// But the remainder would never overflow.
    /// So, it always returns `false`.
    /// 
    /// # Features
    /// - Wrapped division on `BigUInt` types is just normal division.
    /// - The remainder would never overflow.
    /// - The output will be `false` even if the `OVERFLOW` flag of `self`
    ///   was already set because of previous operation of `self`.
    /// - The output reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// The method
    /// [overflowing_rem_assign_uint()](struct@BigUInt#method.overflowing_rem_assign_uint)
    /// is a bit faster than this method `overflowing_rem_assign_uint()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [overflowing_rem_assign_uint()](struct@BigUInt#method.overflowing_rem_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
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
    /// let divisor = UU32::from_uint(87_u8);
    /// let overflow = a_biguint.overflowing_rem_assign(&divisor);
    /// println!("After a_biguint.overflowing_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(overflow, false);
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
    /// let divisor = UU32::from_uint(87_u8);
    /// let overflow = a_biguint.overflowing_rem_assign(&divisor);
    /// println!("After a_biguint.overflowing_rem_assign({}), a_biguint = {}", divisor, a_biguint);
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
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_rem_assign(&_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_rem_assign(&_divisor);
    /// ```
    pub fn overflowing_rem_assign(&mut self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}