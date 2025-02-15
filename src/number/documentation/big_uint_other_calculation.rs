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

    // pub fn panic_free_pow(&mut self, exp: &Self) -> Self
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
    /// The method [panic_free_pow_uint()](struct@BigUInt#method.panic_free_pow_uint)
    /// is more efficient than this method `panic_free_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_pow_uint()](struct@BigUInt#method.panic_free_pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for wrapping (modular) exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.panic_free_pow(&exp);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.panic_free_pow(&exp);
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
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.panic_free_pow(&exp);
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
    /// # Example 5 for 0 ** 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::zero();
    /// let res = a_biguint.panic_free_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn panic_free_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_pow_assign(&mut self, exp: &Self) -> Self
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
    /// The method [panic_free_pow_assign_uint()](struct@BigUInt#method.panic_free_pow_assign_uint)
    /// is more efficient than this method `panic_free_pow_assign()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number, use
    /// the method [pow_assign_uint()](struct@BigUInt#method.pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2 for wrapping (modular) exponentiation
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// let exp = U256::from_uint(100_u8);
    /// a_biguint.panic_free_pow_assign(&exp);
    /// println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_pow_assign(&exp);
    /// println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// a_biguint.panic_free_pow_assign(&exp);
    /// println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// # Example 5
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
    /// let exp = U256::zero();
    /// a_biguint.panic_free_pow_assign(&exp);
    /// println!("After a_biguint.panic_free_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(),  true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_pow_assign(&mut self, _exp: &Self)
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

    // pub fn checked_pow(&self, exp: &Self) -> Option<Self>
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// wrapping around at the boundary of the type `Self`,
    /// and returns the result wrapped by `Some` of enum `Option`.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is of `&Self` type.
    /// 
    /// # Output
    /// - It returns the result of `self` raised to the power of `exp`, using
    ///   exponentiation of type `BigUInt` by squaring,
    ///   wrapping around at the boundary of the type `Self`,
    ///   wrapped by `Some` of enum `Option` if overflow does not occur.
    /// - If overflow occurs, it returns `None` of enum `Option`.
    /// - If both `self` and `rhs` are zero, the result is mathematically
    ///   undefined so this method returns `None`.
    /// 
    /// # Features
    /// - Checked wrapping (modular) exponentiation. 
    /// - If overflowing happens, it returns `None` of enum `Option`.
    /// - If both `self` and `rhs` are zero, the result is mathematically
    ///   undefined so this method returns `None`.
    /// 
    /// # Counterpart Method
    /// The method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint) is a bit
    /// faster than this method `checked_pow()` when the exponent `exp` is
    /// primitive unsigned integral data type such as u8, u16, u32, u64, and
    /// u128. If `exp` is the primitive unsigned integral data type number,
    /// use the method
    /// [checked_pow_uint()](struct@BigUInt#method.checked_pow_uint).
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1000000000000000000000000000000");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Overflow");
    ///             assert_eq!(res, None);
    ///         },
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
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "0");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
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
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => {
    ///             println!("{} ** {} = {}", a_biguint, exp, raised);
    ///             assert_eq!(raised.to_string(), "1");
    ///             assert_eq!(raised.is_overflow(), false);
    ///             assert_eq!(raised.is_underflow(), false);
    ///             assert_eq!(raised.is_infinity(), false);
    ///             assert_eq!(raised.is_divided_by_zero(), false);
    ///             assert_eq!(raised.is_undefined(), false);
    ///             assert_eq!(raised.is_left_carry(), false);
    ///             assert_eq!(raised.is_right_carry(), false);
    ///         },
    ///     None => { println!("Overflow"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::zero();
    /// let res = a_biguint.checked_pow(&exp);
    /// match res
    /// {
    ///     Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
    ///     None => {
    ///             println!("Undefined");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_pow(&self, _exp: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_pow(&self, exp: &Self) -> Self
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
    /// - If overflow occurs, it will panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Output
    /// It returns the result of `self` raised to the power of `exp`.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - If overflowing happens, this method will panic.
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10_u8);
    /// let exp = U256::from_uint(30_u8);
    /// let res = a_biguint.unchecked_pow(&exp);
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
    /// # Example 2 for 123456789012345678901234567890123456789 ** 0
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = U256::zero();
    /// let res = a_biguint.unchecked_pow(&exp);
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
    /// # Example 3 for 0 ** 123456789012345678901234567890123456789
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.unchecked_pow(&exp);
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
    /// use std::str::FromStr;
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::from_uint(10_u8);
    /// let _exp = U256::from_uint(100_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow(&_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_pow(&_exp);
    /// ```
    pub fn unchecked_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow(&self, exp: &Self) -> Self
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
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
    /// - Overflowing never happens.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag of the return value.
    /// 
    /// # Counterpart Method
    /// The method [saturating_pow_uint()](struct@BigUInt#method.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [pow_uint()](struct@BigUInt#method.pow_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let res = a_biguint.saturating_pow(&exp);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let res = a_biguint.saturating_pow(&exp);
    /// println!("{} ** {} = {}", a_biguint, exp, res);
    /// assert_eq!(res, UU32::max());
    /// assert_eq!(res.is_overflow(), false);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let exp = UU32::zero();
    /// let res = a_biguint.saturating_pow(&exp);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    /// let res = a_biguint.saturating_pow(&exp);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.saturating_pow(&_exp);
    /// ```
    pub fn saturating_pow(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn saturating_pow_assign(&mut self, exp: &Self)
    /// Raises `BigUInt` type number to the power of `exp`, using
    /// exponentiation of type `BigUInt` by squaring,
    /// saturating at the numeric bounds instead of overflowing,
    /// and assign the result to `self` back.
    /// 
    /// # Arguments
    /// `exp` is the power to raise `self` to, and is a primitive unsigned
    /// integer such as `u8`, `u16`, `u32`, `u64`, and `u128`.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - If both `self` and `exp` are zero, the result is mathematically
    ///   undefined, so this method will panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation.
    /// - Overflowing never happens.
    /// - `self` will be the maximum value instead of overflowing.
    /// - This method saturates when it reaches maximum value.
    /// - It does not set `OVERFLOW` flag.
    /// 
    /// # Counterpart Method
    /// The method [saturating_pow_assign_uint()](struct@BigUInt#method.saturating_pow_uint)
    /// is more efficient than this method `saturating_pow_assign()` when the exponent
    /// `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [saturating_pow_assign_uint()](struct@BigUInt#method.saturating_pow_assign_uint).
    /// 
    /// # Example 1 for normal exponentiation
    /// ```
    /// use cryptocol::number::BigUint_More;
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
    /// a_biguint.saturating_pow_assign(&exp);
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
    /// use cryptocol::number::BigUint_More;
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
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
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
    /// use cryptocol::number::BigUint_More;
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
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// use cryptocol::number::BigUint_More;
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
    /// a_biguint.saturating_pow_assign(&exp);
    /// println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    /// use cryptocol::number::BigUint_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// println!("Originally, a_biguint = {}", _a_biguint);
    /// // It will panic.
    /// _a_biguint.saturating_pow_assign(&_exp);
    /// ```
    pub fn saturating_pow_assign(&mut self, _exp: &Self)
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

    // pub fn panic_free_modular_pow(&self, exp: &Self, modulo: &Self) -> Self
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
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
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | `self` | `exp` | return value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_modular_pow_uint()](struct@BigUInt#method.panic_free_modular_pow_uint)
    /// is more efficient than this method `panic_free_modular_pow()` when the
    /// exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_modular_pow_uint()](struct@BigUInt#method.panic_free_modular_pow_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "59749648429786538521694772865754025520");
    /// assert_eq!(res.is_overflow(), true);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::zero();
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(2000_u16);
    /// let modulo = UU32::from_uint(1000_u16);
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "1");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(3000_u16);
    /// let exp = UU32::from_uint(30_u8);
    /// let modulo = UU32::from_uint(1000_u16);
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), false);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for self == 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::zero();
    /// let modulo = UU32::halfmax();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for self == 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::from_uint(2000_u16);
    /// let modulo = UU32::from_uint(1000_u16);
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for self == multiple of modulo and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(3000_u16);
    /// let exp = UU32::zero();
    /// let modulo = UU32::from_uint(1000_u16);
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(3000_u16);
    /// let exp = UU32::from_uint(2000_u16);
    /// let modulo = UU32::from_uint(1000_u16);
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 11 for self != 0 and exp != 0 and modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let modulo = UU32::zero();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 12 for self != 0 and exp != 0 and modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::from_uint(10_u8);
    /// let exp = UU32::from_uint(100_u8);
    /// let modulo = UU32::one();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 13 for self == 0 and exp == 0 and modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = UU32::zero();
    /// let exp = UU32::zero();
    /// let modulo = UU32::zero();
    /// let res = a_biguint.panic_free_modular_pow(&exp, &modulo);
    /// println!("{} ** {} = {} (mod {})", a_biguint, exp, res, modulo);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Ccollective Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// for modulo in [UU32::zero(), UU32::one()]
    /// {
    ///     for lhs in [UU32::zero(), UU32::from_uint(50_u8)]
    ///     {
    ///         for rhs in [UU32::zero(), UU32::from_uint(50_u8)]
    ///         {
    ///             let res = lhs.panic_free_modular_pow(&rhs, &modulo);
    ///             println!("{} ** {} = {} (mod {})", lhs, rhs, res, modulo);
    ///             assert_eq!(res.to_string(), "0");
    ///             assert_eq!(res.is_overflow(), false);
    ///             assert_eq!(res.is_underflow(), false);
    ///             assert_eq!(res.is_divided_by_zero(), false);
    ///             assert_eq!(res.is_infinity(), false);
    ///             assert_eq!(res.is_undefined(), true);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_pow(&self, _exp: &Self, _modulo: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
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
    /// If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Features
    /// - Wrapping (modular) exponentiation,
    ///   wrapping around at `modulo` of the `Self` type`.
    /// - All the flags are historical, which means, for example, if an
    ///   overflow occurred even once before this current operation or
    ///   `OVERFLOW` flag is already set before this current operation,
    ///   the `OVERFLOW` flag is not changed even if this current operation
    ///   does not cause overflow.
    /// - If `modulo` is either `zero` or `one`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - If both `self` and `exp` are `zero`, the `UNDEFINED` flag of the
    ///   return value will be set and the return value will have the value `0`.
    /// - In summary, the return value and its flags will be set as follows:
    /// 
    /// | `modulo` | `self` | `exp` | result value | flags       |
    /// |----------|--------|-------|--------------|-------------|
    /// | 0 or 1   | >= 0   | >= 0  | 0            | `UNDEFINED` |
    /// | > 1      | 0      | 0     | 0            | `UNDEFINED` |
    /// 
    /// # Counterpart Method
    /// The method [panic_free_modular_pow_assign_uint()](struct@BigUInt#method.panic_free_modular_pow_assign_uint)
    /// is more efficient than this method `panic_free_modular_pow_assign()`
    /// when the exponent `exp` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `exp` is the primitive unsigned integral data type number,
    /// use the method [panic_free_modular_pow_assign_uint()](struct@BigUInt#method.panic_free_modular_pow_assign_uint).
    /// 
    /// # Example 1 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 2 for Noraml case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(1000000000000000000000000000000_u128);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(100_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "52266245075570873327294567809656160090");
    /// assert_eq!(a_biguint.is_overflow(), true);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for self != 0 and exp == 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::zero();
    /// let modulo = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for self != 0 and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(200_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for self == 0 and exp != 0 and modulo != 0
    /// ```
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
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for self == multiple of modulo and exp != 0 and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(300_u16);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(30_u8);
    /// let modulo = U256::from_uint(100_u8);
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for self == 0 and exp == 0 and modulo != 0
    /// ```
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
    /// 
    /// let exp = U256::zero();
    /// let modulo = U256::halfmax();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for self == multiple of modulo and exp == multiple of modulo and modulo != 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(200_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(150_u8);
    /// let modulo = U256::from_uint(50_u8);
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for self != 0 and exp != 0 and modulo == 0
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(100_u8);
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for self != 0 and exp != 0 and modulo == 1
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_uint(10_u8);
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let exp = U256::from_uint(100_u8);
    /// let modulo = U256::one();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 11 for self == 0 and exp == 0 and modulo == 0
    /// ```
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
    /// 
    /// let exp = U256::zero();
    /// let modulo = U256::zero();
    /// a_biguint.panic_free_modular_pow_assign(&exp, &modulo);
    /// println!("After a_biguint.panic_free_modular_pow_assign_uint({}, {}), a_biguint = {}", exp, modulo, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Ccollective Examples
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// for modulo in [U256::zero(), U256::one()]
    /// {
    ///     for lhs in [U256::zero(), U256::from_uint(50_u8)]
    ///     {
    ///         for rhs in [U256::zero(), U256::from_uint(50_u8)]
    ///         {
    ///             let mut lhs = lhs.clone();
    ///             println!("Originally, lhs = {}", lhs);
    ///             assert_eq!(lhs.is_overflow(), false);
    ///             assert_eq!(lhs.is_underflow(), false);
    ///             assert_eq!(lhs.is_infinity(), false);
    ///             assert_eq!(lhs.is_undefined(), false);
    ///             assert_eq!(lhs.is_divided_by_zero(), false);
    /// 
    ///             lhs.panic_free_modular_pow_assign(&rhs, &modulo);
    ///             println!("After lhs.panic_free_modular_pow_assign({}, {}), lhs = {}", rhs, modulo, lhs);
    ///             assert_eq!(lhs.to_string(), "0");
    ///             assert_eq!(lhs.is_overflow(), false);
    ///             assert_eq!(lhs.is_underflow(), false);
    ///             assert_eq!(lhs.is_infinity(), false);
    ///             assert_eq!(lhs.is_undefined(), true);
    ///             assert_eq!(lhs.is_divided_by_zero(), false);
    ///         }
    ///     }
    /// }
    /// ```
    pub fn panic_free_modular_pow_assign(&mut self, _exp: &Self, _modulo: &Self)
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

    // pub fn panic_free_iroot(&self, exp: &Self) -> Self
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
    /// [panic_free_iroot_uint()](struct@BigUInt#method.panic_free_iroot_uint)
    /// is a bit faster than this method `panic_free_iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [panic_free_iroot_uint()](struct@BigUInt#method.panic_free_iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.panic_free_iroot(&exp);
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
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::zero();
    /// let res = a_biguint.panic_free_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res, U256::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), true);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::one();
    /// let exp = U256::zero();
    /// let res = a_biguint.panic_free_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::zero();
    /// let res = a_biguint.panic_free_iroot(&exp);
    /// println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn panic_free_iroot(&self, _exp: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_iroot_assign(&mut self, exp: &Self)
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
    /// [panic_free_iroot_assign_uint()](struct@BigUInt#method.panic_free_iroot_assign_uint)
    /// is a bit faster than this method `panic_free_iroot_assign()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [panic_free_iroot_assign_uint()](struct@BigUInt#method.panic_free_iroot_assign_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let exp = U256::from_uint(65_u8);
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// define_utypes_with!(u64);
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
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// let exp = U256::from_uint(6_u8);
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
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
    /// let exp = U256::zero();
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::one();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::zero();
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
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
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::zero();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let exp = U256::zero();
    /// a_biguint.panic_free_iroot_assign(&exp);
    /// println!("After a_biguint.panic_free_iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_iroot_assign(&mut self, _exp: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_iroot(&self, exp: &Self) -> Option<Self>
    /// Calculates the `exp`-th root of `self`, rounded down,
    /// and returns the result value, wrapped by `Some` of enum `Option`.
    ///
    /// # Arguments
    /// `exp` is the power of the root of `self`, and is of `&Self` type.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// If the exact value of `exp`-th root of `self` can be expressed with
    /// `Self`-typed unsigned integer, it will be returned wrapped by `Some`
    /// of enum `Option`.
    /// Otherwise, the `Self`-typed biggest unsigned integer that is
    /// less than the exact value of `exp`-th root of `self` will be returned
    /// wrapped by `Some` of enum `Option`.
    /// 
    /// # Features
    /// - If `exp` is greater than zero and `self` is greater than 1,
    ///   the result of this method is never greater than `self`.
    ///   So, this method never causes overflow.
    /// - If `exp` is `0`, this method returns `None`.
    /// 
    /// # Counterpart Method
    /// If `exp` is bigger than `u128`, the method
    /// [checked_iroot_uint()](struct@BigUInt#method.checked_iroot_uint)
    /// is proper rather than this method.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "100000000");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "9");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "1");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); }
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The {}-th root of {} is {}.", exp, a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::one();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::zero();
    /// let res = a_biguint.checked_iroot(&exp);
    /// match res
    /// {
    ///     Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_iroot(&self, _exp: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_iroot(&self, exp: &Self) -> Self
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
    /// [unchecked_iroot_uint()](struct@BigUInt#method.unchecked_iroot_uint)
    /// is a bit faster than this method `unchecked_iroot()`.
    /// So, if `rhs` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128, use the method
    /// [unchecked_iroot_uint()](struct@BigUInt#method.unchecked_iroot_uint).
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(8_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(65_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(212_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(213_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let exp = U256::from_uint(6_u8);
    /// let res = a_biguint.unchecked_iroot(&exp);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// 
    /// let _a_biguint = U256::one();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _exp = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_iroot(&_exp);
    /// ```
    pub fn unchecked_iroot(&self, _exp: &Self) -> Self
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

    // pub fn ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Output
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [ilog2()](struct@BigUInt#method.ilog2)
    /// can produce results more efficiently for base 2, and
    /// [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.ilog(&base);
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "64");
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
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
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
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::from_uint(6_u8);
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let _a_biguint = U256::one();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// 
    /// let _a_biguint = U256::one();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog(&_base);
    /// ```
    pub fn ilog(&self, _base: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn ilog_assign(&mut self, base: &Self)
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and assigns the result back to `self`.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [ilog2_assign()](struct@BigUInt#method.ilog2_assign)
    /// can produce results more efficiently for base 2, and
    /// [ilog10_assign()](struct@BigUInt#method.ilog10_assign)
    /// can produce results more efficiently for base 10.
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
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// a_biguint.ilog_assign(&base);
    /// println!("After a_biguint.ilog_assign({}), a_biguint = {}.", base, a_biguint);
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
    /// let base = U256::from_uint(10_u8);
    /// a_biguint.ilog_assign(&base);
    /// println!("After a_biguint.ilog_assign({}), a_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "64");
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
    /// define_utypes_with!(u16);
    /// 
    /// let mut a_biguint = U256::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let base = U256::from_uint(6_u8);
    /// a_biguint.ilog_assign(&base);
    /// println!("After a_biguint.ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::from_uint(6_u8);
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::one();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// 
    /// let mut _a_biguint = U256::one();
    /// println!("Originally, _a_biguint = {}", _a_biguint);
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.ilog_assign(&_base);
    /// ```
    pub fn ilog_assign(&mut self, _base: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_ilog(&self, base: &Self) -> Self
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
    /// [panic_free_ilog2()](struct@BigUInt#method.panic_free_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10()](struct@BigUInt#method.panic_free_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "64");
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
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
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
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::zero();
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res, U256::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), true);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::one();
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res, U256::max());
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), true);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::zero();
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::one();
    /// let res = a_biguint.panic_free_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog(&self, _base: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_ilog_assign(&mut self, base: &Self)
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
    /// [panic_free_ilog2_assign()](struct@BigUInt#method.panic_free_ilog2_assign)
    /// can produce results more efficiently for base 2, and
    /// [panic_free_ilog10_assign()](struct@BigUInt#method.panic_free_ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let base = U256::from_uint(10_u8);
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "64");
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
    /// let mut a_biguint = U256::one();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// 
    /// let base = U256::from_uint(6_u8);
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let base = U256::zero();
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
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
    /// let base = U256::one();
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint, U256::max());
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), true);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Example 6
    /// ```
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
    /// let base = U256::from_uint(6_u8);
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    /// # Example 7
    /// ```
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
    /// let base = U256::zero();
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
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
    /// # Example 8
    /// ```
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
    /// let base = U256::one();
    /// a_biguint.panic_free_ilog_assign(&base);
    /// println!("After a_biguint.panic_free_ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog_assign(&mut self, _base: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`, rounded
    /// down, and returns the result wrapped with enum `Some` of `Option`.
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
    /// - It returns the logarithm of the number with respect to `base`,
    ///   rounded down, wrapped with enum `Some` of `Option`.
    /// - It returns `None` if `self` is zero.
    /// - It returns `None` if `base` is either `0` or `1`.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [checked_ilog2()](#tymethod.checked_ilog2) can produce
    /// results more efficiently for base 2, and
    /// [checked_ilog10()](#tymethod.checked_ilog10) can produce
    /// results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "2");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "64");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::zero();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::one();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::zero();
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = U256::one();
    /// let res = a_biguint.checked_ilog(&base);
    /// match res
    /// {
    ///     Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog(&self, _base: &Self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog(&self, base: &Self) -> Self
    /// Calculates the logarithm of the number with respect to `base`,
    /// rounded down, and returns the result.
    ///
    /// # Arguments
    /// `base` is the base of logarithm of `self`, and is of `Self` type.
    /// `base` should be greater than 1.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// - This function will panic if `base` is zero or one.
    ///
    /// # Output
    /// It returns the logarithm of the number with respect to `base`,
    /// rounded down.
    ///
    /// # Counterpart Methods
    /// This method might not be optimized owing to implementation details.
    /// [unchecked_ilog2()](#tymethod.unchecked_ilog2)
    /// can produce results more efficiently for base 2, and
    /// [unchecked_ilog10()](#tymethod.unchecked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    /// let res = a_biguint.unchecked_ilog(&base);
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
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let base = U256::from_uint(10_u8);
    /// let res = a_biguint.unchecked_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    /// assert_eq!(res.to_string(), "64");
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::one();
    /// let base = U256::from_uint(6_u8);
    /// let res = a_biguint.unchecked_ilog(&base);
    /// println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::from_uint(6_u8);
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _base = U256::one();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog(&_base);
    /// ```
    pub fn unchecked_ilog(&self, _base: &Self) -> Self
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

    // pub fn panic_free_ilog2(&self) -> Self
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
    /// [ilog_uint()](struct@BigUInt#method.ilog_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10()](struct@BigUInt#method.ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.panic_free_ilog2();
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
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.panic_free_ilog2();
    /// println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog2(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_ilog2_assign(&mut self)
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
    /// [ilog_assign_uint()](struct@BigUInt#method.ilog_assign_uint)
    /// can produce results of the base other than 2, and
    /// [ilog10_assign()](struct@BigUInt#method.ilog10_assign)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
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
    /// define_utypes_with!(u8);
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
    /// # Example 4
    /// ```
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
    /// a_biguint.panic_free_ilog2_assign();
    /// println!("After a_biguint.panic_free_ilog2_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog2_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog2(&self) -> Option<Self>
    /// Calculates the base 2 logarithm of the number.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down,
    /// wrapped by `Some` of enum `Option` if `self` is not zero.
    /// It returns `None` if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [checked_ilog_uint()](#tymethod.checked_ilog_uint)
    /// can produce results of the base other than 2, and
    /// [checked_ilog10()](#tymethod.checked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The base 10 logarithm of {} is {}.", a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog2(&self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog2(&self) -> Self
    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 2 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is optimized for base 2;
    /// [unchecked_ilog_uint()](#tymethod.unchecked_ilog_uint)
    /// can produce results of the base other than 2, and
    /// [unchecked_ilog10()](#tymethod.unchecked_ilog10)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(64_u8);
    /// let res = a_biguint.unchecked_ilog2();
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.unchecked_ilog2();
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.unchecked_ilog2();
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog2();
    /// ```
    pub fn unchecked_ilog2(&self) -> Self
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

    // pub fn panic_free_ilog10(&self) -> Self
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
    /// [painc_free_ilog_uint()](struct@BigUInt#method.painc_free_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [painc_free_ilog2()](struct@BigUInt#method.painc_free_ilog2)
    /// can produce results more efficiently for base 10.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_uint(12345_u32);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.panic_free_ilog10();
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
    /// # Example 4
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::zero();
    /// let res = a_biguint.panic_free_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(res.is_overflow(), false);
    /// assert_eq!(res.is_underflow(), false);
    /// assert_eq!(res.is_infinity(), false);
    /// assert_eq!(res.is_undefined(), true);
    /// assert_eq!(res.is_divided_by_zero(), false);
    /// assert_eq!(res.is_left_carry(), false);
    /// assert_eq!(res.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog10(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_ilog10_assign(&mut self)
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
    /// [panic_free_ilog_assign_uint()](struct@BigUInt#method.panic_free_ilog_assign_uint)
    /// can produce results of the base other than 10, and
    /// [panic_free_ilog2_assign()](struct@BigUInt#method.panic_free_ilog2_assign)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// # Example 2
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// # Example 3
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
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
    /// a_biguint.panic_free_ilog10_assign();
    /// println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
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
    /// # Example 4
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
    /// a_biguint.panic_free_ilog10_assign();
    /// println!("After a_biguint.panic_free_ilog10_assign(),\na_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    pub fn panic_free_ilog10_assign(&mut self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn checked_ilog10(&self) -> Option<Self>
    /// Calculates the base 10 logarithm of the number.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    /// or its behavior may be undefined though it may not panic.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down,
    /// wrapped by `Some` of enum `Option` if `self` is not zero.
    /// It returns `None` if `self` is zero.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [checked_ilog_uint()](#tymethod.checked_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [checked_ilog2()](#tymethod.checked_ilog2)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.checked_ilog10();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 10 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "4");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(70_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", 12345_u32, r);
    ///             assert_eq!(r.to_string(), "6");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.checked_ilog2();
    /// match res
    /// {
    ///     Some(r) => {
    ///             println!("The base 2 logarithm of {} is {}.", a_biguint, r);
    ///             assert_eq!(r.to_string(), "0");
    ///             assert_eq!(r.is_overflow(), false);
    ///             assert_eq!(r.is_underflow(), false);
    ///             assert_eq!(r.is_infinity(), false);
    ///             assert_eq!(r.is_divided_by_zero(), false);
    ///             assert_eq!(r.is_undefined(), false);
    ///             assert_eq!(r.is_left_carry(), false);
    ///             assert_eq!(r.is_right_carry(), false);
    ///         },
    ///     None => { println!("Error"); },
    /// }
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::zero();
    /// let base = 6_u8;
    /// let res = a_biguint.checked_ilog_uint(1_u8);
    /// match res
    /// {
    ///     Some(r) => { println!("The base 10 logarithm of {} is {}.", a_biguint, r); },
    ///     None => {
    ///             println!("Error");
    ///             assert_eq!(res, None);
    ///         },
    /// }
    /// ```
    pub fn checked_ilog10(&self) -> Option<Self>
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn unchecked_ilog10(&self) -> Self
    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    /// - If `size_of::<T>() * N` <= `128`, this method may panic
    ///   or its behavior may be undefined though it may not panic.
    /// - This function will panic if `self` is zero.
    /// 
    /// # Output
    /// It returns the base 10 logarithm of the number, rounded down.
    /// 
    /// # Counterpart Methods
    /// This method is not optimized for base 10 but provides convenience
    /// for base 10;
    /// [unchecked_ilog_uint()](#tymethod.unchecked_ilog_uint)
    /// can produce results of the base other than 10, and
    /// [unchecked_ilog2()](#tymethod.unchecked_ilog2)
    /// can produce results more efficiently for base 2.
    /// 
    /// # Example 1
    /// ```
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(10000_u32);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_uint(12345_u32);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "4");
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
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    ///
    /// let a_biguint = U256::from_uint(1_u8);
    /// let res = a_biguint.unchecked_ilog10();
    /// println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    /// assert_eq!(res.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_left_carry(), false);
    /// assert_eq!(a_biguint.is_right_carry(), false);
    /// ```
    /// 
    /// # Panic Example
    /// ```should_panic
    /// use cryptocol::number::BigInt_More;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let _a_biguint = U256::zero();
    /// // It will panic.
    /// let res = _a_biguint.unchecked_ilog10();
    /// ```
    pub fn unchecked_ilog10(&self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }


    
    /***** METHODS FOR MISCELLANEOUS ARITHMETIC OPERATIONS *****/

    // pub fn gcd(&self, other: &Self) -> Self
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
    /// The method [gcd_uint()](struct@BigUInt#method.gcd_uint)
    /// is more efficient than this method `gcd()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_uint()](struct@BigUInt#method.gcd_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    /// let c_biguint = a_biguint.gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // It will panic!
    /// let c_biguint = _a_biguint.gcd(&_b_biguint);
    /// 
    /// let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// let c_biguint = _a_biguint.gcd(&_b_biguint);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// let c_biguint = _a_biguint.gcd(&_b_biguint);
    /// ```
    pub fn gcd(&self, _other: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn gcd_assign(&mut self, other: &Self)
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
    /// The method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    /// a_biguint.gcd_assign(&b_biguint);
    /// println!("After a_biguint.gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // It will panic!
    /// _a_biguint.gcd_assign(&_b_biguint);
    /// 
    /// let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// _a_biguint.gcd_assign(&_b_biguint);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// _a_biguint.gcd_assign(&_b_biguint);
    /// ```
    pub fn gcd_assign(&mut self, _other: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_gcd(&self, other: &Self) -> Self
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
    /// The method [panic_free_gcd_uint()](struct@BigUInt#method.panic_free_gcd_uint)
    /// is more efficient than this method `panic_free_gcd()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [panic_free_gcd_uint()](struct@BigUInt#method.panic_free_gcd_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "1");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for zero and non-zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for non-zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::zero();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.panic_free_gcd(&b_biguint);
    /// println!("The greatest common divisor of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn panic_free_gcd(&self, _other: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_gcd_assign(&mut self, other: &Self)
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
    /// The method [panic_free_gcd_assign_uint()](struct@BigUInt#method.panic_free_gcd_assign_uint)
    /// is more efficient than this method `panic_free_gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [panic_free_gcd_assign_uint()](struct@BigUInt#method.panic_free_gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("12345678911111222223333344444555556666677777888889999900000").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999000001111122222333334444455555666667777788888999990000011111222223333344444555556666677777888889999900000").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::from_string("999998888877777666665555544444333332222211111000009999988888777776666655555444443333322222111110000099999888887777766666555554444433333222221111100000").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("8438991675438218095037710712144663668370976580551057770447200309276357957168036900503577855298223571234349459464291898319535238059076166186474530555556137").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999111112222233333444445555566666").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999111112222233333444445555566666");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// // let a_biguint = U256::from_uint(2_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(7_u8).pow_uint(63_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // let b_biguint = U256::from_uint(3_u8).pow_uint(72_u8).wrapping_mul(&U256::from_uint(5_u8).pow_uint(42_u8)).wrapping_mul(&U256::from_uint(11_u8).pow_uint(11_u8));
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "1");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for zero and non-zero
    /// ```
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
    /// 
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for non-zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for zero and zero
    /// ```
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
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint.panic_free_gcd_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_gcd_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn panic_free_gcd_assign(&mut self, _other: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn lcm(&self, other: &Self) -> Self
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
    /// The method [lcm_uint()](struct@BigUInt#method.lcm_uint)
    /// is more efficient than this method `lcm()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [lcm_uint()](struct@BigUInt#method.lcm_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    /// // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    /// // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    /// let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    /// // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    /// let c_biguint = a_biguint.lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u16);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // It will panic!
    /// let c_biguint = _a_biguint.lcm(&_b_biguint);
    /// 
    /// let _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// let c_biguint = _a_biguint.lcm(&_b_biguint);
    /// 
    /// let _a_biguint = U256::zero();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// let c_biguint = _a_biguint.lcm(&_b_biguint);
    /// ```
    pub fn lcm(&self, _other: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn lcm_assign(&mut self, other: &Self)
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
    /// The method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    /// // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    /// // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    /// // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    /// a_biguint.lcm_assign(&b_biguint);
    /// println!("After a_biguint.lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Panic Examples
    /// ```should_panic
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// // It will panic!
    /// _a_biguint.lcm_assign(&_b_biguint);
    /// 
    /// let mut _a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// _a_biguint.lcm_assign(&_b_biguint);
    /// 
    /// let mut _a_biguint = U256::zero();
    /// let _b_biguint = U256::zero();
    /// // It will panic!
    /// _a_biguint.lcm_assign(&_b_biguint);
    /// ```
    pub fn lcm_assign(&mut self, _other: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_lcm(&self, other: &Self) -> Self
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
    /// The method [lcm_uint()](struct@BigUInt#method.lcm_uint)
    /// is more efficient than this method `lcm()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [lcm_uint()](struct@BigUInt#method.lcm_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "111112222233333444445555566666777778888899999");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    /// // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    /// // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    /// let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    /// // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), false);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for zero and non-zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for non-zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u64);
    /// 
    /// let a_biguint = U256::zero();
    /// let b_biguint = U256::zero();
    /// let c_biguint = a_biguint.panic_free_lcm(&b_biguint);
    /// println!("The least common multiple of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    /// assert_eq!(c_biguint.to_string(), "0");
    /// assert_eq!(c_biguint.is_overflow(), false);
    /// assert_eq!(c_biguint.is_underflow(), false);
    /// assert_eq!(c_biguint.is_infinity(), false);
    /// assert_eq!(c_biguint.is_undefined(), true);
    /// assert_eq!(c_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn panic_free_lcm(&self, _other: &Self) -> Self
    {
        unimplemented!(); // Dummy code for documentation
    }

    // pub fn panic_free_lcm_assign(&mut self, other: &Self)
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
    /// The method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint)
    /// is more efficient than this method `gcd_assign()`
    /// when the exponent `other` is primitive unsigned integral data type
    /// such as u8, u16, u32, u64, and u128.
    /// If `other` is the primitive unsigned integral data type number,
    /// use the method [gcd_assign_uint()](struct@BigUInt#method.gcd_assign_uint).
    /// 
    /// # Example 1 for normal case
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("11111222223333344444555556666677777").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
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
    /// ```
    /// 
    /// # Example 2 for Two prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "496760330648797086669060276530363892567042647622050102007522050937573974925310180061832952383308835602422640469199738486829665946972839212975403672017049");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 3 for the case self is a prime number and other is a composite number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_string("74472489612356985600031654010179700239186733711125062343372502112654155197337").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "8274803816077825844928633065239507977211631334803000112653099668890202006953504399616683910703926797255292819444704102663");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 4 for the case self is a composite number and another is prime number
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_string("777776666655555444443333322222111110000022222").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("6670387054797362513395707836449423967156351509862541368962968031208086533377").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "5188071408782660471579036145752463281986051988336775031832198742768814408395807276440197274193933124374007024568944703694");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 5 for Same numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::from_string("111112222233333444445555566666777778888899999").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "111112222233333444445555566666777778888899999");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 6 for one prime number and its multiple
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_string("262586890850443215026048316017358917147061433899850397175592679960211511929529269359755816708006242574764016656012965410420527921966695199932942678613269").unwrap();
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807").unwrap();
    /// // let b_biguint = a_biguint.wrapping_mul_uint(3_u8);
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "787760672551329645078144948052076751441184301699551191526778039880634535788587808079267450124018727724292049968038896231261583765900085599798828035839807");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 7 for two relatively prime numbers
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U512::from_string("10842241695257098749029584685234126848").unwrap();
    /// // let a_biguint = U512::from_uint(2_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(7_u8).pow_uint(31_u8));
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U512::from_string("4502551836747118353130482137203216552734375").unwrap();
    /// // let b_biguint = U512::from_uint(3_u8).pow_uint(36_u8).wrapping_mul(&U512::from_uint(5_u8).pow_uint(29_u8)).wrapping_mul(&U512::from_uint(11_u8).pow_uint(5_u8));
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "48817755259436040224079590140222715578041897690278400000000000000000000000000000");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 8 for zero and non-zero
    /// ```
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
    /// 
    /// let b_biguint = U256::from_string("1461470517451445635247458978672662721125232940494242939166724681854248046875").unwrap();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 9 for non-zero and zero
    /// ```
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let mut a_biguint = U256::from_string("822879434848600686045915352446172654881155048801396400670057427986680905728").unwrap();
    /// println!("Originally, a_biguint = {}", a_biguint);
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), false);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    /// 
    /// # Example 10 for zero and zero
    /// ```
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
    /// 
    /// let b_biguint = U256::zero();
    /// a_biguint.panic_free_lcm_assign(&b_biguint);
    /// println!("After a_biguint.panic_free_lcm_assign(), a_biguint = {}.", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "0");
    /// assert_eq!(a_biguint.is_overflow(), false);
    /// assert_eq!(a_biguint.is_underflow(), false);
    /// assert_eq!(a_biguint.is_infinity(), false);
    /// assert_eq!(a_biguint.is_undefined(), true);
    /// assert_eq!(a_biguint.is_divided_by_zero(), false);
    /// ```
    pub fn panic_free_lcm_assign(&mut self, _other: &Self)
    {
        unimplemented!(); // Dummy code for documentation
    }
}