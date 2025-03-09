// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]

use std::str::FromStr;
use std::convert::From;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::{ SmallUInt, NumberErr };

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_arithmetic.rs.
pub struct BigUInt<T, const N: usize>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn add(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility and because the function(s) of the trait should be fixed,
/// trait Add2, Add3, and Add4 are provided and the developer
/// implements none or only one of traits Add, Add2, Add3, and Add4.
/// ```
/// pub trait Add<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Add, the trait PartialEq makes the operators `==` and `!=` take
/// not `&Self` but `Self` as its first operand and not `&Rhs` (or `&Self`) but
/// `Rhs` (or `Self`) as its second operand but makes the functions eq() and
/// ne() take not `self` but `&self` as its first argument and not `Rhs` but
/// `&Rhs` as its second argument. So, I think the third suggestion is possible.
/// The prototype of trait PartialEq is as follows:
/// 
/// ```
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
impl<T, const N: usize> Add for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn add(self, rhs: Self) -> Self
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns an addition result `self` + `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let one_biguint = U512::one();
    /// let res = a_biguint.clone() + one_biguint.clone();
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let two_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.clone() + two_biguint.clone();
    /// println!("{} + {} = {}", a_biguint, two_biguint, res);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let three_biguint = U512::from_uint(3_u8);
    /// let res = a_biguint.clone() + three_biguint.clone();
    /// println!("{} + {} = {}", a_biguint, three_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Cannot-compile Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let one_biguint = U512::one();
    /// let _res = a_biguint + one_biguint;
    /// // println!("{} + {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and one_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let two_biguint = U512::from_uint(2_u8);
    /// let _res = a_biguint + two_biguint;
    /// // println!("{} + {} = {}", a_biguint, two_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and two_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u8;
    /// let three_biguint = U512::from_uint(3_u8);
    /// let _res = a_biguint + three_biguint;
    /// // println!("{} + {} = {}", a_biguint, three_biguint, _res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint and three_biguint.
    /// ```
    fn add(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `+=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
///   or
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait AddAssign2 is provided and the developer
/// implements none or only one of traits AddAssign and AddAssign2.
/// 
/// ```text
/// pub trait AddAssign<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait AddAssign2<Rhs = Self> {
///     // Required method
///     fn add_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait AddAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> AddAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn add_assign(&mut self, rhs: Self)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assign the addition result `self` + `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
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
    /// a_biguint += one_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint += two_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint += three_biguint.clone();
    /// println!("After a_biguint += {},\ta_biguint = {}", three_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let one_biguint = U512::one();
    /// a_biguint += one_biguint;
    /// println!("After a_biguint += {}, a_biguint = {}", one_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) one_biguint.
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint += two_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", two_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) two_biguint.
    /// 
    /// let mut a_biguint = U512::max() - 1_u32;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint += three_biguint.clone();
    /// println!("After a_biguint += {}, a_biguint = {}", three_biguint, a_biguint);
    /// // The operator '+=' swallowed (took the ownership of) three_biguint.
    /// ```
    fn add_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after addition
/// operation. In order to prevent this, the operands should be cloned or
/// copied before addition operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn sub(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Sub2, Sub3, and Sub4 are provided and the developer
/// implements none or only one of traits Sub, Sub2, Sub3, and Sub4.
/// 
/// ```text
/// pub trait Sub<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Sub4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn sub(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Sub, the trait PartialEq makes the operators `==` and `!=` take
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
impl<T, const N: usize> Sub for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn sub(self, rhs: Self) -> Self
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a subtraction result `self` - `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be subtracted from `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let one_biguint = U512::one();
    /// let res = a_biguint.clone() - one_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, one_biguint, res);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let two_biguint = U512::from_uint(2_u8);
    /// let res = a_biguint.clone() - two_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, two_biguint, res);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let three_biguint = U512::from_uint(3_u8);
    /// let res = a_biguint.clone() - three_biguint.clone();
    /// println!("{} - {} = {}", a_biguint, three_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::one();
    /// let one_biguint = U512::one();
    /// let _res = a_biguint - one_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and one_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let two_biguint = U512::from_uint(2_u8);
    /// let _res = a_biguint - two_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and two_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let three_biguint = U512::from_uint(3_u8);
    /// let _res = a_biguint - three_biguint;
    /// println!("{} - {} = {}", a_biguint, one_biguint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint and three_biguint.
    /// ```
    #[inline]
    fn sub(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `-=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after subtraction
/// operation. In order to prevent this, the operands should be cloned or
/// copied before subtraction operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
///   or
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait SubAssign2 is provided and the developer
/// implements none or only one of traits SubAssign and SubAssign2.
/// ```
/// pub trait SubAssign<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait SubAssign2<Rhs = Self> {
///     // Required method
///     fn sub_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait SubAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> SubAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn sub_assign(&mut self, rhs: Self)
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let one_biguint = U512::one();
    /// a_biguint -= one_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
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
    /// define_utypes_with!(u16);
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
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint -= two_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
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
    /// define_utypes_with!(u16);
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
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint -= three_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    ///
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let one_biguint = U512::one();
    /// a_biguint -= one_biguint;
    /// println!("After a_biguint -= {}, a_biguint = {}", one_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) one_biguint.
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let two_biguint = U512::from_uint(2_u8);
    /// a_biguint -= two_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", two_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) two_biguint.
    /// 
    /// let mut a_biguint = U512::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let three_biguint = U512::from_uint(3_u8);
    /// a_biguint -= three_biguint.clone();
    /// println!("After a_biguint -= {}, a_biguint = {}", three_biguint, a_biguint);
    /// // The operator '-=' swallowed (took the ownership of) three_biguint.
    /// ```
    #[inline]
    fn sub_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after multiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before multiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn mul(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Mul2, Mul3, and Mul4 are provided and the developer
/// implements none or only one of traits Mul, Mul2, Mul3, and Mul4.
/// ```
/// pub trait Mul<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Mul4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn mul(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Mul, the trait PartialEq makes the operators `==` and `!=` take
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
impl<T, const N: usize> Mul for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn mul(self, rhs: Self) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a multiplication result `self` * `rhs`.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.clone() * b_biguint.clone();
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let res = a_biguint.clone() * b_biguint.clone();
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
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let _res = a_biguint * b_biguint;
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_biguint = U256::from_uint(248_u8);
    /// let _res = a_biguint * b_biguint;
    /// println!("{} X {} = {}", a_biguint, b_biguint, res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn mul(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `*=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after mutltiplication
/// operation. In order to prevent this, the operands should be cloned or
/// copied before mutltiplication operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
///   or
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait MulAssign2 is provided and the developer
/// implements none or only one of traits MulAssign and MulAssign2.
/// ```
/// pub trait MulAssign<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait MulAssign2<Rhs = Self> {
///     // Required method
///     fn mul_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait MulAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> MulAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn mul_assign(&mut self, rhs: Self)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns a multiplication result `self` * `rhs` to `self` back.
    /// 
    /// # Arguments
    /// `rhs` is to be added to `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// a_biguint *= b_biguint.clone();
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
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
    /// a_biguint *= b_biguint.clone();
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let b_biguint = U256::from_uint(248_u8);
    /// a_biguint *= b_biguint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_biguint, a_biguint);
    /// // The operator '*' swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline] 
    fn mul_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn div(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Div2, Div3, and Div4 are provided and the developer
/// implements none or only one of traits Div, Div2, Div3, and Div4.
/// 
/// ```
/// pub trait Div<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Div4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn div(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Div, the trait PartialEq makes the operators `==` and `!=` take
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
impl<T, const N: usize> Div for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn div(self, rhs: Self) -> Self
    /// Divides `self` by `rhs`, and returns the quotient.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let quotient = dividend.clone() / divisor.clone();
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
    /// let quotient = dividend.clone() / divisor.clone();
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
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // let quotient = _dividend.clone() / _divisor.clone();
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // let quotient = _dividend.clone() / _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = U256::from_uint(87_u8);
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend and divisor.
    /// 
    /// let dividend = U256::zero();
    /// let divisor = U256::from_uint(87_u8);
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend and divisor.
    /// ```
    #[inline]
    fn div(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `/=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
///   or
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait DivAssign2 is provided and the developer
/// implements none or only one of traits DivAssign and DivAssign2.
/// 
/// ```
/// pub trait DivAssign<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait DivAssign2<Rhs = Self> {
///     // Required method
///     fn div_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait DivAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> DivAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn div_assign(&mut self, rhs: Self)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor.clone();
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
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
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor.clone();
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
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
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // _a_biguint /= _divisor.clone();
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // _a_biguint /= _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator '/=' swallowed (took the ownership of) divisor.
    /// 
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = UU32::from_uint(87_u8);
    /// a_biguint /= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint /= {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator '/=' swallowed (took the ownership of) divisor.
    /// ```
    #[inline]
    fn div_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn rem(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Rem2, Rem3, and Rem4 are provided and the developer
/// implements none or only one of traits Rem, Rem2, Rem3, and Rem4.
/// 
/// ```
/// pub trait Rem<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Rem4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn rem(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Rem, the trait PartialEq makes the operators `==` and `!=` take
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
impl<T, const N: usize> Rem for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn rem(self, rhs: Self) -> Self
    /// Divides `self` by `rhs`, and returns the remainder.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
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
    /// let remainder = dividend.clone() % divisor.clone();
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
    /// let remainder = dividend.clone() % divisor.clone();
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
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // let remainder = _dividend.wrapping_rem(&_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = UU32::zero();
    /// // It will panic!
    /// // let remainder = _dividend.wrapping_rem(&_divisor);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = UU32::from_uint(87_u8);
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend and divisor.
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = UU32::from_uint(87_u8);
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend and divisor.
    /// ```
    #[inline]
    fn rem(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `%=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after division
/// operation. In order to prevent this, the operands should be cloned or
/// copied before division operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
///   or
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait RemAssign2 is provided and the developer
/// implements none or only one of traits RemAssign and RemAssign2.
/// 
/// ```
/// pub trait RemAssign<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait RemAssign2<Rhs = Self> {
///     // Required method
///     fn rem_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait RemAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> RemAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn rem_assign(&mut self, rhs: Self)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back.
    /// 
    /// # Arguments
    /// `rhs` divides `self`, and is of `Self` type.
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
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// a_biguint %= divisor.clone();
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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
    /// a_biguint %= divisor.clone();
    /// println!("After a_biguint %= {}, a_biguint = {}", divisor, a_biguint);
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
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // _a_biguint %= _divisor.clone();
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = U256::zero();
    /// // It will panic!
    /// // _a_biguint %= _divisor.clone();
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator %= swallowed (took the ownership of) divisor.
    /// 
    /// let mut a_biguint = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let divisor = U256::from_uint(87_u8);
    /// a_biguint %= divisor;
    /// // It cannot be compiled!
    /// println!("After a_biguint =/ {}, a_biguint = {}", divisor, a_biguint);
    /// // The operator %= swallowed (took the ownership of) divisor.
    /// ```
    #[inline]
    fn rem_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `&` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after AND (&)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before AND (&) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitand(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitand(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitAnd2, BitAnd3, and BitAnd4 are provided and
/// the developer implements none or only one of traits BitAnd, BitAnd2,
/// BitAnd3, and BitAnd4.
/// 
/// ```text
/// pub trait BitAnd<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitAnd4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitand(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitAnd, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitAnd for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitand(self, rhs: Self) -> Self
    /// Performs the bitwise AND (&) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that AND (&) operation is
    ///   performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise AND operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint, a_biguint);
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() & b_biguint.clone();
    /// 
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint & b_biguint;
    /// // It cannot be compiled!
    /// println!("{} & {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator & swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitand(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `&=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after AND (&=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before AND (&=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: Rhs);
///   or
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitAndAssign2 is provided and the developer
/// implements none or only one of traits BitAndAssign and BitAndAssign2.
/// 
/// ```
/// pub trait BitAndAssign<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait ShrAssign2<Rhs = Self> {
///     // Required method
///     fn bitand_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitAndAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitAndAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitand_assign(&mut self, rhs: Self)
    /// Performs the bitwise AND (&) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_00000000_00000000_00000000_11111111_00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000_10001111_00001111_10000011_11110000_00000000_00000000_00000000_00000000_10100010_10001100_00000000_10000011_00000000_00111111_10000000_00000000_00000000");
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
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint &= b_biguint.clone();
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint &= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint &= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator &= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitand_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `|` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after OR (|)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before OR (|) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitor(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitor(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitOr2, BitOr3, and BitOr4 are provided and
/// the developer implements none or only one of traits BitOr2, BitOr3,
/// and BitOr4.
/// 
/// ```
/// pub trait BitOr<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitOr4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitOr, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitOr for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitor(self, rhs: Self) -> Self
    /// Performs the bitwise OR (|) operation, and then returns the result.
    /// 
    /// # Arguments
    /// - `rhs` is the reference of another object that OR (|) operation is
    ///   performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() | b_biguint.clone();
    /// 
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint | b_biguint;
    /// // It cannot be compiled!
    /// println!("{} | {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator | swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitor(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `|=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after Or (|=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before Or (|=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: Rhs);
///   or
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitOrAssign2 is provided and the developer
/// implements none or only one of traits BitOrAssign and BitOrAssign2.
/// 
/// ```
/// pub trait BitOrAssign<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait BitOrAssign2<Rhs = Self> {
///     // Required method
///     fn bitor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitOrAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitOrAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitor_assign(&mut self, rhs: Self)
    /// Performs the bitwise OR (|) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the OR (|) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000_00000000_10110011_11111111_11111111_11111111_11111111_00111111_10000000_11111111_00000000_10111011_11001111_11111111_11111111_11110000_11111111_11111111_11111111_00000000");
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
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint |= b_biguint.clone();
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint |= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint |= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator |= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitor_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `^` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after XOR (^)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before XOR (^) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitxor(&self, rhs: Rhs) -> Self::Output;
///   or
///     fn bitxor(self, rhs: &Rhs) -> Self::Output;
///   or
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitXor2, BitXor3, and BitXor4 are provided and
/// the developer implements none or only one of traits BitXor, BitXor2,
/// BitXor3, and BitXor4.
/// 
/// ```
/// pub trait BitXor<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor2<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait BitXor4<Rhs = Self> {
///     type Output;
///     // Required method
///     fn bitxor(&self, rhs: &Rhs) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait BitXor, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitXor for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn bitxor(self, rhs: Self) -> Self
    /// Performs the bitwise XOR (^) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `Self` type.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise XOR (^) operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.clone() ^ b_biguint.clone();
    /// 
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(c_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::max();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let b_biguint = U256::zero();
    /// let _c_biguint = a_biguint ^ b_biguint;
    /// // It cannot be compiled!
    /// println!("{} ^ {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _c_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^ swallowed (took the ownership of) a_biguint and b_biguint.
    /// ```
    #[inline]
    fn bitxor(self, _rhs: Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `^=` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after XOR (^=)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before XOR (^=) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: Rhs);
///   or
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait BitXorAssign2 is provided and the developer
/// implements none or only one of traits BitXorAssign and BitXorAssign2.
/// 
/// ```
/// pub trait BitXorAssign<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: Rhs);
/// }
/// 
/// pub trait BitXorAssign2<Rhs = Self> {
///     // Required method
///     fn bitxor_assign(&mut self, rhs: &Rhs);
/// }
/// ```
/// 
/// Unlike trait BitXorAssign, the trait PartialEq makes the operators
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
impl<T, const N: usize> BitXorAssign for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString 
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn bitxor_assign(&mut self, rhs: Self)
    /// Performs the bitwise XOR (^) operation,
    /// and then assigns the result to `self` back.
    /// 
    /// # Arguments
    /// - `rhs` is the reference to another object that the AND (&) operation
    ///   is performed with.
    /// - `rhs` is of `&Self` type.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_11111111_00000000_11111111_00000000_11111111_11111111_00000000_11111111_11111111_00000000_00000000_10110011_01110000_11110000_01111100_00001111_00111111_10000000_11111111_00000000_00011001_01000011_11111111_01111100_11110000_11000000_01111111_11111111_00000000");
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
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::max();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
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
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint ^= b_biguint.clone();
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), "10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::from_str_radix("11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000_10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000", 2).unwrap();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::max();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// 
    /// let mut a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// let b_biguint = U256::zero();
    /// a_biguint ^= b_biguint;
    /// // It cannot be compiled!
    /// println!("After a_biguint ^= {}, a_biguint = {}.", b_biguint, a_biguint.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ^= swallowed (took the ownership of) b_biguint.
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, _rhs: Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// I would like to suggest the modification of Rust grammar because the
/// operator `!` swallows (takes the ownership of) two operands which are
/// left-hand side operand `self` and right-hand side operand `rhs` so that
/// the two operands `self` and `rhs` cannot be used again after NOT (!)
/// operation. In order to prevent this, the operands should be cloned or
/// copied before NOT (!) operation. This adds the unnecessary overhead.
/// The heavier the operand object is, the more the overhead is.
/// 
/// So, I would like to suggest one of the following three as follows:
/// 
/// # First suggestion
/// Changing the types of the parameters as follows:
/// 
/// ```text
/// pub trait Not {
///     type Output;
/// 
///     // Required method
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// # Second suggestion
/// If the first suggestion is impossible because of backward compatibility,
/// grammar allows the developer to choose the types of parameters but make
/// only one function.
/// 
/// ```text
/// pub trait Not {
///     type Output;
///     // Required method
///     fn not(self) -> Self::Output;
///   or
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// # Third suggestion
/// If the first and second suggestions are impossible because of backward
/// compatibility, trait Not2 is provided and
/// the developer implements none or only one of traits Not, Not2.
/// 
/// ```
/// pub trait Not {
///     type Output;
///     // Required method
///     fn not(self) -> Self::Output;
/// }
/// 
/// pub trait Not2 {
///     type Output;
///     // Required method
///     fn not(&self) -> Self::Output;
/// }
/// ```
/// 
/// Unlike trait Not, the trait PartialEq makes the operators
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
impl<T, const N: usize> Not for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    // fn not(self) -> Self
    /// Performs the bitwise NOT (!) operation, and then returns the result.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise NOT operation.
    ///
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1010101_00110011_00001111_00000000_11111111_00000000_00000000_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_01001100_01110000_11110000_01111100_00001111_11000000_01111111_00000000_11111111");
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = !a_biguint.clone();
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_str_radix("10101010_11001100_11110000_11111111_00000000_11111111_11111111_00000000_00000000_11111111_11111111_11111111_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_10110011_10001111_00001111_10000011_11110000_00111111_10000000_11111111_00000000", 2).unwrap();
    /// let _res = !a_biguint;
    /// // It cannot be compiled!
    /// println!("! {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), _res.to_string_with_radix_and_stride(2, 8).unwrap());
    /// // The operator ! swallowed (took the ownership of) a_biguint.
    /// ```
    #[inline]
    fn not(self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



/// Trait for comparisons using the equality operator.
/// - Implementing this trait for types provides the == and != operators
///   for those types.
/// - x.eq(y) can also be written x == y, and x.ne(y) can be written x != y.
/// - This trait allows for comparisons using the equality operator,
///   for types that do not have a full equivalence relation.
/// For more information, [read more](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#).
impl<T, const N: usize> PartialEq for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn eq(&self, other: &Self) -> bool
    /// Compare `self` with `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `&Self` type.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    fn eq(&self, _other: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}



impl<T, const N: usize> PartialOrd for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    /// # __self < other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint < b_biguint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// # __self <= other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than or
    /// equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is less than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint <= b_biguint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// # __self > other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is greater
    /// than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint > b_biguint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// # __self >= other -> bool__
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, some methods may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is greater than or equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint >= b_biguint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// # __self == other -> bool__
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint == b_biguint;
    /// if res
    ///     { println!("{} = {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// # __self != other -> bool__
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and is of `Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns `true` if `self` is not equal to `other`.
    /// Otherwise, it returns `false`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_string(num_str).unwrap();
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let num_str = "69743176821145534028236692093846345739169743176821145534028236692093846345739";
    /// let a_biguint = UU32::from_string(num_str).unwrap();
    /// let b_biguint = UU32::from_uint(100_u8);
    /// let res = a_biguint != b_biguint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_biguint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_biguint); }
    /// assert_eq!(res, true);
    /// ```
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering>
    {
        unimplemented!(); // Dummy code for documentation
    }
}



impl<T, const N: usize> From<[T; N]> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn from(val: [T; N]) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from an array of type `T`
    /// with `N` elements.
    /// 
    /// # Argument
    /// `val` is a primitive unsigned integer
    /// which will be converted into `BigUInt`.
    /// 
    /// # Features
    /// The method Self::from() is the same of the method Self::from_uint().
    /// 
    /// # Output
    /// A new `BigUInt<T, N>`-type object
    /// 
    /// # Example
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from([1, 2, 3, 4, 5, 6, 7, 8]);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    /// ```
    fn from(_val: [T; N]) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}



impl<T, const N: usize> FromStr for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Err = NumberErr;

    // fn from_str(s: &str) -> Result<Self, Self::Err>
    /// Constructs a new `BigUInt<T, N>`-type object from a string with radix 10.
    /// 
    /// # Argument
    /// `s` is a text (string) slice, which `BigUInt<T, N>`-type object is
    /// constructed from the string slice `s`.
    /// 
    /// # Features
    /// The constructed object will be wrapped in `Ok(BigUInt<T, N>)` if it is
    /// successfully created.
    /// Otherwise, this method returns `Err(NumberErr::ParsingError)`.
    /// And, if you import (use) std::str::FromStr,
    /// you can automagically use `str::parse::<BigUInt>()` too.
    /// 
    /// # Output
    /// A new `BigUInt<T, N>`-type object constructed
    /// from a string with radix 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = U256::from_str("215679573381144830513811895868694400695694534256768036697775454289921");
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => {
    ///             println!("a_biguint = {}", a_biguint);
    ///             assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    ///         },
    ///     Err(e) => { println!("Error: {}", e); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = U256::from_str("@!#$%^&*()_+=-|-/?><`~");
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
    ///     Err(e) => {
    ///             println!("Error: {}", e);
    ///             assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///         }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = "215679573381144830513811895868694400695694534256768036697775454289921".parse::<U256>();
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => {
    ///             println!("a_biguint = {}", a_biguint);
    ///             assert_eq!(a_biguint.to_string(), "215679573381144830513811895868694400695694534256768036697775454289921");
    ///         },
    ///     Err(e) => { println!("Error: {}", e); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::NumberErr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint_wrapped = "@!#$%^&*()_+=-|-/?><`~".parse::<U256>();
    /// match a_biguint_wrapped
    /// {
    ///     Ok(a_biguint) => { println!("a_biguint = {}", a_biguint); },
    ///     Err(e) => {
    ///             println!("Error: {}", e);
    ///             assert_eq!(e, NumberErr::NotAlphaNumeric);
    ///         }
    /// }
    /// ```
    #[inline]
    fn from_str(_s: &str) -> Result<Self, Self::Err>
    {
        unimplemented!(); // Dummy code for documentation
    }
}


/*
impl<T, const N: usize> Add<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self
    {
        self.wrapping_add(rhs)
    }
}

impl<T, const N: usize> AddAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Adds and assign the result to it.
    /// 
    #[inline]
    fn add_assign(&mut self, rhs: &Self)
    {
        self.wrapping_add_assign(rhs);
    }
}

impl<T, const N: usize> Sub<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self
    {
        self.wrapping_sub(rhs)
    }
}

impl<T, const N: usize> SubAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Subtracts and assign the result to it.
    #[inline]
    fn sub_assign(&mut self, rhs: &Self)
    {
        self.wrapping_sub_assign(rhs);
    }
}

impl<T, const N: usize> Mul<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self
    {
        self.wrapping_mul(rhs)
    }
}

impl<T, const N: usize> MulAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Multiplies and assign the result to it.
    #[inline] 
    fn mul_assign(&mut self, rhs: &Self)
    {
        self.wrapping_mul_assign(rhs);
    }
}

impl<T, const N: usize> Div<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: &Self) -> Self
    {
        self.wrapping_div(rhs)
    }
}

impl<T, const N: usize> DivAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn div_assign(&mut self, rhs: &Self)
    {
        self.wrapping_div_assign(rhs);
    }
}

impl<T, const N: usize> Rem<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: &Self) -> Self
    {
        self.wrapping_rem(rhs)
    }
}

impl<T, const N: usize> RemAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    #[inline]
    fn rem_assign(&mut self, rhs: &Self)
    {
        self.wrapping_rem_assign(rhs);
    }
}

impl<T, const N: usize> BitAnd<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    //type Output = <Self as BitAnd<Self>>::Output;
    //type <BigUInt<T, N> as std::ops::BitAnd<&BigUInt<T, N>>>::Output = BigUInt<T, N>;

    /// Performs the & operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAnd.html#tymethod.bitand)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a & b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// let c = a & b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a & b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a & b);
    /// ```
    fn bitand(self, rhs: &Self) -> Self
    {
        self.and(rhs)
    }
}

impl<T, const N: usize> BitAndAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the &= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a &= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111000000000000110000000000001110001000000100011010101000000000111100000000000011000000000000111000100000010001101010100000000011110000000000001100000000000011100010000001000110101010000000001111000000000000110000000000001110001000000100011010101000000000", 2).unwrap());
    /// ```
    ///
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::zero();
    /// a &= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::zero());
    /// ```
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self)
    {
        self.and_assign(rhs);
    }
}

impl<T, const N: usize> BitOr<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    //type Output = <Self as BitOr<Self>>::Output;

    /// Performs the bitwise OR (|) operation.
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::HugeInteger;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a | b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, a | b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a | b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a | b = {}", c.to_string_with_radix(2));
    /// 
    /// assert_eq!(c, u256::max());
    /// ```
    #[inline]
    fn bitor(self, rhs: &Self) -> Self
    {
        self.or(rhs)
    }
}

impl<T, const N: usize> BitOrAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the bitwise OR (|) operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)
    /// 
    /// # Output
    /// It returns the result after applying the bitwise OR (|) operation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a |= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::from_str_radix("1111111100001111111111000011111111101110011101111111111101010101111111110000111111111100001111111110111001110111111111110101010111111111000011111111110000111111111011100111011111111111010101011111111100001111111111000011111111101110011101111111111101010101", 2).unwrap());
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// 
    /// define_utypes_with!(u128);
    /// 
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a |= b;
    /// 
    /// println!("a = {}", a.to_string_with_radix(2));
    /// 
    /// assert_eq!(a, u256::max());
    /// ```
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self)
    {
        self.or_assign(rhs);
    }
}

impl<T, const N: usize> BitXor<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    type Output = Self;
    // type Output = <Self as BitXor<Self>>::Output;

    /// Performs the ^ operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXor.html#tymethod.bitxor)
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, a ^ b);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// let c = a ^ b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// println!("b = {}", b.to_string_with_radix(2));
    /// println!("a ^ b = {}", c.to_string_with_radix(2));
    /// assert_eq!(c, !a);
    /// ```    
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self
    {
        self.xor(rhs)
    }
}

impl<T, const N: usize> BitXorAssign<&Self> for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /// Performs the ^= operation.
    /// [Read more](https://doc.rust-lang.org/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::from_str_radix("11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000", 2).unwrap();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("111100001111001111000011110001100110011001100101010101010101000011110000111100111100001111000110011001100110010101010101010100001111000011110011110000111100011001100110011001010101010101010000111100001111001111000011110001100110011001100101010101010101", 2).unwrap());
    /// ```
    /// You have to import (use) cryptocol::number::HugeInteger in order to use
    /// its method to_string_with_radix(). If you find headaching to remember
    /// what you should import, you can just import everything
    /// (cryptocol::number::*) as next example. It is not harmful.
    /// ```
    /// use cryptocol::number::*;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// let mut a = u256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    /// let b = u256::max();
    /// a ^= b;
    /// println!("a = {}", a.to_string_with_radix(2));
    /// assert_eq!(a, u256::from_str_radix("11111111000011111111000000110011110011000101010110101010000000001111111100001111111100000011001111001100010101011010101000000000111111110000111111110000001100111100110001010101101010100000000011111111000011111111000000110011110011000101010110101010", 2).unwrap());
    /// ```
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self)
    {
        self.xor_assign(rhs);
    }
}
*/