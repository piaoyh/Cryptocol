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
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

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
/// ```text
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
/// ```text
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
/// compatibility, trait Add2, Add3, and Add4 are provided and the developer
/// implements none or only one of traits Add, Add2, Add3, and Add4.
/// 
/// ```text
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
/// pub trait Add3<Rhs = Self> {
///     type Output;
///     // Required method
///     fn add(self, rhs: &Rhs) -> Self::Output;
/// }
/// 
/// pub trait Add4<Rhs = Self> {
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
impl<T, const N: usize> Add<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    type Output = Self;

    // fn add(self, rhs: T)-> Self
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns an addition result `self` + `rhs`.
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let one_uint = 1_u16;
    /// let res = a_biguint.clone() + one_uint;
    /// println!("{} + {} = {}", a_biguint, one_uint, res);
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let two_uint = 2_u16;
    /// let res = a_biguint.clone() + two_uint;
    /// println!("{} + {} = {}", a_biguint, two_uint, res);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let three_uint = 3_u16;
    /// let res = a_biguint.clone() + three_uint;
    /// println!("{} + {} = {}", a_biguint, three_uint, res);
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
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let one_uint = 1_u16;
    /// let res = a_biguint + one_uint;
    /// println!("{} + {} = {}", a_biguint, one_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let two_uint = 2_u16;
    /// let res = a_biguint + two_uint;
    /// println!("{} + {} = {}", a_biguint, two_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::max() - 1_u16;
    /// let three_uint = 3_u16;
    /// let res = a_biguint + three_uint;
    /// println!("{} + {} = {}", a_biguint, three_uint, res);
    /// // The operator '+' swallowed (took the ownership of) a_biguint.
    /// ```
    fn add(self, _rhs: T)-> Self
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
impl<T, const N: usize> AddAssign<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    // fn add_assign(&mut self, rhs: T)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` to `self` back.
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU64::max() - 1_u64;
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
    /// let one_uint = 1_u64;
    /// a_biguint += one_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", one_uint, a_biguint);
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU64::max() - 1_u64;
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
    /// let two_uint = 2_u64;
    /// a_biguint += two_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", two_uint, a_biguint);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::max() - 1_u64;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let three_uint = 3_u64;
    /// a_biguint += three_uint;
    /// println!("After a_biguint += {}, a_biguint = {}", three_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    fn add_assign(&mut self, _rhs: T)
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
impl<T, const N: usize> Sub<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    type Output = Self;

    // fn sub(self, rhs: T) -> Self
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a subtraction result `self` - `rhs`.
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let one_uint = 1_u8;
    /// let res = a_biguint.clone() - one_uint.clone();
    /// println!("{} - {} = {}", a_biguint, one_uint, res);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let two_uint = 2_u8;
    /// let res = a_biguint.clone() - two_uint.clone();
    /// println!("{} - {} = {}", a_biguint, two_uint, res);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let three_uint = 3_u8;
    /// let res = a_biguint.clone() - three_uint.clone();
    /// println!("{} - {} = {}", a_biguint, three_uint, res);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let one_uint = 1_8;
    /// let _res = a_biguint - one_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let two_uint = 2_u8;
    /// let _res = a_biguint - two_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U512::one();
    /// let three_uint = 3_u8;
    /// let _res = a_biguint - three_uint;
    /// println!("{} - {} = {}", a_biguint, one_uint, _res);
    /// // The operator '-' swallowed (took the ownership of) a_biguint.
    /// ```
    fn sub(self, _rhs: T) -> Self
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
/// 
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
impl<T, const N: usize> SubAssign<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    // fn sub_assign(&mut self, rhs: T)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the subtraction result `self` - `rhs` to `self` back.
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
    /// define_utypes_with!(u32);
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
    /// let one_uint = 1_u32;
    /// a_biguint -= one_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", one_uint, a_biguint);
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
    /// define_utypes_with!(u32);
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
    /// let two_uint = 2_u32;
    /// a_biguint -= two_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", two_uint, a_biguint);
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
    /// define_utypes_with!(u32);
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
    /// let three_uint = 3_u32;
    /// a_biguint -= three_uint;
    /// println!("After a_biguint -= {}, a_biguint = {}", three_uint, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    fn sub_assign(&mut self, _rhs: T)
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
impl<T, const N: usize> Mul<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    type Output = Self;

    // fn mul(self, rhs: T) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a multiplication result `self` * `rhs`.
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u128;
    /// let res = a_biguint.clone() * b_uint;
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let res = a_biguint.clone() * b_uint;
    /// println!("{} X {} = {}", a_biguint, b_uint, res);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u128;
    /// let _res = a_biguint * b_uint;
    /// println!("{} X {} = {}", a_biguint, b_uint, _res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint.
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let _res = a_biguint * b_uint;
    /// println!("{} X {} = {}", b_biguint, b_uint, _res);
    /// // The operator '*' swallowed (took the ownership of) a_biguint.
    /// ```
    #[inline]
    fn mul(self, _rhs: T) -> Self
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
/// 
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
impl<T, const N: usize> MulAssign<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    // fn mul_assign(&mut self, rhs: T)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns a multiplication result `self` * `rhs` to `self` back.
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
    /// define_utypes_with!(u16);
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
    /// a_biguint *= b_uint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_uint = 248_u16;
    /// a_biguint *= b_uint;
    /// println!("After a_biguint *= {}, a_biguint = {}", b_uint, a_biguint);
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
    fn mul_assign(&mut self, _rhs: T)
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
impl<T, const N: usize> Div<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    type Output = Self;

    // fn div(self, rhs: T) -> Self
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
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u64;
    /// let quotient = dividend.clone() / divisor;
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
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u64;
    /// let quotient = dividend.clone() / divisor;
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
    /// define_utypes_with!(u64);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u64;
    /// // It will panic!
    /// let quotient = _dividend.clone() / _divisor;
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u64;
    /// // It will panic!
    /// let quotient = _dividend.clone() / _divisor;
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u64;
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend.
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u64;
    /// let _quotient = dividend / divisor;
    /// // It cannot be compiled!
    /// println!("{} / {} = {}", dividend, divisor, _quotient);
    /// // The operator '/' swallowed (took the ownership of) dividend.
    /// ```
    #[inline]
    fn div(self, _rhs: T) -> Self
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
impl<T, const N: usize> DivAssign<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    // fn div_assign(&mut self, rhs: T)
    /// Divides `self` by `rhs`, and assigns the quotient to `self` back..
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
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
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
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
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
    /// a_biguint.wrapping_div_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Exmaples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// // a_biguint.wrapping_div_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// // a_biguint.wrapping_div_assign_uint(_divisor);
    /// ```
    #[inline]
    fn div_assign(&mut self, _rhs: T)
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
impl<T, const N: usize> Rem<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    type Output = T;

    // fn rem(self, rhs: T) -> T
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
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u32;
    /// let remainder = dividend.clone() % divisor;
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u32;
    /// let remainder = dividend.clone() % divisor;
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u32;
    /// // It will panic!
    /// // let remainder = _dividend.clone() % _divisor;
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u32;
    /// // It will panic!
    /// // let remainder = _dividend.clone() % _divisor;
    /// ```
    /// 
    /// # Compile-fail Examples
    /// ```compile_fail
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u32;
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend.
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u32;
    /// let _remainder = dividend % divisor;
    /// // It cannot be compiled!
    /// println!("{} % {} = {}", dividend, divisor, _remainder);
    /// // The operator '%' swallowed (took the ownership of) dividend.
    /// ```
    #[inline]
    fn rem(self, _rhs: T) -> T
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
impl<T, const N: usize> RemAssign<T> for BigUInt<T, N>
where T: TraitsBigUInt<T>
{
    // fn rem_assign(&mut self, rhs: T)
    /// Divides `self` by `rhs`, and assigns the remainder to `self` back..
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
    /// let divisor = 87_u128;
    /// a_biguint %= divisor;
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
    /// let divisor = 87_u128;
    /// a_biguint %= divisor;
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// let _divisor = 0_u128;
    /// // It will panic!
    /// // _a_biguint %= _divisor;
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _divisor = 0_u128;
    /// // It will panic!
    /// // _a_biguint %= _divisor;
    /// ```
    #[inline]
    fn rem_assign(&mut self, _rhs: T)
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
impl<T, U, const N: usize> PartialEq<U> for BigUInt<T, N>
where T: TraitsBigUInt<T>,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn eq(&self, other: &U) -> bool
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    #[inline]
    fn eq(&self, _other: &U) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}



impl<T, U, const N: usize> PartialOrd<U> for BigUInt<T, N>
where T: TraitsBigUInt<T>,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn partial_cmp(&self, other: &U) -> Option<Ordering>
    /// # __self < other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is less than `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint < b_uint;
    /// if res
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} >= {}", a_biguint, b_uint); }
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
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint <= b_uint;
    /// if res
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} > {}", a_biguint, b_uint); }
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
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint > b_uint;
    /// if res
    ///     { println!("{} > {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} <= {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// # __self >= other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is greater than
    /// or equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(200_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint >= b_uint;
    /// if res
    ///     { println!("{} >= {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} < {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// 
    /// # __self == other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint == b_uint;
    /// if res
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// 
    /// # __self != other -> bool__
    /// 
    /// Compares `self` and `other` to find whether `self` is equal to `other`.
    /// 
    /// # Arguments
    /// `rhs` is to be compared with `self`, and primitive unsigned integer
    /// such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 100_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(100_u8);
    /// let b_uint = 200_u8;
    /// let res = a_biguint != b_uint;
    /// if res
    ///     { println!("{} != {}", a_biguint, b_uint); }
    /// else
    ///     { println!("{} == {}", a_biguint, b_uint); }
    /// assert_eq!(res, true);
    /// ```
    #[inline]
    fn partial_cmp(&self, _other: &U) -> Option<Ordering>
    {
        unimplemented!(); // Dummy code for documentation
    }
}



impl<T, U, const N: usize> From<U> for BigUInt<T, N>
where T: TraitsBigUInt<T>,
    U: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
        + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
        + Rem<Output=U> + RemAssign
        + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
        + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
        + BitXor<Output=U> + BitXorAssign + Not<Output=U>
        + PartialEq + PartialOrd
{
    // fn from(val: U) -> Self
    /// Constructs a new `BigUInt<T, N>`-type object from a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
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
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from(123456789123456789123456789123456789_u128);
    /// println!("a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "123456789123456789123456789123456789");
    /// ```
    /// 
    /// # For more examples,
    /// click [here](../documentation/big_uint_traits_impl/struct.BigUInt.html#method.from)
    #[inline]
    fn from(_val: U) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }
}
