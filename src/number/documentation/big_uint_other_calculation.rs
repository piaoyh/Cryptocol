#[allow(unused_variables)]
#[allow(dead_code)]

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
    /***** MULTIPLE OPERATIONS WITH BIGUINT *****/

    // pub fn next_multiple_of(&self, rhs: &Self) -> Self
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
    /// The method
    /// [next_multiple_of_uint()](struct@BigUInt#method.next_multiple_of_uint)
    /// is a bit faster than this method `next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [next_multiple_of_uint()](struct@BigUInt#method.next_multiple_of_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::max();
    /// let num = U256::from(586478_u32);
    /// let multiple = a_biguint.next_multiple_of(&num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "448670");
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = U256::zero();
    /// // It will panic.
    /// let _multiple = _a_biguint.next_multiple_of(&_num);
    /// ```
    pub fn next_multiple_of(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn next_multiple_of_assign(&mut self, rhs: &Self)
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
    /// The method
    /// [next_multiple_of_assign_uint()](struct@BigUInt#method.next_multiple_of_assign_uint)
    /// is a bit faster than this method `next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [next_multiple_of_assign_uint()](struct@BigUInt#method.next_multiple_of_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// let num = UU32::from(586478_u32);
    /// a_biguint.next_multiple_of_assign(&num);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "448670");
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let _num = UU32::zero();
    /// // It will panic.
    /// _a_biguint.next_multiple_of_assign(&_num);
    /// ```
    pub fn next_multiple_of_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_next_multiple_of(&self, rhs: &Self) -> Self
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
    /// [panic_free_next_multiple_of_uint()](struct@BigUInt#method.panic_free_next_multiple_of_uint)
    /// is a bit faster than this method `panic_free_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_next_multiple_of_uint()](struct@BigUInt#method.panic_free_next_multiple_of_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::max();
    /// let num = U256::from(586478_u32);
    /// let multiple = a_biguint.panic_free_next_multiple_of(&num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "448670");
    /// assert_eq!(multiple.is_overflow(), true);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), false);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::zero();
    /// let multiple = a_biguint.panic_free_next_multiple_of(&num);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    pub fn panic_free_next_multiple_of(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_next_multiple_of_assign(&mut self, rhs: &Self)
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
    /// [panic_free_next_multiple_of_assign_uint()](struct@BigUInt#method.panic_free_next_multiple_of_assign_uint)
    /// is a bit faster than this method `panic_free_next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_next_multiple_of_assign_uint()](struct@BigUInt#method.panic_free_next_multiple_of_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::max();
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
    /// assert_eq!(a_biguint.to_string(), "448670");
    /// assert_eq!(a_biguint.is_overflow(), true);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = UU32::zero();
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
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_next_multiple_of_assign(&mut self, _rhs: &Self)
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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

    // pub fn panic_free_modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// - It returns the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulo`. So,
    ///   if overflow occurs, it returns the value wrapped around at `modulo`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulo` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// 
    /// # Feature
    /// - Wrapping (modular) arround at `modulo`.
    /// - If `rhs` is zero, it returns `zero` and the `UNDEFINED` flag
    ///   of the return value will be set.
    /// - If `modulo` is either `zero` or `one`, it returns `zero` and
    ///   the `UNDEFINED` flag of the return value will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of()` and the method
    ///   `panic_free_next_multiple_of()` are, first, where wrapping around
    ///   happens, and, second, when `OVERFLOW` flag is set.
    ///   First, this method wraps around at `modulo` while the method
    ///   `panic_free_next_multiple_of()` wraps around at `maximum value + 1`.
    ///   Second, this method sets `OVERFLOW` flag when wrapping around happens
    ///   at `modulo` while the method `panic_free_next_multiple_of()` sets
    ///   `OVERFLOW` flag when wrapping around happens at `maximum value + 1`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [panic_free_modular_next_multiple_of_uint()](struct@BigUInt#method.panic_free_modular_next_multiple_of_uint)
    /// is a bit faster than this method `panic_free_modular_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_next_multiple_of_uint()](struct@BigUInt#method.panic_free_modular_next_multiple_of_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::max();
    /// let num = U256::from(100_u8);
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
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
    /// # Example 3 for rhs == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::zero();
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for rhs == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(200_u8);
    /// let modulo = U256::from(100_u8);
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for modulo == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(100_u8);
    /// let modulo = U256::zero();
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6 for modulo == 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::from(100_u8);
    /// let modulo = U256::one();
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for rhs == 0 and modulo == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::zero();
    /// let modulo = U256::zero();
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7 for rhs == 0 and modulo == 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let num = U256::zero();
    /// let modulo = U256::one();
    /// let multiple = a_biguint.panic_free_modular_next_multiple_of(&num, &modulo);
    /// println!("The next multiple of {} is {}", a_biguint, multiple);
    /// assert_eq!(multiple.to_string(), "0");
    /// assert_eq!(multiple.is_overflow(), false);
    /// assert_eq!(multiple.is_underflow(), false);
    /// assert_eq!(multiple.is_infinity(), false);
    /// assert_eq!(multiple.is_divided_by_zero(), false);
    /// assert_eq!(multiple.is_undefined(), true);
    /// assert_eq!(multiple.is_left_carry(), false);
    /// assert_eq!(multiple.is_right_carry(), false);
    /// ```
    pub fn panic_free_modular_next_multiple_of(&self, _rhs: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self)
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) arround at `modulo`.
    /// - `self` will be the smallest value greater than or equal to `self`,
    ///   which is a multiple of `rhs`, wrapping around at `modulo`. So, if
    ///   overflow occurs, `self` will be the value wrapped around at `modulo`.
    /// - If `rhs` is zero, it assigns `zero` to `self` back
    ///   and the `UNDEFINED` flag of `self` will be set.
    /// - If `modulo` is either `zero` or `one`, it assigns `zero`
    ///   to `self` back and the `UNDEFINED` flag of `self` will be set.
    /// - The differences between this method
    ///   `panic_free_modular_next_multiple_of_assign()`
    ///   and the method `panic_free_next_multiple_of_assign()` are, first,
    ///   where wrapping around happens, and, second, when `OVERFLOW` flag is
    ///   set. First, this method wraps araound at `modulo` while the method
    ///   `panic_free_next_multiple_of_assign()` wraps araound at `maximum
    ///   value + 1`. Second, this method set `OVERFLOW` flag when wrapping
    ///   around happens at `modulo` while the method
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
    /// [panic_free_modular_next_multiple_of_assign_uint()](struct@BigUInt#method.panic_free_modular_next_multiple_of_assign_uint)
    /// is a bit faster than this method `panic_free_modular_next_multiple_of_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [panic_free_modular_next_multiple_of_assign_uint()](struct@BigUInt#method.panic_free_modular_next_multiple_of_assign_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
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
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
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
    /// # Example 3 for rhs == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::zero();
    /// let modulo = a_biguint.wrapping_add_uint(200_u8);
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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
    /// # Example 4 for rhs == multiple of modulo
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::from(200_u8);
    /// let modulo = U256::from(100_u8);
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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
    /// # Example 5 for modulo == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::from(100_u8);
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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
    /// # Example 6 for modulo == 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::from(100_u8);
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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
    /// # Example 7 for rhs == 0 and modulo == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::zero();
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
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
    /// # Example 8 for rhs == 0 and modulo == 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
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
    /// let num = U256::zero();
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_next_multiple_of_assign(&num, &modulo);
    /// println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_modular_next_multiple_of_assign(&mut self, _rhs: &Self, _modulo: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn is_multiple_of(&self, rhs: &Self) -> bool
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
    /// [is_next_multiple_of_uint()](struct@BigUInt#method.is_next_multiple_of_uint)
    /// is a bit faster than this method `is_next_multiple_of()`.
    /// So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    /// u32, u64, and u128, use the method
    /// [is_next_multiple_of_uint()](struct@BigUInt#method.is_next_multiple_of_uint).
    /// 
    /// # Example 1 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::from(100_u8);
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    /// 
    /// # Example 2 for Normal case
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::from(99_u8);
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 3 for rhs == 0 and self != 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    /// let rhs = U256::zero();
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, false);
    /// ```
    /// 
    /// # Example 4 for rhs == 0 and self == 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let rhs = U256::zero();
    /// let ans = a_biguint.is_multiple_of(&rhs);
    /// println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    /// assert_eq!(ans, true);
    /// ```
    pub fn is_multiple_of(&self, _rhs: &Self) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }



    /***  MIDDLE POINT ***/

    // pub fn midpoint(&self, rhs: &Self) -> Self
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
    /// The method [midpoint_uint()](struct@BigUInt#method.midpoint_uint)
    /// is more efficient than this method `midpoint()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_uint()](struct@BigUInt#method.midpoint_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    /// let c_biguint = a_biguint.midpoint(&b_biguint);
    /// println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// assert_eq!(c_biguint.is_left_carry(), false);
    /// assert_eq!(c_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn midpoint(&self, _rhs: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

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
    /// The method [midpoint_assign_uint()](struct@BigUInt#method.midpoint_assign_uint)
    /// is more efficient than this method `midpoint_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `rhs` is the primitive unsigned integral data type number,
    /// use the method [midpoint_assign_uint()](struct@BigUInt#method.midpoint_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for the case that self is even number and rhs is even number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 3 for the case that self is even number and rhs is odd number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 4 for the case that self is odd number and rhs is even number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5 for the case that self is odd number and rhs is odd number
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
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
    /// let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    /// a_biguint.midpoint_assign(&b_biguint);
    /// println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    #[inline]
    pub fn midpoint_assign(&mut self, _rhs: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }



    /*** METHODS FOR PRIME NUMBER TEST ***/

    // pub fn is_prime_using_miller_rabin(&self, _repetition: usize) -> bool
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    /// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    /// assert_eq!(b, true);
    /// ```
    /// 
    /// # Example 2 for composite number case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b = a_biguint.is_prime_using_miller_rabin(5_usize);
    /// println!("{} is {}a prime number", a_biguint, if b {""} else {"not "});
    /// assert_eq!(b, false);
    /// ```
    pub fn is_prime_using_miller_rabin(&self, _repetition: usize) -> bool
    {
        unimplemented!(); // Dummy code for documentation
    }
}