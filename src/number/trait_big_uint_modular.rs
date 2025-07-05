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


use std::fmt::{ Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
                BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign };
use crate::number::SmallUInt;



/// # Introduction
/// Trait `BigUInt_Modular` is for BigUInt
///
/// # Quick start
/// In order to use this trait, you have to import (use)
/// `cryptocol::number::BigUInt_More` as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::BigUInt_Modular;
/// ```
/// If you import (use) `cryptocol::number::BigUInt_Modular`, all the methods of
/// `BigUInt_Modular` are available immediately and automagically, as if such
/// primitive data types had the methods from the begining.
/// 
/// ## Example 2
/// ```
/// use std::str::FromStr;
/// use cryptocol::define_utypes_with;
/// use cryptocol::number::BigUInt_Modular;
/// 
/// define_utypes_with!(u128);
/// 
/// let a_biguint = U1024::from([1; 8]);
/// let b_biguint = U1024::from_str_radix("00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001__00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001", 2).unwrap();
/// let modulo_biguint = UU128::from_str("1234567891234567879123456789111111111222222222333333333444444444555555555666666666777777777888888888999999999000000000").unwrap();
///
/// let mut res_biguint = a_biguint.modular_add(&b_biguint, &modulo_biguint);
/// println!("{} + {} = {} (mod {})", a_biguint, b_biguint, res_biguint, modulo_biguint);
/// assert_eq!(res_biguint.to_string(), "715095817710923963845493130173735203953802580597972250927984767909588940861217692608653370556592101327600086389804546");
/// 
/// res_biguint = a_biguint.modular_sub(&b_biguint, &modulo_biguint);
/// println!("{} - {} = {} (mod {})", a_biguint, b_biguint, res_biguint, modulo_biguint);
/// assert_eq!(res_biguint.to_string(), "0");
/// 
/// res_biguint = a_biguint.modular_mul(&b_biguint, &modulo_biguint);
/// println!("{} * {} = {} (mod {})", a_biguint, b_biguint, res_biguint, modulo_biguint);
/// assert_eq!(res_biguint.to_string(), "607396048570755101032038914863987293211201188879216700220932513864139285125189692949640169667338866132733039020566529");
///
/// res_biguint = a_biguint.modular_div(&b_biguint, &modulo_biguint);
/// println!("{} / {} = {} (mod {})", a_biguint, b_biguint, res_biguint, modulo_biguint);
/// assert_eq!(res_biguint.to_string(), "1");
/// 
/// res_biguint = a_biguint.modular_rem(&b_biguint, &modulo_biguint);
/// println!("{} % {} = {} (mod {})", a_biguint, b_biguint, res_biguint, modulo_biguint);
/// assert_eq!(res_biguint.to_string(), "0");
/// ```
#[allow(non_camel_case_types)]
pub trait BigUInt_Modular<T, const N: usize> : Clone + Sized //+ Display + + ToString
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    /*** ADDITION ***/

    // fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// # Counterpart Methods
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_add()](trait@BigUInt_Modular#tymethod.modular_add)
    ///   is proper rather than this method `modular_add_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_add_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_add_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_add_uint)
    fn modular_add_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger tham `ui128`, the method
    ///   [modular_add_assign()](trait@BigUInt_Modular#tymethod.modular_add_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_add_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_add_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_add_assign_uint)
    fn modular_add_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_add_uint()](trait@BigUInt_Modular#tymethod.modular_add_uint)
    ///   is a bit faster than this method `modular_add()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_add_uint()](trait@BigUInt_Modular#tymethod.modular_add_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_add()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_add),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_add)
    fn modular_add(&self, rhs: &Self, modulo: &Self) -> Self;

    // fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_add_assign_uint()](trait@BigUInt_Modular#tymethod.modular_add_assign_uint)
    ///   is a bit faster than this method `modular_add_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_add_assign_uint()](trait@BigUInt_Modular#tymethod.modular_add_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_add_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_add_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_add_assign)
    fn modular_add_assign(&mut self, rhs: &Self, modulo: &Self);



    /*** SUBTRACTION ***/

    // fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_sub()](trait@BigUInt_Modular#tymethod.modular_sub)
    ///   is proper rather than this method `modular_sub_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_sub_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_sub_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_sub_uint)
    fn modular_sub_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger tham `ui128`, the method
    ///   [modular_sub_assign()](trait@BigUInt_Modular#tymethod.modular_sub_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_sub_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_sub_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_sub_assign_uint)
    fn modular_sub_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_sub_uint()](trait@BigUInt_Modular#tymethod.modular_sub_uint)
    ///   is a bit faster than this method `modular_sub()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_sub_uint()](trait@BigUInt_Modular#tymethod.modular_sub_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_sub()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_sub),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_sub)
    fn modular_sub(&self, rhs: &Self, modulo: &Self) -> Self;

    // fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_sub_assign_uint()](trait@BigUInt_Modular#tymethod.modular_sub_assign_uint)
    ///   is a bit faster than this method `modular_sub_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_sub_assign_uint()](trait@BigUInt_Modular#tymethod.modular_sub_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_sub_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_sub_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_sub_assign)
    fn modular_sub_assign(&mut self, rhs: &Self, modulo: &Self);



    /*** MULTIPLICATION ***/

    // fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_mul()](trait@BigUInt_Modular#tymethod.modular_mul)
    ///   is proper rather than this method `modular_mul_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_mul_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_mul_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_mul_uint)
    fn modular_mul_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger tham `ui128`, the method
    ///   [modular_mul_assign()](trait@BigUInt_Modular#tymethod.modular_mul_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_mul_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_mul_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_mul_assign_uint)
    fn modular_mul_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_mul_uint()](trait@BigUInt_Modular#tymethod.modular_mul_uint)
    ///   is a bit faster than this method `modular_mul()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_mul_uint()](trait@BigUInt_Modular#tymethod.modular_mul_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_mul()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_mul),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_mul)
    fn modular_mul(&self, rhs: &Self, modulo: &Self) -> Self;

    // fn modular_mul_assign(&self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_mul_assign_uint()](trait@BigUInt_Modular#tymethod.modular_mul_assign_uint)
    ///   is a bit faster than this method `modular_mul_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_mul_assign_uint()](trait@BigUInt_Modular#tymethod.modular_mul_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_mul_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_mul_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_mul_assign)
    fn modular_mul_assign(&mut self, rhs: &Self, modulo: &Self);


 
    /*** DIVISION ***/

    // fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_div()](trait@BigUInt_Modular#tymethod.modular_div)
    ///   is proper rather than this method `modular_div_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_div_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_div_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_div_uint)
    fn modular_div_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger tham `ui128`, the method
    ///   [modular_div_assign()](trait@BigUInt_Modular#tymethod.modular_div_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_div_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_div_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_div_assign_uint)
    fn modular_div_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_div_uint()](trait@BigUInt_Modular#tymethod.modular_div_uint)
    ///   is a bit faster than this method `modular_div()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_div_uint()](trait@BigUInt_Modular#tymethod.modular_div_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_div()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_div),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_div)
    fn modular_div(&self, rhs: &Self, modulo: &Self) -> Self;

    // fn modular_div_assign(&self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_div_assign_uint()](trait@BigUInt_Modular#tymethod.modular_div_assign_uint)
    ///   is a bit faster than this method `modular_mul_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_div_assign_uint()](trait@BigUInt_Modular#tymethod.modular_div_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_div_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_div_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_div_assign)
    fn modular_div_assign(&mut self, rhs: &Self, modulo: &Self);

    // fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_rem()](trait@BigUInt_Modular#tymethod.modular_rem)
    ///   is proper rather than this method `modular_rem_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_rem_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_rem_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_rem_uint)
    fn modular_rem_uint<U>(&self, rhs: U, modulo: &Self) -> U
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger tham `ui128`, the method
    ///   [modular_rem_assign()](trait@BigUInt_Modular#tymethod.modular_rem_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_rem_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_rem_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_rem_assign_uint)
    fn modular_rem_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_rem_uint()](trait@BigUInt_Modular#tymethod.modular_rem_uint)
    ///   is a bit faster than this method `modular_rem()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_rem_uint()](trait@BigUInt_Modular#tymethod.modular_rem_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_rem()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_rem),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_rem)
    fn modular_rem(&self, rhs: &Self, modulo: &Self) -> Self;

    // fn modular_rem_assign(&self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_rem_assign_uint()](trait@BigUInt_Modular#tymethod.modular_rem_assign_uint)
    ///   is a bit faster than this method `modular_mul_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_rem_assign_uint()](trait@BigUInt_Modular#tymethod.modular_rem_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_rem_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_rem_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_rem_assign)
    fn modular_rem_assign(&mut self, rhs: &Self, modulo: &Self);



    /*** METHODS FOR EXPONENTIATION AND LOGARITHM ***/

    // fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
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
    /// - If `exp` is bigger than `u128`, use the method
    ///   [modular_pow()](trait@BigUInt_Modular#tymethod.modular_pow) instead.
    /// - In order to use any one of
    ///   [panic_free_modular_pow_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_pow_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_pow_uint)
    fn modular_pow_uint<U>(&self, exp: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
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
    /// - If `exp` is bigger than `u128`, the method
    ///   [modular_pow_assign()](trait@BigUInt_Modular#tymethod.modular_pow_assign)
    ///   is proper rather than this method.
    /// - In order to use any one of
    ///   [panic_free_modular_pow_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_pow_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_pow_assign_uint)
    fn modular_pow_assign_uint<U>(&mut self, exp: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn modular_pow(&self, exp: &Self, modulo: &Self) -> Self
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
    /// - The method [modular_pow_uint()](trait@BigUInt_Modular#tymethod.modular_pow_uint)
    ///   is more efficient than this method `modular_pow()` when the exponent
    ///   `exp` is primitive unsigned integral data type
    ///   such as u8, u16, u32, u64, and u128.
    ///   If `rhs` is the primitive unsigned integral data type number,
    ///   use the method [modular_pow_uint()](trait@BigUInt_Modular#tymethod.modular_pow_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_pow()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_pow),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_pow)
    fn modular_pow(&self, exp: &Self, modulo: &Self) -> Self;

    // fn modular_pow_assign(&mut self, exp: &Self, modulo: &Self)
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
    /// - The method [modular_pow_assign_uint()](trait@BigUInt_Modular#tymethod.modular_pow_assign_uint)
    ///   is more efficient than this method `modular_pow_assign()`
    ///   when the exponent `exp` is primitive unsigned integral data type
    ///   such as u8, u16, u32, u64, and u128.
    ///   If `exp` is the primitive unsigned integral data type number,
    ///   use the method [modular_pow_assign_uint()](trait@BigUInt_Modular#tymethod.modular_pow_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_pow_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_pow_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_pow_assign)
    fn modular_pow_assign(&mut self, exp: &Self, modulo: &Self);



    /*** MULTIPLE ***/

    // fn modular_next_multiple_of_uint<U>(&self, rhs: U, modulo: &Self) -> Self
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [modular_next_multiple_of()](#tymethod.modular_next_multiple_of)
    ///   is proper rather than this method `modular_next_multiple_of_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_next_multiple_of_uint)
    fn modular_next_multiple_of_uint<U>(&self, rhs: U, modulo: &Self) -> Self
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;
    
    // fn modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
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
    /// - If `rhs` is bigger than `u128`, the method
    ///   [next_multiple_of_assign()](#tymethod.next_multiple_of_assign)
    ///   is proper rather than this method `next_multiple_of_assign_uint()`.
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_assign_uint()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_assign_uint),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_modular/struct.BigUInt.html#method.modular_next_multiple_of_assign_uint)
    fn modular_next_multiple_of_assign_uint<U>(&mut self, rhs: U, modulo: &Self)
    where U: SmallUInt + Copy + Clone + Display + Debug + ToString
            + Add<Output=U> + AddAssign + Sub<Output=U> + SubAssign
            + Mul<Output=U> + MulAssign + Div<Output=U> + DivAssign
            + Rem<Output=U> + RemAssign
            + Shl<Output=U> + ShlAssign + Shr<Output=U> + ShrAssign
            + BitAnd<Output=U> + BitAndAssign + BitOr<Output=U> + BitOrAssign
            + BitXor<Output=U> + BitXorAssign + Not<Output=U>
            + PartialEq + PartialOrd;

    // fn modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self
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
    /// - The method
    ///   [modular_next_multiple_of_uint()](#tymethod.modular_next_multiple_of_uint)
    ///   is a bit faster than this method `modular_next_multiple_of()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_next_multiple_of_uint()](#tymethod.modular_next_multiple_of_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_next_multiple_of)
    fn modular_next_multiple_of(&self, rhs: &Self, modulo: &Self) -> Self;
    
    // fn modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self)
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
    /// - The method
    ///   [modular_next_multiple_of_assign_uint()](#tymethod.modular_next_multiple_of_assign_uint)
    ///   is a bit faster than this method `modular_next_multiple_of_assign()`.
    ///   So, if `rhs` is primitive unsigned integral data type such as u8, u16,
    ///   u32, u64, and u128, use the method
    ///   [modular_next_multiple_of_assign_uint()](#tymethod.modular_next_multiple_of_assign_uint).
    /// - In order to use any one of
    ///   [panic_free_modular_next_multiple_of_assign()](trait.BigUInt_Panic_Free.html#tymethod.panic_free_modular_next_multiple_of_assign),
    ///   you need to import (use) the trait `BigUInt_Panic_Free`.
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
    /// # For more examples,
    /// click [here](./documentation/big_uint_other_calculation/struct.BigUInt.html#method.modular_next_multiple_of_assign)
    fn modular_next_multiple_of_assign(&mut self, rhs: &Self, modulo: &Self);
}
