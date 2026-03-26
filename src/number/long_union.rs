// Copyright 2023, 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a 64-bit integer union for efficient memory sharing and byte-level
//! manipulation between different integer types and arrays.

// #![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]

use std::cmp::{ PartialEq, PartialOrd, Ordering };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };
use std::fmt::{ self, Alignment, Error, Formatter, Display, Debug, Pointer,
                Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp };

use crate::number::{ SmallUInt, ShortUnion, IntUnion, LongerUnion, SizeUnion };
use crate::number::{ union_calc_assign_to_calc, union_fmt_with_radix, union_fmt_with_exponent };

/// A 64-bit integer union that enables bit-level slicing and seamless 
/// conversion between various primitive types, including `u64`, `i64`, 
/// `u32`, `i32`, `u16`, `i16`, `u8`, and `i8`.
/// 
/// # Introduction
/// `LongUnion` provides efficient, bit-level access to 64-bit values. 
/// It allows manipulating the underlying memory as a single 64-bit word, 
/// two 32-bit words, four 16-bit bytes, or eight 8-bit bytes
/// in both signed and unsigned formats.
/// 
/// # Quick Start
/// To use this union, import `cryptocol::number::LongUnion` 
/// as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::LongUnion;
/// ```
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::LongUnion;
/// 
/// let a = LongUnion::new_with_signed(-1234567890987645_i64);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ulong() = {}", a.get_ulong());
/// println!("a.get_slong() = {}", a.get_slong());
/// assert_eq!(a.get(), 18445509505818563971_u64);
/// assert_eq!(a.get_signed(), -1234567890987645_i64);
/// assert_eq!(a.get_ulong(), 18445509505818563971_u64);
/// assert_eq!(a.get_slong(), -1234567890987645_i64);
/// 
/// for i in 0..2
///     { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
/// for i in 0..2
///     { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
/// for i in 0..4
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..4
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..8
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..8
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_uint_(0), 3278378371_u32);
/// assert_eq!(a.get_uint_(1), 4294679850_u32);
/// assert_eq!(a.get_sint_(0), -1016588925_i32);
/// assert_eq!(a.get_sint_(1), -287446_i32);
/// assert_eq!(a.get_ushort_(0), 5507_u16);
/// assert_eq!(a.get_ushort_(1), 50024_u16);
/// assert_eq!(a.get_ushort_(2), 40234_u16);
/// assert_eq!(a.get_ushort_(3), 65531_u16);
/// assert_eq!(a.get_sshort_(0), 5507_i16);
/// assert_eq!(a.get_sshort_(1), -15512_i16);
/// assert_eq!(a.get_sshort_(2), -25302_i16);
/// assert_eq!(a.get_sshort_(3), -5_i16);
/// assert_eq!(a.get_ubyte_(0), 131_u8);
/// assert_eq!(a.get_ubyte_(1), 21_u8);
/// assert_eq!(a.get_ubyte_(2), 104_u8);
/// assert_eq!(a.get_ubyte_(3), 195_u8);
/// assert_eq!(a.get_ubyte_(4), 42_u8);
/// assert_eq!(a.get_ubyte_(5), 157_u8);
/// assert_eq!(a.get_ubyte_(6), 251_u8);
/// assert_eq!(a.get_ubyte_(7), 255_u8);
/// assert_eq!(a.get_sbyte_(0), -125_i8);
/// assert_eq!(a.get_sbyte_(1), 21_i8);
/// assert_eq!(a.get_sbyte_(2), 104_i8);
/// assert_eq!(a.get_sbyte_(3), -61_i8);
/// assert_eq!(a.get_sbyte_(4), 42_i8);
/// assert_eq!(a.get_sbyte_(5), -99_i8);
/// assert_eq!(a.get_sbyte_(6), -5_i8);
/// assert_eq!(a.get_sbyte_(7), -1_i8);
/// 
/// #[cfg(target_pointer_width = "64")]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 18445509505818563971_usize);
///     assert_eq!(a.get_ssize(), -1234567890987645_isize);
/// }
/// #[cfg(target_pointer_width = "32")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 3278378371_usize);
///     assert_eq!(a.get_usize_(1), 4294679850_usize);
///     assert_eq!(a.get_ssize_(0), -1016588925_isize);
///     assert_eq!(a.get_ssize_(1), -287446_isize);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     const N: usize = 4;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 5507_usize);
///         assert_eq!(a.get_usize_(1), 50024_usize);
///         assert_eq!(a.get_usize_(2), 40234_usize);
///         assert_eq!(a.get_usize_(3), 65531_usize);
///         assert_eq!(a.get_ssize_(0), 5507_isize);
///         assert_eq!(a.get_ssize_(1), -15512_isize);
///         assert_eq!(a.get_ssize_(2), -25302_isize);
///         assert_eq!(a.get_ssize_(3), -5_isize);
/// }
/// #[cfg(target_pointer_width = "8")]
/// {
///     const N: usize = 8;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_ubyte_(0), 131_usize);
///     assert_eq!(a.get_ubyte_(1), 21_usize);
///     assert_eq!(a.get_ubyte_(2), 104_usize);
///     assert_eq!(a.get_ubyte_(3), 195_usize);
///     assert_eq!(a.get_ubyte_(4), 42_usize);
///     assert_eq!(a.get_ubyte_(5), 157_usize);
///     assert_eq!(a.get_ubyte_(6), 251_usize);
///     assert_eq!(a.get_ubyte_(7), 255_usize);
///     assert_eq!(a.get_sbyte_(0), -125_isize);
///     assert_eq!(a.get_sbyte_(1), 21_isize);
///     assert_eq!(a.get_sbyte_(2), 104_isize);
///     assert_eq!(a.get_sbyte_(3), -61_isize);
///     assert_eq!(a.get_sbyte_(4), 42_isize);
///     assert_eq!(a.get_sbyte_(5), -99_isize);
///     assert_eq!(a.get_sbyte_(6), -5_isize);
///     assert_eq!(a.get_sbyte_(7), -1_isize);
/// }
/// ```
/// 
/// Note that `get_usize()` and `get_ssize()` (including their indexed 
/// variants) are available only on architectures with supported pointer 
/// widths, such as 64-bit, 32-bit, 16-bit, or 8-bit systems.
/// 
/// `LongUnion` can be used just like a `u64`, supporting all standard 
/// arithmetic operations, including addition, subtraction, 
/// multiplication, and division. Integrating it with `SmallUInt` 
/// provides enhanced functionality and convenience, often eliminating 
/// the need for an explicit `LongUnion` import.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_longunion = 12345678987654321_u64.into_longunion();
/// let b_longunion = 87654321012345678_u64.into_longunion();
/// let c_longunion = a_longunion.wrapping_add(b_longunion);
/// println!("{} + {} = {}", a_longunion, b_longunion, c_longunion);
/// assert_eq!(c_longunion.get(), 99999999999999999_u64);
/// for i in 0..2
///     { println!("c_longunion.get_uint_({}) = {}", i, c_longunion.get_uint_(i)); }
/// assert_eq!(c_longunion.get_uint_(0), 1569325055_u32);
/// assert_eq!(c_longunion.get_uint_(1), 23283064_u32);
/// for i in 0..4
///     { println!("c_longunion.get_ushort_({}) = {}", i, c_longunion.get_ushort_(i)); }
/// assert_eq!(c_longunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_longunion.get_ushort_(1), 23945_u16);
/// assert_eq!(c_longunion.get_ushort_(2), 17784_u16);
/// assert_eq!(c_longunion.get_ushort_(3), 355_u16);
/// for i in 0..8
///     { println!("c_longunion.get_ubyte_({}) = {}", i, c_longunion.get_ubyte_(i)); }
/// assert_eq!(c_longunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_longunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_longunion.get_ubyte_(2), 137_u8);
/// assert_eq!(c_longunion.get_ubyte_(3), 93_u8);
/// assert_eq!(c_longunion.get_ubyte_(4), 120_u8);
/// assert_eq!(c_longunion.get_ubyte_(5), 69_u8);
/// assert_eq!(c_longunion.get_ubyte_(6), 99_u8);
/// assert_eq!(c_longunion.get_ubyte_(7), 1_u8);
/// 
/// let d_longunion = b_longunion - a_longunion;
/// println!("{} - {} = {}", b_longunion, a_longunion, d_longunion);
/// assert_eq!(d_longunion.get(), 75308642024691357_u64);
/// for i in 0..2
///     { println!("d_longunion.get_uint_({}) = {}", i, d_longunion.get_uint_(i)); }
/// assert_eq!(d_longunion.get_uint_(0), 2556827293_u32);
/// assert_eq!(d_longunion.get_uint_(1), 17534159_u32);
/// for i in 0..4
///     { println!("d_longunion.get_ushort_({}) = {}", i, d_longunion.get_ushort_(i)); }
/// assert_eq!(d_longunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_longunion.get_ushort_(1), 39014_u16);
/// assert_eq!(d_longunion.get_ushort_(2), 36047_u16);
/// assert_eq!(d_longunion.get_ushort_(3), 267_u16);
/// for i in 0..8
///     { println!("d_longunion.get_ubyte_({}) = {}", i, d_longunion.get_ubyte_(i)); }
/// assert_eq!(d_longunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_longunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_longunion.get_ubyte_(2), 102_u8);
/// assert_eq!(d_longunion.get_ubyte_(3), 152_u8);
/// assert_eq!(d_longunion.get_ubyte_(4), 207_u8);
/// assert_eq!(d_longunion.get_ubyte_(5), 140_u8);
/// assert_eq!(d_longunion.get_ubyte_(6), 11_u8);
/// assert_eq!(d_longunion.get_ubyte_(7), 1_u8);
/// 
/// let e_longunion = d_longunion * 3_u64.into_longunion();
/// println!("{} * {} = {}", d_longunion, 3_u64.into_longunion(), e_longunion);
/// assert_eq!(e_longunion.get(), 225925926074074071_u64);
/// 
/// let f_longunion = c_longunion / 10_u32.into_longunion();
/// println!("{} / {} = {}", c_longunion, 10_u16.into_longunion(), f_longunion);
/// assert_eq!(f_longunion.get(), 9999999999999999_u64);
/// 
/// let g_longunion = c_longunion % 10_u64.into_longunion();
/// println!("{} % {} = {}", c_longunion, 10_u64.into_longunion(), g_longunion);
/// assert_eq!(g_longunion.get(), 9_u64);
/// ```
///  
/// # Big-endian Support
/// Support for Big-Endian architectures is currently experimental and not 
/// recommended for production environments. Users assume all 
/// responsibility for any issues that may arise when using this crate 
/// on Big-Endian systems.
#[derive(Copy, Clone, Eq)]
#[allow(dead_code)]
pub union LongUnion
{
    /// The 64-bit unsigned representation (for compatibility).
    this: u64,

    /// The 64-bit signed representation (for compatibility).
    that: i64,

    /// The primary 64-bit unsigned integer field.
    ulong: u64,

    /// The primary 64-bit signed integer field.
    slong: i64,

    /// Array of two 32-bit unsigned integers.
    uint: [u32; 2],

    /// Array of two 32-bit signed integers.
    sint: [i32; 2],

    /// Array of four 16-bit unsigned integers.
    ushort: [u16; 4],

    /// Array of four 16-bit signed integers.
    sshort: [i16; 4],

    /// Array of eight 8-bit unsigned integers.
    ubyte: [u8; 8],

    /// Array of eight 8-bit signed integers.
    sbyte: [i8; 8],

    /// Pointer-sized unsigned representation (64-bit architectures).
    #[cfg(target_pointer_width = "64")] u_size: usize,

    /// Pointer-sized signed representation (64-bit architectures).
    #[cfg(target_pointer_width = "64")] s_size: isize,

    /// Array of two pointer-sized unsigned integers (32-bit architectures).
    #[cfg(target_pointer_width = "32")] u_size: [usize; 2],

    /// Array of two pointer-sized signed integers (32-bit architectures).
    #[cfg(target_pointer_width = "32")] s_size: [isize; 2],

    /// Array of four pointer-sized unsigned integers (16-bit architectures).
    #[cfg(target_pointer_width = "16")] u_size: [usize; 4],

    /// Array of four pointer-sized signed integers (16-bit architectures).
    #[cfg(target_pointer_width = "16")] s_size: [isize; 4],

    // / Array of eight pointer-sized unsigned integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] u_size: [usize; 8],

    // / Array of eight pointer-sized signed integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] s_size: [isize; 8],
}

impl LongUnion
{
    // pub const fn new() -> Self
    /// Constructs a new `LongUnion` with all fields initialized to zero.
    /// 
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u64);
    /// ```
    #[inline] pub const fn new() -> Self    { Self { ulong: 0 } }

    // pub const fn new_with(ulong: u64) -> Self
    /// Constructs a new `LongUnion` initialized with the given `u64` value.
    /// 
    /// # Arguments
    /// * `ulong`: The 64-bit unsigned integer value to initialize the union.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(12345678909876456_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 12345678909876456_u64);
    /// ```
    #[inline] pub const fn new_with(ulong: u64) -> Self { Self { ulong } }

    // pub const fn new_with_signed(slong: i64) -> Self
    /// Constructs a new `LongUnion` initialized with the given `i64` value.
    /// 
    /// # Arguments
    /// * `slong`: The 64-bit signed integer value to initialize the union.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with_signed(-12345678909876456_i64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -12345678909876456_i64);
    /// ```
    #[inline] pub const fn new_with_signed(slong: i64) -> Self  { Self { slong } }

    // pub const fn new_with_ubytes(ubyte: [u8; 8]) -> Self
    /// Constructs a new `LongUnion` initialized with the given byte array.
    /// 
    /// # Arguments
    /// * `ubyte`: An array of eight 8-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let arr = [131_u8, 21_u8, 104_u8, 195_u8, 42_u8, 157_u8, 251_u8, 255_u8];
    /// let a = LongUnion::new_with_ubytes(arr);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub const fn new_with_ubytes(ubyte: [u8; 8]) -> Self  { Self { ubyte } }

    // pub const fn new_with_ushorts(ushort: [u16; 4]) -> Self
    /// Constructs a new `LongUnion` initialized with the given 16-bit word array.
    /// 
    /// # Arguments
    /// * `ushort`: An array of four 16-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_ushorts([5507_u16, 50024_u16, 40234_u16, 65531_u16]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub const fn new_with_ushorts(ushort: [u16; 4])  -> Self  { Self { ushort } }

    // pub const fn new_with_uints(uint: [u32; 2]) -> Self
    /// Constructs a new `LongUnion` initialized with the given 32-bit word array.
    /// 
    /// # Arguments
    /// * `uint`: An array of two 32-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_uints([3278378371_u32, 4294679850_u32]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// ```
    #[inline] pub const fn new_with_uints(uint: [u32; 2]) -> Self   { Self { uint } }

    // pub const fn new_with_u128(num: u128) -> Self
    /// Constructs a new `LongUnion` initialized with the lowest 64 bits of
    /// the given `u128` value.
    /// 
    /// # Arguments
    /// * `num`: The 128-bit unsigned integer to initialize from.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_u128(18445509505818563971_u128);
    /// let b = LongUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 18445509505818563971_u64);
    /// assert_eq!(b.get(), 12312739301371248917_u64);
    /// ```
    #[inline] pub const fn new_with_u128(num: u128) -> Self { Self { ulong: num as u64 } }

    // pub const fn new_with_bool(b: bool) -> Self
    /// Constructs a new `LongUnion` initialized based on the given boolean 
    /// value.
    /// 
    /// # Arguments
    /// * `b`: The boolean value. `true` becomes `1`, and `false` becomes `0`.
    ///
    /// # Returns
    /// A new `LongUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with_bool(true);
    /// let b = LongUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u64);
    /// assert_eq!(b.get(), 0_u64);
    /// ```
    #[inline] pub const fn new_with_bool(b: bool) -> Self   { Self { ulong: b as u64 } }

    // pub fn get(self) -> u64
    /// Returns the union's value as a 64-bit unsigned integer.
    /// 
    /// # Returns
    /// The 64-bit unsigned integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(654321987654321_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 654321987654321_u64);
    /// ```
    #[inline] pub fn get(self) -> u64           { unsafe { self.this } }

    // pub fn set(&mut self, val: u64)
    /// Sets the union's value from a 64-bit unsigned integer.
    /// 
    /// # Arguments
    /// * `val`: The 64-bit unsigned integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let mut a = LongUnion::new();
    /// a.set(654321987654321_u64);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 654321987654321_u64);
    /// ```
    #[inline] pub fn set(&mut self, val: u64)   { self.this = val; }

    // pub fn get_signed(self) -> i64
    /// Returns the union's value as a 64-bit signed integer.
    /// 
    /// # Returns
    /// The 64-bit signed integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let a = LongUnion::new_with(12345678909876456789_u64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -6101065163833094827_i64);
    /// ```
    #[inline] pub fn get_signed(self) -> i64    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i64)
    /// Sets the union's value from a 64-bit signed integer.
    /// 
    /// # Arguments
    /// * `val`: The 64-bit signed integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;    
    /// let mut a = LongUnion::new();
    /// a.set_signed(-6101065163833094827_i64);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -6101065163833094827_i64);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i64)    { self.that = val; }

    crate::number::get_set_long_fit!();

    crate::number::get_set_byte!(8);

    crate::number::get_set_short!(4);

    crate::number::get_set_int!(2);

    #[cfg(target_pointer_width = "64")]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_usize!(2);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_usize!(4);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_usize!(8);

    crate::number::integer_union_methods!(u64);

    // pub fn as_ptr(&self) -> *const u64
    /// Returns a raw pointer to the union's memory as a `u64`.
    /// 
    /// # Returns
    /// A `*const u64` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let a = LongUnion::new_with(0x1234567887654321_u64);
    /// let ptr = a.as_ptr();
    /// unsafe { assert_eq!(*ptr, 0x1234567887654321_u64); }
    /// ```
    #[inline] pub fn as_ptr(&self) -> *const u64 { unsafe { self.ubyte.as_ptr() as *const u64 } }

    // pub fn as_mut_ptr(&mut self) -> *mut u64
    /// Returns a mutable raw pointer to the union's memory as a `u64`.
    /// 
    /// # Returns
    /// A `*mut u64` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongUnion;
    /// let mut a = LongUnion::new();
    /// let ptr = a.as_mut_ptr();
    /// unsafe { *ptr = 0x8765432112345678_u64; }
    /// assert_eq!(a.get(), 0x8765432112345678_u64);
    /// ```
    #[inline] pub fn as_mut_ptr(&mut self) -> *mut u64 { unsafe { self.ubyte.as_mut_ptr() as *mut u64 } }
}



crate::number::operators_for_integer_unions_impl! { LongUnion }

crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { LongUnion, usize }

crate::number::shift_ops_for_integer_unions_by_union_impl! { LongUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongUnion, SizeUnion }

crate::number::format_for_integer_unions_impl! { LongUnion }



impl Ord for LongUnion
{
    // fn cmp(&self, other: &Self) -> Ordering
    /// This method returns an Ordering between self and other.
    /// 
    /// # Returns
    /// An Ordering between self and other.
    /// 
    /// # Features
    /// By convention, self.cmp(&other) returns the ordering matching
    /// the expression self <operator> other if true.
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.get().cmp(&other.get())
    }
}


impl Debug for LongUnion
{
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    /// Formats the value using the given formatter for debugging purposes.
    /// 
    /// # Arguments
    /// * `f`: The formatter.
    /// 
    /// # Returns
    /// A `fmt::Result` indicating the outcome of the operation.
    /// 
    /// # Formatting Options
    /// The `:?` format specifier provides standard debug output, while the 
    /// alternate `:#?` specifier produces pretty-printed output for 
    /// enhanced readability.
    /// 
    /// # Example for the format specifier :?
    /// Using the format specifier `:?` for debug output:
    /// ```
    /// use cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:?}"), "LongUnion { this: 17212176183586094827, that: -1234567890123456789, ulong: 17212176183586094827, slong: -1234567890123456789, uint: [2182512363, 4007522059], sint: [-2112454933, -287445237], ushort: [32491, 33302, 61195, 61149], sshort: [32491, -32234, -4341, -4387], ubyte: [235, 126, 22, 130, 11, 239, 221, 238], sbyte: [-21, 126, 22, -126, 11, -17, -35, -18], u_size: 17212176183586094827, s_size: -1234567890123456789 }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// Using the alternate format specifier `:#?` for pretty-printed debug output:
    /// ```
    /// use cryptocol::number::*;
    /// let a_long = LongUnion::new_with_signed(-1234567890123456789_i64);
    /// println!("a_long = {:#?}", a_long);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_long:#?}"), r#"LongUnion {
    ///     this: 17212176183586094827,
    ///     that: -1234567890123456789,
    ///     ulong: 17212176183586094827,
    ///     slong: -1234567890123456789,
    ///     uint: [
    ///         2182512363,
    ///         4007522059,
    ///     ],
    ///     sint: [
    ///         -2112454933,
    ///         -287445237,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         33302,
    ///         61195,
    ///         61149,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         -32234,
    ///         -4341,
    ///         -4387,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         22,
    ///         130,
    ///         11,
    ///         239,
    ///         221,
    ///         238,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         22,
    ///         -126,
    ///         11,
    ///         -17,
    ///         -35,
    ///         -18,
    ///     ],
    ///     u_size: 17212176183586094827,
    ///     s_size: -1234567890123456789,
    /// }"#);
    /// ```
    /// 
    /// # Note
    /// This method follows the standard implementation of `fmt()` for the 
    /// `Debug` trait, providing a consistent experience for users familiar 
    /// with primitive integer types. For more details, refer to the official 
    /// Rust documentation linked in the References section.
    /// 
    /// # References
    /// - [Rust `fmt::Debug` documentation](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut ff = f.debug_struct("LongUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulong", &self.get_ulong())
            .field("slong", &self.get_slong())
            .field("uint", &[self.get_uint_(0), self.get_uint_(1)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7)]);
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        // #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
        //                                         .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        ff.finish()
    }
}
