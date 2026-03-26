// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]

use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::SmallUInt;

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_arithmetic.rs.
pub struct BigUInt<T, const N: usize>
where T: SmallUInt
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}



macro_rules! shl_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shl(&self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shl(self, rhs: &Rhs) -> Self::Output;
        ///   or
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait Shl2, Shl3, and Shl4 are provided and the developer
        /// implements none or only one of traits Shl, Shl2, Shl3, and Shl4.
        /// 
        /// ```
        /// pub trait Shl<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl2<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl3<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(self, rhs: &Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shl4<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shl(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// Unlike trait Shl, the trait PartialEq makes the operators `==` and `!=` take
        /// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
        /// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
        /// ne() take not `self` but `&self` as its first argument and not `Rhs` but
        /// `&Rhs` as its second argument. So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> Shl<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            type Output = Self;

            // fn shl(self, rhs: $f) -> Self
            /// Shift left the field `number: [T;N]` to the left by `n`,
            /// and returns the result.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, and
            /// `usize`.
            /// 
            /// # Output
            /// It returns the left-shifted version of `self`, which is shifted
            /// to the left by `rhs` bits.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `<<` work as
            ///   `>>` for BigUInt.
            ///   So, `self` << -2 means `self` >> 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` << `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `rhs` is a negative integer, this operation may cause carry-right.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// - If `1` is pushed out to the right during the << operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 7 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 8 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 9 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let res = a_biguint.clone() << n;
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
            /// # Example 10 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 11 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 12 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 13 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 14 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 15 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 16 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 17 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 18 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let res = a_biguint.clone() << n;
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Compile-fail Examples
            /// ```compile_fail
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u8);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let _res = a_biguint << n;
            /// // It cannot be compiled!
            /// println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator << swallowed (took the ownership of) a_biguint.
            /// ```
            fn shl(self, _rhs: $f) -> Self
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}



macro_rules! shlassign_i_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShlAssign2 is provided and the developer
        /// implements none or only one of traits ShlAssign and ShlAssign2.
        /// 
        /// ```
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShlAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShlAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            // fn shl_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the left by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, and `isize`.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, and `isize`, and for Union data types
            ///   such as ShortUnion, IntUnion, LongUnion, LongerUnion, and
            ///   SizeUnion, the negative value of `rhs` makes the operator
            ///   `<<=` work as `>>=` for BigUInt.
            ///   So, `self` <<= -2 means `self` >>= 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` <<= `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `rhs` is a negative integer, this operation may cause carry-right.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// - If `1` is pushed out to the right during the << operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// 
            /// # Example 1 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_i8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 2 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_i16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_i32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 4 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_i64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_i128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_isize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 7 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -3_i8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 8 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -4_i16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -128_i32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 10 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -256_i64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 11 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -512_i128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 12 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -1024_isize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            fn shl_assign(&mut self, _rhs: $f)
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}



macro_rules! shlassign_u_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `<<=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-left
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-left operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShlAssign2 is provided and the developer
        /// implements none or only one of traits ShlAssign and ShlAssign2.
        /// 
        /// ```
        /// pub trait ShlAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShlAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shl_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShlAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShlAssign<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            // fn shl_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the left by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` left by,
            /// and can be any primitive integer such as `u8`, `u16`, `u32`,
            /// `u64`, `u128`, and `usize`.
            /// 
            /// # Left Carry
            /// 'A left-carry occurs' means that a bit `1` is pushed out
            /// during shift-left operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift left' means 'move left all bits'. So, if `10011010`
            ///   is shifted left by 2, it will be `01101000`, for example.
            /// - Unlike for primitive unsigned integer data types such as
            ///   `u8`, `u16`, `u32`, `u64`, `u128`, and `usize` and for Union
            ///   data types such as ShortUnion, IntUnion, LongUnion,
            ///   LongerUnion, and SizeUnion, the value of `rhs` greater than
            ///   or equal to `size_of::<T>() * N` pushes all the bits out so
            ///   that the return value will be zero.
            ///   For example, `self` <<= `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the left during the << operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_u8;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
             /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_u16;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_u32;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_u64;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_u128;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u16);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_usize;
            /// a_biguint <<= n;
            /// println!("After a_biguint <<= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            #[inline]
            fn shl_assign(&mut self, _rhs: $f)
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}



shl_for_BigUInt_impl! { i8 }
shl_for_BigUInt_impl! { i16 }
shl_for_BigUInt_impl! { i32 }
shl_for_BigUInt_impl! { i64 }
shl_for_BigUInt_impl! { i128 }
shl_for_BigUInt_impl! { isize }
shl_for_BigUInt_impl! { u8 }
shl_for_BigUInt_impl! { u16 }
shl_for_BigUInt_impl! { u32 }
shl_for_BigUInt_impl! { u64 }
shl_for_BigUInt_impl! { u128 }
shl_for_BigUInt_impl! { usize }
shlassign_i_for_BigUInt_impl! { i8 }
shlassign_i_for_BigUInt_impl! { i16 }
shlassign_i_for_BigUInt_impl! { i32 }
shlassign_i_for_BigUInt_impl! { i64 }
shlassign_i_for_BigUInt_impl! { i128 }
shlassign_i_for_BigUInt_impl! { isize }
shlassign_u_for_BigUInt_impl! { u8 }
shlassign_u_for_BigUInt_impl! { u16 }
shlassign_u_for_BigUInt_impl! { u32 }
shlassign_u_for_BigUInt_impl! { u64 }
shlassign_u_for_BigUInt_impl! { u128 }
shlassign_u_for_BigUInt_impl! { usize }



macro_rules! shr_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shr(&self, rhs: Rhs) -> Self::Output;
        ///   or
        ///     fn shr(self, rhs: &Rhs) -> Self::Output;
        ///   or
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait Shr2, Shr3, and Shr4 are provided and the developer
        /// implements none or only one of traits Shr, Shr2, Shr3, and Shr4.
        /// 
        /// ```text
        /// pub trait Shr<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr2<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr3<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(self, rhs: &Rhs) -> Self::Output;
        /// }
        /// 
        /// pub trait Shr4<Rhs = Self> {
        ///     type Output;
        ///     // Required method
        ///     fn shr(&self, rhs: &Rhs) -> Self::Output;
        /// }
        /// ```
        /// 
        /// Unlike trait Shr, the trait PartialEq makes the operators `==` and `!=` take
        /// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
        /// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
        /// ne() take not `self` but `&self` as its first argument and not `Rhs` but
        /// `&Rhs` as its second argument. So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> Shr<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            type Output = Self;

            // fn shr(self, rhs: $f) -> Self
            /// Shift right the field `number: [T;N]` to the right by `n`,
            /// and returns the result.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, and
            /// `usize`.
            /// 
            /// # Output
            /// It returns the right-shifted version of `self`,
            /// which is shifted to the right by `rhs` bits.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `>>` work as
            ///   `<<` for BigUInt.
            ///   So, `self` >> -2 means `self` << 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >> `size_of::<T>() * N` will return zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-right.
            /// - If `rhs` is a negative integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the right during the >> operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// - If `1` is pushed out to the left during the >> operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 3_u8;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = 4_u16;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 128_u32;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 7 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 3_i8;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 8 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = 4_i16;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 9 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = 128_i32;
            /// let res = a_biguint.clone() >> n;
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
            /// # Example 10 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 11 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 12 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), false);
            /// assert_eq!(res.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 13 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -3_i8;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 14 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -4_i16;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 15 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -128_i32;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
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
            /// # Example 16 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 17 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 18 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let res = a_biguint.clone() >> n;
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(res.to_string(), "0");
            /// assert_eq!(res.is_overflow(), false);
            /// assert_eq!(res.is_underflow(), false);
            /// assert_eq!(res.is_infinity(), false);
            /// assert_eq!(res.is_undefined(), false);
            /// assert_eq!(res.is_divided_by_zero(), false);
            /// assert_eq!(res.is_left_carry(), true);
            /// assert_eq!(res.is_right_carry(), false);
            /// ```
            /// 
            /// # Compile-fail Examples
            /// ```compile_fail
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u32);
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_u8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_u16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_u32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_u64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_u128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_usize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 3_i8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 4_i16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 128_i32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 256_i64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 512_i128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = 1024_isize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -3_i8;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
            /// let n = -4_i16;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// let n = -128_i32;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -256_i64;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -512_i128;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// 
            /// let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// let n = -1024_isize;
            /// let _res = a_biguint >> n;
            /// // It cannot be compiled!
            /// println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
            /// // The operator >> swallowed (took the ownership of) a_biguint.
            /// ```
            fn shr(self, _rhs: $f) -> Self
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}



macro_rules! shrassign_i_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShrAssign2 is provided and the developer
        /// implements none or only one of traits ShrAssign and ShrAssign2.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShrAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShrAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            // fn shr_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the right by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `i8`, `i16`, `i32`,
            /// `i64`, `i128`, and `isize`.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize`, and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the negative value of `rhs` makes the operator `>>=` work as
            ///   `<<=` for BigUInt.
            ///   So, `self` >>= -2 means `self` <<= 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >>= `size_of::<T>() * N`
            ///   will make `self` zero.
            /// - If `rhs` is a positive integer, this operation may cause carry-right.
            /// - If `rhs` is a negative integer, this operation may cause carry-left.
            /// - If `1` is pushed out to the right during the >> operation,
            ///   `RIGHT_CARRY` flag will be set and the method is_right_carry()
            ///   will return true.
            /// - If `1` is pushed out to the left during the >> operation,
            ///   `LEFT_CARRY` flag will be set and the method is_left_carry()
            ///   will return true.
            /// 
            /// # Example 1 for positive i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_i8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
             /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 2 for positive i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_i16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for positive i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_i32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 4 for positive i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_i64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for positive i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_i128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for positive isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_isize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 7 for negative i8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -3_i8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 8 for negative i16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -4_i16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 9 for negative i32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -128_i32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 10 for negative i64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -256_i64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 11 for negative i128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -512_i128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 12 for negative isize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = -1024_isize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), true);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            fn shr_assign(&mut self, _rhs: $f)
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}


macro_rules! shrassign_u_for_BigUInt_impl {
    ($f:ty) => {
        /// I would like to suggest the modification of Rust grammar because the
        /// operator `>>=` swallows (takes the ownership of) two operands which are
        /// left-hand side operand `self` and right-hand side operand `rhs` so that
        /// the two operands `self` and `rhs` cannot be used again after shift-right
        /// operation. In order to prevent this, the operands should be cloned or
        /// copied before shift-right operation. This adds the unnecessary overhead.
        /// The heavier the operand object is, the more the overhead is.
        /// 
        /// So, I would like to suggest one of the following three as follows:
        /// 
        /// # First suggestion
        /// Changing the types of the parameters as follows:
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Second suggestion
        /// If the first suggestion is impossible because of backward compatibility,
        /// grammar allows the developer to choose the types of parameters but make
        /// only one function.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        ///   or
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// # Third suggestion
        /// If the first and second suggestions are impossible because of backward
        /// compatibility, trait ShrAssign2 is provided and the developer
        /// implements none or only one of traits ShrAssign and ShrAssign2.
        /// 
        /// ```text
        /// pub trait ShrAssign<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: Rhs);
        /// }
        /// 
        /// pub trait ShrAssign2<Rhs = Self> {
        ///     // Required method
        ///     fn shr_assign(&mut self, rhs: &Rhs);
        /// }
        /// ```
        /// 
        /// Unlike trait ShrAssign, the trait PartialEq makes the operators
        /// `==` and `!=` take not `&Self` but `Self` as its first operand and not
        /// `&Rhs` (or `&Self`) but `Rhs` (or `Self`) as its second operand but makes
        /// the functions eq() and ne() take not `self` but `&self` as its first
        /// argument and not `Rhs` but `&Rhs` as its second argument.
        /// So, I think the third suggestion is possible.
        /// The prototype of trait PartialEq is as follows:
        /// 
        /// ```text
        /// pub trait PartialEq<Rhs = Self>
        /// where
        /// Rhs: ?Sized,
        /// {
        ///     // Required method
        ///     fn eq(&self, other: &Rhs) -> bool;
        /// 
        ///     // Provided method
        ///     fn ne(&self, other: &Rhs) -> bool { ... }
        /// }
        /// ```
        impl<T, const N: usize> ShrAssign<$f> for BigUInt<T, N>
        where T: SmallUInt
        {
            // fn shr_assign(&mut self, rhs: $f)
            /// shifts the field `number: [T;N]` to the right by `n`,
            /// and assigns the result to `self` back.
            /// 
            /// # Arguments
            /// `rhs` indicates how many bits this method shift `self` right by,
            /// and can be any primitive integer such as `u8`, `u16`, `u32`,
            /// `u64`, `u128`, and `usize`.
            /// 
            /// # Right Carry
            /// 'A right-carry occurs' means that a bit `1` is pushed out
            /// during shift-right operation.
            ///
            /// # Panics
            /// If `size_of::<T>() * N` <= `128`, this method may panic
            /// or its behavior may be undefined though it may not panic.
            /// 
            /// # Features
            /// - 'Shift right' means 'move right all bits'. So, if `10011010`
            ///   is shifted right by 2, it will be `00100110`, for example.
            ///   So, `self` >> -2 means `self` << 2 for example. 
            /// - Unlike for primitive integer data types such as `i8`, `i16`,
            ///   `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`,
            ///   `u128`, and `usize` and for Union data types such as
            ///   ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion,
            ///   the value of `rhs` greater than or equal to `size_of::<T>() * N` 
            ///   pushes all the bits out so that the return value will be zero.
            ///   For example, `self` >>= `size_of::<T>() * N`
            ///   will make `self` zero.
            /// - If underflow happens during the >>= operation, `UNDERFLOW` flag
            ///   will be set and the method is_underflow() will return true.
            /// - If overflow happens during the >>= operation, `OVERFLOW` flag
            ///   will be set and the method is_overflow() will return true.
            /// 
            /// # Example 1 for u8
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 3_u8;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 2 for u16
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 4_u16;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// ```
            /// 
            /// # Example 3 for u32
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 128_u32;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 4 for u64
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 256_u64;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 5 for u128
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 512_u128;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            /// 
            /// # Example 6 for usize
            /// ```
            /// use cryptocol::define_utypes_with;
            /// define_utypes_with!(u64);
            /// 
            /// let mut a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
            /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), false);
            /// 
            /// let n = 1024_usize;
            /// a_biguint >>= n;
            /// println!("After a_biguint >>= {}, a_biguint = {}.", n, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
            /// assert_eq!(a_biguint.to_string(), "0");
            /// assert_eq!(a_biguint.is_overflow(), false);
            /// assert_eq!(a_biguint.is_underflow(), false);
            /// assert_eq!(a_biguint.is_infinity(), false);
            /// assert_eq!(a_biguint.is_undefined(), false);
            /// assert_eq!(a_biguint.is_divided_by_zero(), false);
            /// assert_eq!(a_biguint.is_left_carry(), false);
            /// assert_eq!(a_biguint.is_right_carry(), true);
            /// ```
            #[inline]
            fn shr_assign(&mut self, _rhs: $f)
            {
                unimplemented!(); // Dummy code for documentation
            }
        }
    }
}



shr_for_BigUInt_impl! { i8 }
shr_for_BigUInt_impl! { i16 }
shr_for_BigUInt_impl! { i32 }
shr_for_BigUInt_impl! { i64 }
shr_for_BigUInt_impl! { i128 }
shr_for_BigUInt_impl! { isize }
shr_for_BigUInt_impl! { u8 }
shr_for_BigUInt_impl! { u16 }
shr_for_BigUInt_impl! { u32 }
shr_for_BigUInt_impl! { u64 }
shr_for_BigUInt_impl! { u128 }
shr_for_BigUInt_impl! { usize }
shrassign_i_for_BigUInt_impl! { i8 }
shrassign_i_for_BigUInt_impl! { i16 }
shrassign_i_for_BigUInt_impl! { i32 }
shrassign_i_for_BigUInt_impl! { i64 }
shrassign_i_for_BigUInt_impl! { i128 }
shrassign_i_for_BigUInt_impl! { isize }
shrassign_u_for_BigUInt_impl! { u8 }
shrassign_u_for_BigUInt_impl! { u16 }
shrassign_u_for_BigUInt_impl! { u32 }
shrassign_u_for_BigUInt_impl! { u64 }
shrassign_u_for_BigUInt_impl! { u128 }
shrassign_u_for_BigUInt_impl! { usize }