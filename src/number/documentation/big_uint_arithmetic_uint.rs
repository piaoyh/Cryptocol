// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::From;
use std::str::FromStr;
use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };

use crate::number::SmallUInt;

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_arithmetic_uint.rs.
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

impl<T, const N: usize> BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd,
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


    /***** ARITHMATIC OPERATIONS WITH UINT *****/

    /*** ADDITION ***/

    // pub fn carrying_add_uint<U>(&self, rhs: U, carry: bool) -> (Self, bool)
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of an addition result `self` + `rhs` + `carry`
    /// along with a carry-out bit.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and is of primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `carry` is of `bool` type so that `1` may be added to `self`
    ///   if `carry` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns a tuple containing the sum and the output carry. It performs
    /// "ternary addition" of one `Self`-typed operand, a primitive unsigned
    /// integer, and a carry-in bit, and returns an tuple of an addition result
    /// in `Self` type and a carry-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - This allows chaining together multiple additions to create even a
    ///   wider addition. This can be thought of as a big integer
    ///   "full adder", in the electronics sense.
    /// - If the input carry is `false`, this method is equivalent to
    ///   `overflowing_add_uint()`, and the output carry reflects current
    ///   overflow.
    /// - The output carry is equal to the `OVERFLOW` flag of the return value.
    /// - If overflow happened, the flag `OVERFLOW` of the return value will
    ///   be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [carrying_add()](struct@BigUInt#method.carrying_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = a_biguint.carrying_add_uint(num_uint, false);
    /// println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722605");
    /// assert_eq!(carry, false);
    /// assert_eq!(sum.is_overflow(), false);
    /// assert_eq!(sum.is_underflow(), false);
    /// assert_eq!(sum.is_divided_by_zero(), false);
    /// assert_eq!(sum.is_infinity(), false);
    /// assert_eq!(sum.is_undefined(), false);
    /// assert_eq!(sum.is_left_carry(), false);
    /// assert_eq!(sum.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = a_biguint.carrying_add_uint(num_uint, true);
    /// println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722606");
    /// assert_eq!(carry, false);
    /// assert_eq!(sum.is_overflow(), false);
    /// assert_eq!(sum.is_underflow(), false);
    /// assert_eq!(sum.is_divided_by_zero(), false);
    /// assert_eq!(sum.is_infinity(), false);
    /// assert_eq!(sum.is_undefined(), false);
    /// assert_eq!(sum.is_left_carry(), false);
    /// assert_eq!(sum.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = a_biguint.carrying_add_uint(num_uint, false);
    /// println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "22774453838368691933710012711845097214");
    /// assert_eq!(carry, true);
    /// assert_eq!(sum.is_overflow(), true);
    /// assert_eq!(sum.is_underflow(), false);
    /// assert_eq!(sum.is_divided_by_zero(), false);
    /// assert_eq!(sum.is_infinity(), false);
    /// assert_eq!(sum.is_undefined(), false);
    /// assert_eq!(sum.is_left_carry(), false);
    /// assert_eq!(sum.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;
    /// 
    /// let (sum, carry) = a_biguint.carrying_add_uint(num_uint, true);
    /// println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    /// assert_eq!(sum.to_string(), "22774453838368691933710012711845097215");
    /// assert_eq!(carry, true);
    /// assert_eq!(sum.is_overflow(), true);
    /// assert_eq!(sum.is_underflow(), false);
    /// assert_eq!(sum.is_divided_by_zero(), false);
    /// assert_eq!(sum.is_infinity(), false);
    /// assert_eq!(sum.is_undefined(), false);
    /// assert_eq!(sum.is_left_carry(), false);
    /// assert_eq!(sum.is_right_carry(), false);
    /// ```
    pub fn carrying_add_uint<U>(&self, _rhs: U, _carry: bool) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn carrying_add_assign_uint<U>(&mut self, rhs: U, carry: bool) -> bool
    /// Calculates `self` + `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` + `carry` to `self` back,
    /// and returns the resulting carry.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `carry` is of `bool` type so that `1` may be added to `self`
    ///   if `carry` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns the output carry. It performs "ternary addition" of a
    /// `Self`-typed operands, primitive unsigned integer, and a carry-in bit,
    /// and returns a carry-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - This allows chaining together multiple additions to create even a
    ///   wider addition. This can be thought of as a big integer "full adder",
    ///   in the electronics sense.
    /// - If the input carry is false, this method is equivalent to
    ///   `overflowing_add_assign_uint()`, and the output carry reflects current
    ///   overflow.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [carrying_add_assign()](struct@BigUInt#method.carrying_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let carry = a_biguint.carrying_add_assign_uint(num_uint, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302573");
    /// assert_eq!(carry, false);
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
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// let carry = a_biguint.carrying_add_assign_uint(num_uint, true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302574");
    /// assert_eq!(carry, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let carry = a_biguint.carrying_add_assign_uint(num_uint, false);
    /// println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    /// assert_eq!(a_biguint.to_string(), "11024999611375677182");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_uint = 0x9900AABB_CCDDEEFF_u64;
    /// let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    /// let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let carry = a_biguint.carrying_add_assign_uint(num_uint, true);
    /// println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    /// assert_eq!(a_biguint.to_string(), "11024999611375677183");
    /// assert_eq!(carry, true);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn carrying_add_assign_uint<U>(&mut self, _rhs: U, _carry: bool) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_add_uint<U>(&self, rhs: U) -> Self
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
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [wrapping_add()](struct@BigUInt#method.wrapping_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.wrapping_add_uint(1_u8);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.wrapping_add_uint(2_u8);
    /// println!("{} + 2 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let res = a_biguint.wrapping_add_uint(3_u8);
    /// println!("{} + 3 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// ```
    pub fn wrapping_add_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_add_assign_uint<U>(&mut self, rhs: U)
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
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [wrapping_add_assign()](struct@BigUInt#method.wrapping_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
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
    /// a_biguint.wrapping_add_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::max();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_add_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_add_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
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
    pub fn wrapping_add_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_add_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the addition result `self` + `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_add()](struct@BigUInt#method.overflowing_add)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let (res, overflow) = a_biguint.overflowing_add_uint(1_u8);
    /// println!("{} + 1 = {}\noverflow = {}", a_biguint, res, overflow);
    /// assert_eq!(res, U512::max());
    /// assert_eq!(overflow, false);
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
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let (res, overflow) = a_biguint.overflowing_add_uint(2_u8);
    /// println!("{} + 2 = {}\noverflow = {}", a_biguint, res, overflow);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(overflow, true);
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    /// let (res, overflow) = a_biguint.overflowing_add_uint(3_u8);
    /// println!("{} + 3 = {}\noverflow = {}", a_biguint, res, overflow);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_add_uint<U>(&self, _rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` + `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the addition result `self` + `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// It returns true if an arithmetic overflow would occur.
    /// Otherwise, it returns `false`.
    /// 
    /// # Features
    /// - Wrapping (modular) addition.
    /// - If overflow happened, the flag `OVERFLOW` of `self` will be set.
    /// - If overflow did not happen in the current operation, the output
    ///   will be false even if the `OVERFLOW` flag of `self` was already set
    ///   because of previous operation of `self`.
    /// - The output reflects only the current overflow.
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_add_assign()](struct@BigUInt#method.overflowing_add_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_add_assign_uint(1_u8);
    /// println!("After a_biguint.overflowing_add_assign_uint(1_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    /// assert_eq!(a_biguint, UU64::max());
    /// assert_eq!(overflow, false);
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
    /// let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let mut overflow = a_biguint.overflowing_add_assign_uint(2_u8);
    /// println!("After a_biguint.overflowing_add_assign_uint(2_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// overflow = a_biguint.overflowing_add_assign_uint(2_u8);
    /// println!("After a_biguint.overflowing_add_assign_uint(2_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_add_assign_uint<U>(&mut self, _rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_add_uint(2_u8);
    /// ```
    #[inline]
    pub fn unchecked_add_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the modulo-sum (`self` + `rhs`) % `modulo` with wrapping
    /// (modular) addition at `modulo`.
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `sum` divided by `modulo`.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method `panic_free_modular_add_uint()`
    ///   and the method `wrapping_add_uint()` are, first, where
    ///   wrapping around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_add()](struct@BigUInt#method.panic_free_modular_add)
    /// is proper rather than this method `panic_free_modular_add_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 2_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 3 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = a_biguint.wrapping_add_uint(2_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 5 for op1 == op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 10 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 11 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 12 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::zero();
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 13 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::one();
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    /// println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    ///             println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    pub fn panic_free_modular_add_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd

    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// 
    /// # Features
    /// - It takes the addition (= `sum`) of `self` and `rhs`,
    ///   and then finally assigns the remainder of `sum` divided by `modulo`
    ///   to `self` back.
    /// - Wrapping (modular) addition at `modulo`.
    /// - The differences between this method
    ///   `panic_free_modular_add_assign_uint()` and the method
    ///   `wrapping_add_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_add_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_add_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulo` | result value (self) | flags       |
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
    /// [panic_free_modular_add_assign_uint()](struct@BigUInt#method.panic_free_modular_add_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// let rhs = 2_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_add_assign_uint(1_u8, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint(1_u8, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "2");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "3");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "6");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 0_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 250_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = U256::zero();
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    /// let m = U256::one();
    /// println!("Originally, a = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// 
    /// let rhs = 3_u8;
    /// a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let mut a_biguint = a.clone();
    ///             println!("Originally, a = {}", a_biguint);
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), false);
    ///             assert_eq!(res.is_left_carry(), false);
    ///             assert_eq!(res.is_right_carry(), false);
    ///         
    ///             a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    ///             println!("After a_biguint.panic_free_modular_add_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
    ///             assert_eq!(a_biguint.to_string(), "0");
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), true);
    ///             assert_eq!(res.is_left_carry(), false);
    ///             assert_eq!(res.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_add_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    /*** Subtraction ***/

    // pub fn borrowing_sub_uint<U>(&self, rhs: U, borrow: bool) -> (Self, bool)
    /// Calculates `self` - `rhs` - `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of a subtraction result `self` - `rhs` - `carry`
    /// along with a borrow-out bit.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and is of primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    ///   `overflowing_sub_uint()`, and the output borrow reflects current
    ///   underflow.
    /// - The output borrow is equal to the `UNDERFLOW` flag
    ///   of the return value.
    /// - If underflow happened, the flag `UNDERFLOW` of the return value will
    ///   be set.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [borrowing_sub()](struct@BigUInt#method.borrowing_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, false);
    /// println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    /// assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528175");
    /// assert_eq!(borrow, false);
    /// assert_eq!(dif.is_underflow(), false);
    /// assert_eq!(dif.is_overflow(), false);
    /// assert_eq!(dif.is_divided_by_zero(), false);
    /// assert_eq!(dif.is_infinity(), false);
    /// assert_eq!(dif.is_undefined(), false);
    /// assert_eq!(dif.is_left_carry(), false);
    /// assert_eq!(dif.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, true);
    /// println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    /// assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528174");
    /// assert_eq!(borrow, false);
    /// assert_eq!(dif.is_underflow(), false);
    /// assert_eq!(dif.is_overflow(), false);
    /// assert_eq!(dif.is_divided_by_zero(), false);
    /// assert_eq!(dif.is_infinity(), false);
    /// assert_eq!(dif.is_undefined(), false);
    /// assert_eq!(dif.is_left_carry(), false);
    /// assert_eq!(dif.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    /// let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, false);
    /// println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    /// assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639919");
    /// assert_eq!(borrow, true);
    /// assert_eq!(dif.is_underflow(), true);
    /// assert_eq!(dif.is_overflow(), false);
    /// assert_eq!(dif.is_divided_by_zero(), false);
    /// assert_eq!(dif.is_infinity(), false);
    /// assert_eq!(dif.is_undefined(), false);
    /// assert_eq!(dif.is_left_carry(), false);
    /// assert_eq!(dif.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    /// let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;
    /// 
    /// let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, true);
    /// println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    /// assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639918");
    /// assert_eq!(borrow, true);
    /// assert_eq!(dif.is_underflow(), true);
    /// assert_eq!(dif.is_overflow(), false);
    /// assert_eq!(dif.is_divided_by_zero(), false);
    /// assert_eq!(dif.is_infinity(), false);
    /// assert_eq!(dif.is_undefined(), false);
    /// assert_eq!(dif.is_left_carry(), false);
    /// assert_eq!(dif.is_right_carry(), false);
    /// ```
    pub fn borrowing_sub_uint<U>(&self, _rhs: U, _borrow: bool) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn borrowing_sub_assign_uint<U>(&mut self, rhs: U, borrow: bool) -> bool
    /// Calculates `self` - `rhs` - `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the subtraction result `self` - `rhs` - `carry`
    /// to `self` back,
    /// and return the resulting borrow.
    /// 
    /// # Arguments
    /// - `rhs` is to be subtracted from `self`, and is of primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `borrow` is of `bool` type so that `1` may be subtracted from `self`
    ///   if `borrow` is `true`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns the output borrow. It performs "ternary subtraction" of one
    /// `Self`-typed operand, primitive unsigned integer, and a borrow-in bit,
    /// and returns a borrow-out bit.
    /// 
    /// # Features
    /// - Wrapping (modular) subtraction.
    /// - This allows chaining together multiple subtraction to create even a
    ///   wider subtraction. This can be thought of as a big integer
    ///   "full subtracter", in the electronics sense.
    /// - If the input borrow is false, this method is equivalent to
    ///   `overflowing_sub_assign_uint()`, and the output borrow reflects
    ///   the current underflow.
    /// - If underflow happened, the flag `UNDERFLOW` of `self` will be set.
    /// - All the flags are historical, which means, for example, if an underflow
    ///   occurred even once before this current operation or `UNDERFLOW`
    ///   flag is already set before this current operation, the `UNDERFLOW` flag
    ///   is not changed even if this current operation does not cause underflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `ui128`, the method
    /// [borrowing_sub_assign()](struct@BigUInt#method.borrowing_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, false);
    /// println!("After a_biguint -= {}, a_biguint = {}\tborrow = {}", num_uint, a_biguint, borrow);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948207");
    /// assert_eq!(borrow, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    /// let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, true);
    /// println!("After a_biguint -= {}, a_biguint = {}\tborrow = {}", num_uint, a_biguint, borrow);
    /// assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948206");
    /// assert_eq!(borrow, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let num_str2 = "9900AABB_CCDDEEFe";
    /// let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, false);
    /// println!("After a_biguint -= {}, a_biguint = {}\tcarry = {}", num_uint, a_biguint, borrow);
    /// assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    /// assert_eq!(borrow, true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let num_str2 = "9900AABB_CCDDEEFe";
    /// let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    /// let num_uint = 0x9900AABB_CCDDEEFf_u64;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, true);
    /// println!("After a_biguint -= {}, a_biguint = {}\tborrow = {}", num_uint, a_biguint, borrow);
    /// assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639934");
    /// assert_eq!(borrow, true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn borrowing_sub_assign_uint<U>(&mut self, _rhs: U, _borrow: bool) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_sub_uint<U>(&self, rhs: U) -> Self
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
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [wrapping_sub()](struct@BigUInt#method.wrapping_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub_uint(1_u8);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub_uint(2_u8);
    /// println!("{} - 2 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
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
    /// let a_biguint = U512::one();
    /// let res = a_biguint.wrapping_sub_uint(3_u8);
    /// println!("{} - 3 = {}", a_biguint, res);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn wrapping_sub_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_sub_assign_uint<U>(&mut self, rhs: U)
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
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [wrapping_sub_assign()](struct@BigUInt#method.wrapping_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// a_biguint.wrapping_sub_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
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
    /// a_biguint.wrapping_sub_assign_uint(2_u8);
    /// println!("After a_biguint.wrapping_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
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
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_sub_assign_uint(3_u8);
    /// println!("After a_biguint.wrapping_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.wrapping_sub_assign_uint(1_u8);
    /// println!("After a_biguint.wrapping_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn wrapping_sub_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_sub_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the subtraction result `self` - `rhs` along with
    /// a boolean indicating whether an arithmetic underflow would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_sub()](struct@BigUInt#method.overflowing_sub)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::one();
    /// let (res, underflow) = a_biguint.overflowing_sub_uint(1_u8);
    /// println!("{} - 1 = {}\nunderflow = {}", a_biguint, res, underflow);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(underflow, false);
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
    /// let (res, underflow) = a_biguint.overflowing_sub_uint(2_u8);
    /// println!("{} - 2 = {}\nunderflow = {}", a_biguint, res, underflow);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(underflow, true);
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
    /// let (res, underflow) = a_biguint.overflowing_sub_uint(3_u8);
    /// println!("{} - 3 = {}\nunderflow = {}", a_biguint, res, underflow);
    /// assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(underflow, true);
    /// assert_eq!(res.is_underflow(), true);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_sub_uint<U>(&self, _rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_add_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` - `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the subtraction result `self` - `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic underflow
    /// would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_sub_assign()](struct@BigUInt#method.overflowing_sub_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign_uint(1_u8);
    /// println!("After a_biguint.overflowing_sub_assign_uint(1_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(underflow, false);
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
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign_uint(2_u8);
    /// println!("After a_biguint.overflowing_sub_assign_uint(2_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    /// assert_eq!(underflow, true);
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
    /// let mut a_biguint = UU64::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign_uint(3_u8);
    /// println!("After a_biguint.overflowing_sub_assign_uint(3_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(underflow, true);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let underflow = a_biguint.overflowing_sub_assign_uint(1_u8);
    /// println!("After a_biguint.overflowing_sub_assign_uint(1_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    /// assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(underflow, false);
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_sub_assign_uint<U>(&mut self, _rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU64::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_sub_uint(2_u8);
    /// ```
    #[inline]
    pub fn unchecked_sub_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
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
    /// - The differences between this method `panic_free_modular_sub_uint()`
    ///   and the method `wrapping_sub_uint()` are, first, where
    ///   wrapping around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub_uint()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_sub()](struct@BigUInt#method.panic_free_modular_sub)
    /// is proper rather than this method `panic_free_modular_sub_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 2_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 3 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
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
    /// # Example 4 for op1 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 3_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 7 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 8 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 9 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(750_u16);
    /// let m = U256::from_uint(250_u8);
    /// let rhs = 0_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 10 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(0_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 11 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(150_u8);
    /// let m = U256::from_uint(50_u8);
    /// let rhs = 250_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    /// # Example 12 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::zero();
    /// let rhs = 1_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 13 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(2_u8);
    /// let m = U256::one();
    /// let rhs = 1_u8;
    /// let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    /// println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    ///             println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
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
    pub fn panic_free_modular_sub_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` - `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then, assigns the result back to `self`.
    /// 
    /// # Arguments
    /// - `rhs` is to be added to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `modulo` is the divisor to divide the result of (`self` - `rhs`),
    ///   and is of `&Self` type.
    /// 
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the subtraction (= `difference`) of `rhs` from `self`, and
    ///   then finally returns the remainder of `difference` divided
    ///   by `modulo`.
    /// - Wrapping (modular) subtraction at `modulo`.
    /// - The differences between this method
    ///   `panic_free_modular_sub_assign_uint()` and the method
    ///   `wrapping_sub_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `UNDERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_sub_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `UNDERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_sub_assign_uint()` sets
    ///   `UNDERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulo` | result value (self) | flags       |
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
    /// [panic_free_modular_sub_assign_uint()](struct@BigUInt#method.panic_free_modular_sub_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 2_u8;
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
    /// # Example 3 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_uint(2_u8);
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let rhs = 3_u8;
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
    /// assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    /// assert_eq!(a_biguint.is_underflow(), true);
    /// assert_eq!(a_biguint.is_overflow(), false);
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 5 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 6 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 11 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(2_u8);
    /// let m = U256::zero();
    /// let rhs = 3_u8;
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
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_uint(2_u8);
    /// let m = U256::one();
    /// let rhs = 3_u8;
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
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example 12 for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let mut a_biguint = a.clone();
    ///             println!("Originally, a = {}", a_biguint);
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), false);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         
    ///             a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    ///             println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
    ///             assert_eq!(a_biguint.to_string(), "0");
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), true);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_sub_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** Multiplication ***/

    // pub fn carrying_mul_uint<U>(&self, rhs: U, carry: Self) -> (Self, Self)
    /// Calculates `self` * `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple the low-order (wrapping) bits and the high-order
    /// (overflow) bits of the result of the calculation
    /// `self` * `rhs` + `carry`.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `carry` is of `Self` type
    ///   so that `carry` may be added to `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
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
    ///   `widening_mul_uint()`.
    /// 
    /// # Counterpart Methods
    /// - If you don’t need the carry, then you can use
    ///   [widening_mul_uint()](struct@BigUInt#method.widening_mul_uint) instead.
    /// - The value of the first field in the returned tuple matches
    ///   what you’d get by combining the methods
    ///   [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint) and
    ///   [wrapping_add()](struct@BigUInt#method.wrapping_add):
    ///   `self.wrapping_mul_uint(rhs).wrapping_add(carry)`. So,
    ///   `self.carrying_mul_uint(rhs, carry).0` == `self.wrapping_mul_uint(rhs).wrapping_add(carry)`
    /// - If `rhs` is bigger than `u128`, the method
    ///   [widening_mul()](struct@BigUInt#method.widening_mul)
    ///   is proper rather than this method `widening_mul_uint()`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_low_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let a_high_biguint = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// let (res_low, res_high) = a_low_biguint.carrying_mul_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// let (res_high, res_higher) = a_high_biguint.carrying_mul_uint(b_uint, res_high);
    /// println!("{}:{} X {} = {}:{}:{}", a_high_biguint, a_low_biguint, b_uint, res_higher, res_high, res_low);
    /// assert_eq!(res_higher.to_string(), "0");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// assert_eq!(res_low.is_overflow(), false);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_low_biguint = U256::max();
    /// let a_high_biguint = UU32::max();
    /// let b_uint = u64::MAX;
    /// let (res_low, res_high) = a_low_biguint.carrying_mul_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// let (res_high, res_higher) = a_high_biguint.carrying_mul_uint(b_uint, res_high);
    /// println!("{}:{} X {:X} = {}:{}:{}", a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_uint, res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), res_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_low.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000040");
    /// assert_eq!(res_high.is_overflow(), true);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    /// assert_eq!(res_low.is_overflow(), true);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    pub fn carrying_mul_uint<U>(&self, _rhs: U, _carry: Self) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn carrying_mul_assign_uint<U>(&mut self, rhs: U, carry: Self) -> Self
    /// Calculates `self` * `rhs` + `carry`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the low-order (wrapping) bits of the result
    /// `self` * `rhs` + `carry` back to `self`,
    /// and returns the high-order (overflow) bits of the result.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// - `carry` is of `Self` type
    ///   so that `carry` may be added to `self` * `rhs`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
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
    ///   `widening_mul_assign_uint()`.
    /// 
    /// # Counterpart Method
    /// - If you don’t need the carry, then you can use
    ///   [widening_mul_assign_uint()](struct@BigUInt#method.widening_mul_assign_uint)
    ///   instead.
    /// - The value of `self` after calculation matches what you’d get by
    ///   combining the mehtods
    ///   [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint) and
    ///   [wrapping_add_assign()](struct@BigUInt#method.wrapping_add_assign):
    ///   `self.wrapping_mul_uint(rhs).wrapping_add_assign_uint(carry)`.
    /// - If `rhs` is bigger than `u128`, the method
    ///   [carrying_mul_assign()](struct@BigUInt#method.carrying_mul_assign)
    ///   is proper rather than this method `carrying_mul_assign_uint()`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_low_biguint = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_high_biguint = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    /// let b_uint = 225_u8;
    /// 
    /// println!("Originally, a_low_biguint = {}", a_low_biguint);
    /// assert_eq!(a_low_biguint.is_overflow(), false);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// println!("Originally, a_high_biguint = {}\n", a_high_biguint);
    /// assert_eq!(a_high_biguint.is_overflow(), false);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// 
    /// let res_high = a_low_biguint.carrying_mul_assign_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// println!("After a_low_biguint.carrying_mul_assign_uint(225_u8, 0),\na_low_biguint = {}", a_low_biguint);
    /// assert_eq!(a_low_biguint.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    /// assert_eq!(a_low_biguint.is_overflow(), false);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// let res_higher = a_high_biguint.carrying_mul_assign_uint(b_uint, res_high);
    /// println!("After a_high_biguint.carrying_mul_assign_uint(225_u8, res_higher),\na_high_biguint = {}\nres_higher = {}", a_high_biguint, res_higher);
    /// assert_eq!(res_higher.to_string(), "0");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// 
    /// assert_eq!(a_high_biguint.is_overflow(), false);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_low_biguint = U256::max();
    /// let mut a_high_biguint = UU32::max();
    /// let b_uint = u64::MAX;
    /// 
    /// println!("Originally, a_low_biguint = {}", a_low_biguint);
    /// assert_eq!(a_low_biguint.is_overflow(), false);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// println!("Originally, a_high_biguint = {}\n", a_high_biguint);
    /// assert_eq!(a_high_biguint.is_overflow(), false);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// 
    /// let res_high = a_low_biguint.carrying_mul_assign_uint(b_uint, UU32::zero());
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// println!("After a_low_biguint.carrying_mul_assign_uint(u64::MAX, 0),\na_low_biguint = {}", a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    /// assert_eq!(a_low_biguint.is_overflow(), true);
    /// assert_eq!(a_low_biguint.is_underflow(), false);
    /// assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_low_biguint.is_infinity(), false);
    /// assert_eq!(a_low_biguint.is_undefined(), false);
    /// assert_eq!(a_low_biguint.is_left_carry(), false);
    /// assert_eq!(a_low_biguint.is_right_carry(), false);
    /// 
    /// let res_higher = a_high_biguint.carrying_mul_assign_uint(b_uint, res_high);
    /// println!("After a_high_biguint.carrying_mul_assign_uint(u64::MAX),\na_high_biguint = {}\nres_higher = {}", a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_higher.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000040");
    /// assert_eq!(a_high_biguint.is_overflow(), true);
    /// assert_eq!(a_high_biguint.is_underflow(), false);
    /// assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_high_biguint.is_infinity(), false);
    /// assert_eq!(a_high_biguint.is_undefined(), false);
    /// assert_eq!(a_high_biguint.is_left_carry(), false);
    /// assert_eq!(a_high_biguint.is_right_carry(), false);
    /// 
    /// assert_eq!(res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    /// assert_eq!(res_higher.is_overflow(), false);
    /// assert_eq!(res_higher.is_underflow(), false);
    /// assert_eq!(res_higher.is_divided_by_zero(), false);
    /// assert_eq!(res_higher.is_infinity(), false);
    /// assert_eq!(res_higher.is_undefined(), false);
    /// assert_eq!(res_higher.is_left_carry(), false);
    /// assert_eq!(res_higher.is_right_carry(), false);
    /// ```
    pub fn carrying_mul_assign_uint<U>(&mut self, _rhs: U, _carry: Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn widening_mul_uint<U>(&self, rhs: U) -> (Self, Self)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple the low-order (wrapping) bits and the high-order
    /// (overflow) bits of the result of the calculation  `self` * `rhs`.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and is of primitive unsigned
    ///   integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
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
    ///   use [carrying_mul_uint()](struct@BigUInt#method.carrying_mul_uint)
    ///   instead.
    /// - The value of the first field in the returned tuple matches what
    ///   you’d get the method
    ///   [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint).
    ///   `self.widening_mul_uint(rhs).0` == `self.wrapping_mul_uint(rhs)`.
    /// - If `rhs` is bigger than `u128`, the method
    ///   [widening_mul()](struct@BigUInt#method.widening_mul)
    ///   is proper rather than this method `widening_mul_uint()`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u128;
    /// let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    /// println!("{} X {} = {}:{}", a_biguint, b_uint, res_high, res_low);
    /// assert_eq!(res_high.to_string(), "1");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res_low.is_overflow(), true);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::max();
    /// let b_uint = u128::MAX;
    /// let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    /// println!("{} X {:X} = {}:{}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_uint, res_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_low.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "7F");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(res_low.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000000_00000000_00000001");
    /// assert_eq!(res_low.is_overflow(), true);
    /// assert_eq!(res_low.is_underflow(), false);
    /// assert_eq!(res_low.is_divided_by_zero(), false);
    /// assert_eq!(res_low.is_infinity(), false);
    /// assert_eq!(res_low.is_undefined(), false);
    /// assert_eq!(res_low.is_left_carry(), false);
    /// assert_eq!(res_low.is_right_carry(), false);
    /// ```
    pub fn widening_mul_uint<U>(&self, _rhs: U) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn widening_mul_assign_uint<U>(&mut self, rhs: U) -> Self
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the low-order (wrapping) bits of the result `self` * `rhs`,
    /// and returns the high-order (overflow) bits of the result.
    /// 
    /// # Arguments
    /// - `rhs` is to be multiplied to `self`, and primitive unsigned integer
    ///   such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    /// 
    /// # Panics
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Outputs
    /// It returns the high-order (overflow) bits of the result `self` * `rhs`.
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
    ///   [carrying_mul_assign_uint()](struct@BigUInt#method.carrying_mul_assign_uint)
    ///   instead.
    /// - The value of `self` after calculation matches what you’d get the
    ///   method [wrapping_mul_uint()](struct@BigUInt#method.wrapping_mul_uint)
    ///   so `self` == `self.wrapping_mul_uint(rhs)`.
    /// - If `rhs` is bigger than `u128`, the method 
    ///   [widening_mul_assign()](struct@BigUInt#method.widening_mul_assign)
    ///   is proper rather than this method `widening_mul_assign_uint()`.
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u64;
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
    /// let res_high = a_biguint.widening_mul_assign_uint(b_uint);
    /// println!("After a_biguint.widening_mul_assign_uint(248_u8),\na_biguint = {}\nres_high = {}", a_biguint, res_high);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res_high.to_string(), "1");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    ///     
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 2 for Maximum case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::max();
    /// let b_uint = u64::MAX;
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let res_high = a_biguint.widening_mul_assign_uint(b_uint);
    /// println!("After a_biguint.widening_mul_assign_uint(u64::MAX),\na_biguint = {}\nres_high = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_high.to_string_with_radix_and_stride(16, 8).unwrap());
    /// assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    /// assert_eq!(res_high.is_overflow(), false);
    /// assert_eq!(res_high.is_underflow(), false);
    /// assert_eq!(res_high.is_divided_by_zero(), false);
    /// assert_eq!(res_high.is_infinity(), false);
    /// assert_eq!(res_high.is_undefined(), false);
    /// assert_eq!(res_high.is_left_carry(), false);
    /// assert_eq!(res_high.is_right_carry(), false);
    /// 
    /// assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn widening_mul_assign_uint<U>(&mut self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_mul_uint<U>(&self, rhs: U) -> Self
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
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_mul()](struct@BigUInt#method.wrapping_mul)
    /// is proper rather than this method `wrapping_mul_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u16;
    /// let res = a_biguint.wrapping_mul_uint(b_uint);
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
    /// define_utypes_with!(u32);
    /// 
    /// let c_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u16;
    /// let res = c_biguint.wrapping_mul_uint(b_uint);
    /// println!("{} X {} = {}", c_biguint, b_uint, res);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn wrapping_mul_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_mul_assign_uint<U>(&mut self, rhs: U)
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
    /// # Counterpart Method
    /// If `rhs` is bigger tham `u128`, the method
    /// [wrapping_mul_assign()](struct@BigUInt#method.wrapping_mul_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    /// let b_uint = 248_u16;
    /// 
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
    /// a_biguint.wrapping_mul_assign_uint(b_uint);
    /// println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
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
    /// let b_uint = 248_u16;
    /// 
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
    /// a_biguint.wrapping_mul_assign_uint(b_uint);
    /// println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn wrapping_mul_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_mul_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and returns a tuple of the multiplication result `self` * `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_mul()](struct@BigUInt#method.overflowing_mul)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    /// println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    /// assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    /// println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    /// assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(overflow, true);
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn overflowing_mul_uint<U>(&self, _rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_mul_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Calculates `self` * `rhs`,
    /// wrapping around at the boundary of the `Self` type,
    /// and assigns the multiplication result `self` * `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// - All the flags are historical, which means, for example, if an overflow
    ///   occurred even once before this current operation or `OVERFLOW`
    ///   flag is already set before this current operation, the `OVERFLOW` flag
    ///   is not changed even if this current operation does not cause overflow.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_mul_assign()](struct@BigUInt#method.overflowing_mul_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let b_uint = 248_u8;
    /// 
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1874298166903427690031858186486050853753882811946569946433649006084094");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    /// println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    /// assert_eq!(overflow, false);
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
    /// let b_uint = 248_u8;
    /// 
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
    /// let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    /// println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    /// assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    /// assert_eq!(overflow, true);
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn overflowing_mul_assign_uint<U>(&mut self, _rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_mul_uint(248_u8);
    /// ```
    #[inline]
    pub fn unchecked_mul_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
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
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | return value | flags       |
    /// |----------|--------------|-------------|
    /// | 0 or 1   | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_mul()](struct@BigUInt#method.panic_free_modular_mul)
    /// is proper rather than this method `panic_free_modular_mul_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_uint = 248_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// # Example 4 for op1 = multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 5_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// # Example 7 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// # Example 8 for op1 == 0 and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::zero();
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 0_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// define_utypes_with!(u128);
    /// 
    /// let m = UU32::from_uint(1000_u16);
    /// let a_biguint = U256::from_uint(4321000_u32);
    /// let mul_uint = 4321000_u32;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
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
    /// # Example 11 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let m = UU32::zero();
    /// let mul_uint = 248_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let m = UU32::one();
    /// let mul_uint = 248_u8;
    /// let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    /// println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let res = a_biguint.panic_free_modular_mul_uint(rhs, &m);
    ///             println!("{} * {} = {} (mod {})", a_biguint, rhs, res, m);
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
    pub fn panic_free_modular_mul_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    /// Calculates (`self` * `rhs`) % `modulo`,
    /// wrapping around at `modulo` of the `Self` type,
    /// and then assigns the result back to `self`.
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
    /// 
    /// # Features
    /// - It takes the multiplication (= `product`) of `self` and `rhs`,
    ///   and then finally returns the remainder of `product`
    ///   divided by `modulo`.
    /// - Wrapping (modular) multiplication at `modulo`.
    /// - The differences between this method
    ///   `panic_free_modular_mul_assign_uint()` and the method
    ///   `wrapping_mul_assign_uint()` are, first, where wrapping
    ///   around happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `wrapping_mul_assign_uint()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `wrapping_mul_assign_uint()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of `self`
    ///   will be set and `self` will have the value `0`.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulo` | result value (self) | flags       |
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
    /// [panic_free_modular_mul_assign_uint()](struct@BigUInt#method.panic_free_modular_mul_assign_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let mul_uint = 248_u8;
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
    /// # Example 3 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
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
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 4 for op1 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 5 for op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 6 for op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 7 for op1 == 0 and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 8 for op1 == multiple of modulo and op2 == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 9 for op1 == multiple of modulo and op2 == multiple of modulo
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    /// # Example 10 for modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let m = UU32::zero();
    /// let mul_uint = 248_u8;
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
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    /// let m = UU32::one();
    /// let mul_uint = 248_u8;
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
    /// println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    ///  
    /// for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    /// {
    ///     for rhs in [0_u8, 3_u8, 50_u8]
    ///     {
    ///         for m in [U256::zero(), U256::one()]
    ///         {
    ///             let mut a_biguint = a.clone();
    ///             println!("Originally, a = {}", a_biguint);
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), false);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         
    ///             a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    ///             println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
    ///             assert_eq!(a_biguint.to_string(), "0");
    ///             assert_eq!(a_biguint.is_overflow(), false);
    ///             assert_eq!(a_biguint.is_underflow(), false);
    ///             assert_eq!(a_biguint.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_infinity(), false);
    ///             assert_eq!(a_biguint.is_undefined(), true);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_mul_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }


    /*** Division ***/

    // pub fn divide_fully_uint<U>(&self, rhs: U) -> (Self, U)
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
    /// - If `rhs` is zero, this method will panic.
    /// 
    /// # Output
    /// It returns tuple of a quotient and a remainder.
    /// The quotient is of `Self` type, and the remainder is of the primitive
    /// unsigned integral data type such as `u8`, `u16`, `u32`, `u64`,
    /// and `u128`.
    /// 
    /// # Features
    /// - There’s no way wrapping could ever happen unless `rhs` is zero.
    /// - If `rhs` is zero, this method will panic.
    /// - This function is the base function for all the methods *_div_uint(),
    ///   *_div_assign_uint(), *_rem_uint(), and *_rem_assign_uint().
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [divide_fully()](struct@BigUInt#method.divide_fully)
    /// is proper rather than this method `divide_fully_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(quotient.is_overflow(), false);
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
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(remainder.to_string(), "0");
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
    /// define_utypes_with!(u8);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (quotient, remainder) = _dividend.divide_fully_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (quotient, remainder) = _dividend.divide_fully_uint(_divisor);
    /// ```
    pub fn divide_fully_uint<U>(&self, _rhs: U) -> (Self, U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_divide_fully_uint<U>(&self, rhs: U) -> (Self, Self)
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
    /// [panic_free_divide_fully()](struct@BigUInt#method.panic_free_divide_fully)
    /// is proper rather than this method `panic_free_divide_fully_uint()`.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
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
    /// # Example 3 for dividend != 0 and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient, UU32::max());
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), true);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for dividend == 0 and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 0_u8;
    /// let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    /// println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// 
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    pub fn panic_free_divide_fully_uint<U>(&self, _rhs: U) -> (Self, Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_div_uint<U>(&self, rhs: U) -> Self
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
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_div()](struct@BigUInt#method.wrapping_div)
    /// is proper rather than this method `wrapping_div_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let quotient = dividend.wrapping_div_uint(divisor);
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
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u8;
    /// let quotient = dividend.wrapping_div_uint(divisor);
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
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will painc!
    /// let quotient = _dividend.wrapping_div_uint(_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u8;
    /// // It will painc!
    /// let quotient = _dividend.wrapping_div_uint(_divisor);
    /// ```
    pub fn wrapping_div_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_div_assign_uint<U>(&mut self, rhs: U)
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
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_div_assign()](struct@BigUInt#method.wrapping_div_assign)
    /// is proper rather than this method `wrapping_div_assign_uint()`.
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
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_div_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = 0_u8;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_div_assign_uint(_divisor);
    /// ```
    pub fn wrapping_div_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_div_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of the quotient of `self` / `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_div()](struct@BigUInt#method.overflowing_div)
    /// is proper rather than this method.
    ///
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (quotient, overflow) = dividend.overflowing_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
    /// assert_eq!(quotient.is_overflow(), false);
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
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u8;
    /// let (quotient, overflow) = dividend.overflowing_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(overflow, false);
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_divided_by_zero(), false);
    /// assert_eq!(quotient.is_undefined(), false);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Exmaples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (quotient, overflow) = _dividend.overflowing_div_uint(_divisor);
    /// 
    /// let _dividend = U256::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (quotient, overflow) = _dividend.overflowing_div_uint(_divisor);
    /// ```
    pub fn overflowing_div_uint<U>(&self, _rhs: U) -> (Self, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_div_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Divides `self` by `rhs`,
    /// and assigns the quotient of `self` / `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_div_assign()](struct@BigUInt#method.overflowing_div_assign)
    /// is proper rather than this method.
    ///
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_div_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u8);
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
    /// let overflow = a_biguint.overflowing_div_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(overflow, false);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
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
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_div_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = UU32::zero();
    /// let _divisor = 0_u16;
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_div_assign_uint(_divisor);
    /// ```
    pub fn overflowing_div_assign_uint<U>(&mut self, _rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_div_uint<U>(&self, rhs: U) -> Self
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
    /// [panic_free_div_uint()](struct@BigUInt#method.panic_free_div_uint)
    /// is a bit faster than this method `wrapping_div()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_div_uint()](struct@BigUInt#method.panic_free_div_uint).
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 87_u8;
    /// let quotient = dividend.panic_free_div_uint(divisor);
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
    /// # Example 3 for dividend != 0 and divisor = 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let quotient = dividend.panic_free_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
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
    /// # Example 4 for dividend == 0 and divisor = 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let quotient = dividend.panic_free_div_uint(divisor);
    /// println!("{} / {} = {}", dividend, divisor, quotient);
    /// assert_eq!(quotient.to_string(), "0");
    /// assert_eq!(quotient.is_overflow(), false);
    /// assert_eq!(quotient.is_underflow(), false);
    /// assert_eq!(quotient.is_infinity(), false);
    /// assert_eq!(quotient.is_undefined(), true);
    /// assert_eq!(quotient.is_divided_by_zero(), true);
    /// assert_eq!(quotient.is_left_carry(), false);
    /// assert_eq!(quotient.is_right_carry(), false);
    /// ```
    pub fn panic_free_div_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_div_assign_uint<U>(&mut self, rhs: U)
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
    /// [panic_free_div_assign()](struct@BigUInt#method.panic_free_div_assign)
    /// is proper rather than this method `panic_free_div_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = 87_u8;
    /// a_biguint.panic_free_div_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
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
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let divisor = 0_u8;
    /// a_biguint.panic_free_div_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
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
    /// let mut a_biguint = UU32::zero();
    /// println!("Originally,\n_a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 0_u8;
    /// a_biguint.panic_free_div_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_div_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the quotient of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the quotient of `rd1` divided by `rd2`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulo` or `modulo` is zero or one.
    /// - If `modulo` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulo` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulo` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulo` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo`, and `self` is zero or multiple of `modulo` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo`, and `self` is not zero, and `modulo` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulo` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulo`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulo`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulo`)    | 0 (mod `modulo`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulo`)    | != 0 (mod `modulo`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_div()](struct@BigUInt#method.panic_free_modular_div)
    /// is proper rather than this method `panic_free_modular_div_uint()`.
    /// 
    /// # Example 1 for a normal case for modulo >= 2 and dividend != 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
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
    /// # Example 2 for normal case for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.modular_div_uint(divisor, &modulo);
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
    /// # Example 3 for normal case for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(10000_u16);
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.modular_div_uint(divisor, &modulo);
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
    /// # Example 4 for modulo >= 2 and divisor == 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 5 for modulo >= 2 and divisor == multiple of modulo and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 6 for modulo >= 2 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 7 for modulo >= 2 and divisor == 0 and dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(30000_u16);
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 8 for modulo >= 2 and divisor == multiple of modulo and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 9 for modulo >= 2 and divisor == multiple of modulo and dividend == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_uint(30000_u16);
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 10 for modulo == 0 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::zero();
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 11 for modulo == 1 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::one();
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 12 for modulo == 0 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::zero();
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Example 13 for modulo == 1 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::one();
    /// let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    /// println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
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
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// for modulo in [U256::zero(), U256::one()]
    /// {
    ///     let op1 = U256::zero();
    ///     let op2 = 0_u8;
    ///     let res = op1.panic_free_modular_div_uint(op2, &modulo);
    ///     println!("{} / {} = {} (mod {})", op1, op2, res, modulo);
    ///     assert_eq!(res.to_string(), "0");
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), true);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), true);
    ///     assert_eq!(quotient.is_left_carry(), false);
    ///     assert_eq!(quotient.is_right_carry(), false);
    ///     
    ///     for dividend in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
    ///     {
    ///         let rhs = 0_u8;
    ///         let res = dividend.panic_free_modular_div_uint(rhs, &modulo);
    ///         println!("{} / {} = {} (mod {})", dividend, rhs, res, modulo);
    ///         assert_eq!(res.to_string(), "0");
    ///         assert_eq!(res.is_overflow(), false);
    ///         assert_eq!(res.is_underflow(), false);
    ///         assert_eq!(res.is_divided_by_zero(), true);
    ///         assert_eq!(res.is_infinity(), true);
    ///         assert_eq!(res.is_undefined(), true);
    ///         assert_eq!(quotient.is_left_carry(), false);
    ///         assert_eq!(quotient.is_right_carry(), false);
    /// 
    ///         for divisor in [3_u8, 50_u8]
    ///         {
    ///             let res = dividend.panic_free_modular_div_uint(divisor, &modulo);
    ///             println!("{} / {} = {} (mod {})", dividend, divisor, res, modulo);
    ///             assert_eq!(res.to_string(), "0");
    ///             assert_eq!(res.is_overflow(), false);
    ///             assert_eq!(res.is_underflow(), false);
    ///             assert_eq!(res.is_divided_by_zero(), false);
    ///             assert_eq!(res.is_infinity(), false);
    ///             assert_eq!(res.is_undefined(), true);
    ///             assert_eq!(quotient.is_left_carry(), false);
    ///             assert_eq!(quotient.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_div_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally assigns the quotient of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulo` or `modulo` is zero or one.
    /// - If `modulo` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulo` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulo` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulo` then, the quotient will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo`, and `self` is zero or multiple of `modulo` then, the
    ///   quotient will have the value `zero`, and `UNDEFINED` and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo`, and `self` is not zero, and `modulo` is neither zero nor
    ///   one, the quotient will have the max value and `INFINITY`, and
    ///   `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the quotients and the flags will be set as follows:
    /// 
    /// | `modulo` | `rhs`               | `self`              | quotient | flags                          |
    /// |----------|---------------------|---------------------|----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulo`)    | >= 0                | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulo`) | >= 0                | 0        | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulo`)    | 0 (mod `modulo`)    | 0        | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | >= 2     | 0 (mod `modulo`)    | != 0 (mod `modulo`) | max      | `INFINITY`, `DIVIDED_BY_ZERO`  |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_div_assign()](struct@BigUInt#method.panic_free_modular_div_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for a normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 2 for a normal case for modulo >= 2 and self == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::zero();
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 3 for a normal case for modulo >= 2 and self == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_uint(10000_u16);
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 4 for amodulo >= 2 and self != 0 and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 5 for modulo >= 2 and self != 0 and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 6 for modulo >= 2 and self == 0 and divisor == 0
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
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 7 for modulo >= 2 and self == 0 and divisor == multiple of modulo
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
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 8 for modulo >= 2 and self == multiple of modulo and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 9 for modulo >= 2 and self == multiple of modulo and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 10 for modulo == 0 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::zero();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 11 for modulo == 1 and divisor != 0 and dividend != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 12 for modulo == 0 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// // modulo == 0 and divisor == 0 and dividend == 0
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
    /// let divisor = 0_u8;
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 13 for modulo == 1 and divisor == 0 and dividend == 0
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
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 0_u8;
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Collective Example for modulo == 0 or 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// for modulo in [U256::zero(), U256::one()]
    /// {
    ///     let mut dividend = U256::zero();
    ///     println!("Originally, op1 = {}", dividend);
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let divisor = 0_u8;
    ///     dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
    ///     println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), op1 = {}", divisor, modulo, dividend);
    ///     assert_eq!(dividend.to_string(), "0");
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), true);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), true);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
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
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///         let divisor = 0_u8;
    ///         dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
    ///         println!("After op1.panic_free_modular_div_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
    ///         assert_eq!(dividend.to_string(), "0");
    ///         assert_eq!(dividend.is_overflow(), false);
    ///         assert_eq!(dividend.is_underflow(), false);
    ///         assert_eq!(dividend.is_divided_by_zero(), true);
    ///         assert_eq!(dividend.is_infinity(), true);
    ///         assert_eq!(dividend.is_undefined(), true);
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///         for divisor in [3_u8, 50_u8]
    ///         {
    ///             let mut dividend = op.clone();
    ///             println!("Originally, dividend = {}", dividend);
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///             dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
    ///             println!("After dividend.panic_free_modular_div_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
    ///             assert_eq!(dividend.to_string(), "0");
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), true);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_div_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }



    // pub fn wrapping_rem_uint<U>(&self, rhs: U) -> Self
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
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_rem()](struct@BigUInt#method.wrapping_rem)
    /// is proper rather than this method `wrapping_rem_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let remainder = dividend.wrapping_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "8");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let remainder = dividend.wrapping_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let remainder = _dividend.wrapping_rem_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let remainder = _dividend.wrapping_rem_uint(_divisor);
    /// ```
    pub fn wrapping_rem_uint<U>(&self, _rhs: U) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn wrapping_rem_assign_uint(&mut self, rhs: U)
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
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [wrapping_rem_assign()](struct@BigUInt#method.wrapping_rem_assign)
    /// is proper rather than this method `wrapping_rem_assign_uint()`.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// # Example 2
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
    /// let divisor = 87_u8;
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_rem_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = 0_u8;
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// _a_biguint.wrapping_rem_assign_uint(_divisor);
    /// ```
    pub fn wrapping_rem_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_rem_uint<U>(&self, rhs: U) -> (Self, bool)
    /// Divides `self` by `rhs`,
    /// and returns a tuple of the remainder of `self` / `rhs` along with
    /// a boolean indicating whether an arithmetic overflow would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_rem()](struct@BigUInt#method.overflowing_rem)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 87_u8;
    /// let (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 8);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder, 0);
    /// assert_eq!(overflow, false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (remainder, overflow) = _dividend.overflowing_rem_uint(_divisor);
    /// 
    /// let _dividend = UU32::zero();
    /// let _divisor = 0_u8;
    /// // It will panic!
    /// let (remainder, overflow) = _dividend.overflowing_rem_uint(_divisor);
    /// ```
    pub fn overflowing_rem_uint<U>(&self, _rhs: U) -> (U, bool)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn overflowing_rem_assign_uint<U>(&mut self, rhs: U) -> bool
    /// Divides `self` by `rhs`,
    /// and assigns the remainder of `self` / `rhs` to `self` back,
    /// and returns a boolean indicating whether an arithmetic overflow
    /// would occur.
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
    /// If `rhs` is bigger tham `ui128`, the method
    /// [overflowing_rem_assign()](struct@BigUInt#method.overflowing_rem_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// let overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8");
    /// assert_eq!(overflow, false);
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
    /// let overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    /// println!("After a_biguint.overflowing_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(overflow, false);
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
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let _divisor = 0_u16;
    /// println!("Originally,\n_a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_rem_assign_uint(_divisor);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _divisor = 0_u16;
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic!
    /// let overflow = _a_biguint.overflowing_rem_assign_uint(_divisor);
    /// ```
    pub fn overflowing_rem_assign_uint<U>(&mut self, _rhs: U) -> bool
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_rem_uint<U>(&self, rhs: U) -> Self
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
    /// [panic_free_rem_uint()](struct@BigUInt#method.panic_free_rem_uint)
    /// is a bit faster than this method `wrapping_rem()`.
    /// If `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_rem_uint()](struct@BigUInt#method.panic_free_rem_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 87_u8;
    /// let remainder = dividend.panic_free_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
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
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let remainder = dividend.panic_free_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let dividend = UU32::zero();
    /// let divisor = 0_u8;
    /// let remainder = dividend.panic_free_rem_uint(divisor);
    /// println!("{} % {} = {}", dividend, divisor, remainder);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    pub fn panic_free_rem_uint<U>(&self, _rhs: U) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_rem_assign_uint<U>(&mut self, rhs: U)
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
    /// [panic_free_rem_assign()](struct@BigUInt#method.panic_free_rem_assign)
    /// is proper rather than this method `panic_free_rem_assign_uint()`.
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
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let divisor = 87_u8;
    /// a_biguint.wrapping_rem_assign_uint(divisor);
    /// println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 3
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
    /// let divisor = 0_u8;
    /// a_biguint.panic_free_rem_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4
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
    /// let divisor = 0_u8;
    /// a_biguint.panic_free_rem_assign_uint(divisor);
    /// println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_rem_assign_uint<U>(&mut self, _rhs: U)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
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
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the remainder of when (`self` % `modulo`) is divided by
    /// (`rhs` % `modulo`) if (`rhs` % `modulo`) is not zero.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally returns the remainder of `rd1` divided by `rd2`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulo` or `modulo` is zero or one.
    /// - If `modulo` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulo` then, the remainder will have the value `zero` and
    ///   `DIVIDED_BY_ZERO` flag of the remainder will be set.
    /// - If `modulo` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulo` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` flag will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo` then, the remainder will have the value `zero` and
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - In summary, the remainder and the flags will be set as follows:
    /// 
    /// | `modulo` | `rhs`               | remainder | flags                          |
    /// |----------|---------------------|-----------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulo`)    | 0         | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulo`) | 0         | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulo`)    | 0         | `DIVIDED_BY_ZERO`              |
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger than `u128`, the method
    /// [panic_free_modular_rem()](struct@BigUInt#method.panic_free_modular_rem)
    /// is proper rather than this method `panic_free_modular_rem_uint()`.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
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
    /// # Example 2 for modulo >= 2 and dividend == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
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
    /// # Example 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_uint(200_u8);
    /// let divisor = 128_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
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
    /// # Example 4 for modulo >= 2 and dividend != 0 and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for modulo >= 2 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for modulo >= 2 and dividend == multiple of modulo and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_uint(200_u8);
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_uint(200_u8);
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), false);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for modulo == 0 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::zero();
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), true);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 11 for modulo == 1 and divisor == 0 and dividend == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::zero();
    /// let divisor = 0_u8;
    /// let modulo = U256::one();
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), true);
    /// assert_eq!(remainder.is_divided_by_zero(), true);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 12 for modulo == 0 and divisor != 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::zero();
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), true);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 13 for modulo == 1 and divisor != 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    /// let divisor = 128_u8;
    /// let modulo = U256::one();
    /// let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    /// println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    /// assert_eq!(remainder.to_string(), "0");
    /// assert_eq!(remainder.is_overflow(), false);
    /// assert_eq!(remainder.is_underflow(), false);
    /// assert_eq!(remainder.is_infinity(), false);
    /// assert_eq!(remainder.is_undefined(), true);
    /// assert_eq!(remainder.is_divided_by_zero(), false);
    /// assert_eq!(remainder.is_left_carry(), false);
    /// assert_eq!(remainder.is_right_carry(), false);
    /// ```
    /// 
    /// Collective Examples
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// for modulo in [U256::zero(), U256::one()]
    /// {
    ///     let op1 = U256::zero();
    ///     let op2 = 0_u8;
    ///     let res = op1.panic_free_modular_rem_uint(op2, &modulo);
    ///     println!("{} % {} = {} (mod {})", op1, op2, res, modulo);
    ///     assert_eq!(res.to_string(), "0");
    ///     assert_eq!(res.is_overflow(), false);
    ///     assert_eq!(res.is_underflow(), false);
    ///     assert_eq!(res.is_divided_by_zero(), true);
    ///     assert_eq!(res.is_infinity(), false);
    ///     assert_eq!(res.is_undefined(), true);
    ///     assert_eq!(res.is_left_carry(), false);
    ///     assert_eq!(res.is_right_carry(), false);
    ///     
    ///     for dividend in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
    ///     {
    ///         let rhs = 0_u8;
    ///         let res = dividend.panic_free_modular_rem_uint(rhs, &modulo);
    ///         println!("{} % {} = {} (mod {})", dividend, rhs, res, modulo);
    ///         assert_eq!(res.to_string(), "0");
    ///         assert_eq!(res.is_overflow(), false);
    ///         assert_eq!(res.is_underflow(), false);
    ///         assert_eq!(res.is_divided_by_zero(), true);
    ///         assert_eq!(res.is_infinity(), false);
    ///         assert_eq!(res.is_undefined(), true);
    ///         assert_eq!(res.is_left_carry(), false);
    ///         assert_eq!(res.is_right_carry(), false);
    /// 
    ///         for divisor in [3_u8, 50_u8]
    ///         {
    ///             let res = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    ///             println!("{} % {} = {} (mod {})", dividend, divisor, res, modulo);
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
    pub fn panic_free_modular_rem_uint<U>(&self, _rhs: U, _modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - It takes the remainder (= `rd1`) of `self` divided by `modulo`,
    ///   and takes the remainder (= `rd2`) of `rhs` divided by `modulo`,
    ///   and then finally assigns the remainder of `rd1` divided by `rd2`
    ///   back to `self`.
    /// - Overflow will not happen.
    /// - __It does not panic__ even if `rhs` is zero or multiple of
    ///   `modulo` or `modulo` is zero or one.
    /// - If `modulo` is either zero or one, and `rhs` is zero or multiple of
    ///   `modulo` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` and `DIVIDED_BY_ZERO` flags will be set.
    /// - If `modulo` is either zero or one, and `rhs` is not zero nor multiple
    ///   of `modulo` then, `self` will have the value `zero` and its
    ///   `UNDEFINED` flag will be set.
    /// - If `modulo` is greater than one, and `rhs` is either zero or multiple
    ///   of `modulo` then, `self` will have the value `zero` and its
    ///   `DIVIDED_BY_ZERO` flag will be set.
    /// - In summary, `self` and its flags will be set as follows:
    /// 
    /// | `modulo` | `rhs`               | remainder (= `self`) | flags                          |
    /// |----------|---------------------|----------------------|--------------------------------|
    /// | 0 or 1   | 0 (mod `modulo`)    | 0                    | `UNDEFINED`, `DIVIDED_BY_ZERO` |
    /// | 0 or 1   | != 0 (mod `modulo`) | 0                    | `UNDEFINED`                    |
    /// | >= 2     | 0 (mod `modulo`)    | 0                    | `DIVIDED_BY_ZERO`              |
    /// 
    /// - All the flags are historical, which means, for example, if an
    ///   divided_by_zero occurred even once before this current operation or
    ///   `DIVIDED_BY_ZERO` flag is already set before this current operation,
    ///   the `DIVIDED_BY_ZERO` flag is not changed even if this current operation
    ///   does not cause divided_by_zero.
    /// 
    /// # Counterpart Method
    /// If `rhs` is bigger tham `ui128`, the method
    /// [panic_free_modular_rem_assign()](struct@BigUInt#method.panic_free_modular_rem_assign)
    /// is proper rather than this method.
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
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
    /// # Example 2 for modulo >= 2 and self == 0 and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let divisor = 128_u8;
    /// let modulo = UU32::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
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
    /// # Example 3 for modulo >= 2 and self == multiple of modulo and divisor != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 4 for modulo >= 2 and self != 0 and divisor == 0  
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for modulo >= 2 and self != 0 and divisor == multiple of modulo
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
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for modulo >= 2 and self == 0 and divisor == 0
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for modulo >= 2 and self == 0 and divisor == multiple of modulo
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
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8 for modulo >= 2 and self == multiple of modulo and divisor == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// let divisor = 0_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9 for modulo >= 2 and self == multiple of modulo and divisor == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// let divisor = 200_u8;
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), true);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 10 for modulo == 0 and self != 0 and divisor != 0
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
    /// let divisor = 128_u8;
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    /// # Example 11 for modulo >= 2 and self != 0 and divisor == 0
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
    /// let divisor = 128_u8;
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 12 for modulo == 0 and divisor == 0 and self == 0
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
    /// let divisor = 0_u8;
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Example 13 for modulo == 1 and divisor == 0 and self == 0
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
    /// let divisor = 0_u8;
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    /// println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
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
    /// # Collective Examples for modulo == 0 or 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// for modulo in [U256::zero(), U256::one()]
    /// {
    ///     let mut dividend = U256::zero();
    ///     println!("Originally, op1 = {}", dividend);
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), false);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///     let divisor = 0_u8;
    ///     dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
    ///     println!("After a_biguint.panic_free_modular_rem_assign_uint({}, {}), op1 = {}", divisor, modulo, dividend);
    ///     assert_eq!(dividend.to_string(), "0");
    ///     assert_eq!(dividend.is_overflow(), false);
    ///     assert_eq!(dividend.is_underflow(), false);
    ///     assert_eq!(dividend.is_divided_by_zero(), true);
    ///     assert_eq!(dividend.is_infinity(), false);
    ///     assert_eq!(dividend.is_undefined(), true);
    ///     assert_eq!(a_biguint.is_left_carry(), false);
    ///     assert_eq!(a_biguint.is_right_carry(), false);
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
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///         let divisor = 0_u8;
    ///         dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
    ///         println!("After op1.panic_free_modular_rem_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
    ///         assert_eq!(dividend.to_string(), "0");
    ///         assert_eq!(dividend.is_overflow(), false);
    ///         assert_eq!(dividend.is_underflow(), false);
    ///         assert_eq!(dividend.is_divided_by_zero(), true);
    ///         assert_eq!(dividend.is_infinity(), false);
    ///         assert_eq!(dividend.is_undefined(), true);
    ///         assert_eq!(a_biguint.is_left_carry(), false);
    ///         assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///         for divisor in [3_u8, 50_u8]
    ///         {
    ///             let mut dividend = op.clone();
    ///             println!("Originally, dividend = {}", dividend);
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    ///             dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
    ///             println!("After dividend.panic_free_modular_rem_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
    ///             assert_eq!(dividend.to_string(), "0");
    ///             assert_eq!(dividend.is_overflow(), false);
    ///             assert_eq!(dividend.is_underflow(), false);
    ///             assert_eq!(dividend.is_divided_by_zero(), false);
    ///             assert_eq!(dividend.is_infinity(), false);
    ///             assert_eq!(dividend.is_undefined(), true);
    ///             assert_eq!(a_biguint.is_left_carry(), false);
    ///             assert_eq!(a_biguint.is_right_carry(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_rem_assign_uint<U>(&mut self, _rhs: U, _modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd
    {
        unimplemented!(); // Dummy code for documentation
    }
}