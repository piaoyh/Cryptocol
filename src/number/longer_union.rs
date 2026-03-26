// Copyright 2023, 2024. 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a 128-bit integer union for efficient memory sharing and byte-level
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

use crate::number::{ SmallUInt, ShortUnion, IntUnion, LongUnion, SizeUnion };
use crate::number::{ union_calc_assign_to_calc, union_fmt_with_radix, union_fmt_with_exponent };

/// A 128-bit integer union that enables bit-level slicing and seamless 
/// conversion between various primitive types, including `u128`, `i128`, 
/// `u64`, `i64`, `u32`, `i32`, `u16`, `i16`, `u8`, and `i8`.
/// 
/// # Introduction
/// `LongerUnion` provides efficient, bit-level access to 128-bit values. 
/// It allows manipulating the underlying memory as a single 128-bit word, 
/// two 64-bit words, four 32-bit words, eight 16-bit words, or sixteen 
/// 8-bit bytes, in both signed and unsigned formats.
/// 
/// # Quick Start
/// To use this union, import `cryptocol::number::LongerUnion` 
/// as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::LongerUnion;
/// ```
/// 
/// Use methods such as `get()`, `get_signed()`, `get_ulonger()`, and 
/// `get_slonger()` to retrieve the underlying 128-bit value in various 
/// integer formats. You can also slice the data into two `u64` values 
/// using `get_ulong()` and `get_slong()`, four `u32` values using 
/// `get_uint()` and `get_sint()`, eight `u16` values using `get_ushort()` 
/// and `get_sshort()`, or sixteen `u8` values using `get_ubyte()` and 
/// `get_sbyte()` (including their indexed variants). Note that 
/// `get_usize()` and `get_ssize()` are available only on architectures 
/// with supported pointer widths, such as 128-bit, 64-bit, 32-bit, 
/// 16-bit, or 8-bit systems.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::LongerUnion;
/// 
/// let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
/// 
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ulonger() = {}", a.get_ulonger());
/// println!("a.get_slonger() = {}", a.get_slonger());
/// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
/// assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_i128);
/// assert_eq!(a.get_ulonger(), 339047799029950809142362261752780557135_u128);
/// assert_eq!(a.get_slonger(), -1234567890987654321012345678987654321_i128);
/// 
/// #[cfg(target_endian = "little")]
/// {
///     for i in 0..2
///         { println!("a.get_ulong_({}) = {}", i, a.get_ulong_(i)); }
///     for i in 0..2
///         { println!("a.get_slong_({}) = {}", i, a.get_slong_(i)); }
///     for i in 0..4
///         { println!("a.get_uint_({}) = {}", i, a.get_uint_(i)); }
///     for i in 0..4
///         { println!("a.get_sint_({}) = {}", i, a.get_sint_(i)); }
///     for i in 0..8
///         { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
///     for i in 0..8
///         { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
///     for i in 0..16
///         { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
///     for i in 0..16
///         { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
///     assert_eq!(a.get_ulong_(0), 13664881099896654671_u64);
///     assert_eq!(a.get_ulong_(1), 18379818014235068504_u64);
///     assert_eq!(a.get_slong_(0), -4781862973812896945_i64);
///     assert_eq!(a.get_slong_(1), -66926059474483112_i64);
///     assert_eq!(a.get_uint_(0), 4048161615_u32);
///     assert_eq!(a.get_uint_(1), 3181603061_u32);
///     assert_eq!(a.get_uint_(2), 2127464536_u32);
///     assert_eq!(a.get_uint_(3), 4279384858_u32);
///     assert_eq!(a.get_sint_(0), -246805681_i32);
///     assert_eq!(a.get_sint_(1), -1113364235_i32);
///     assert_eq!(a.get_sint_(2), 2127464536_i32);
///     assert_eq!(a.get_sint_(3), -15582438_i32);
///     assert_eq!(a.get_ushort_(0), 2895_u16);
///     assert_eq!(a.get_ushort_(1), 61770_u16);
///     assert_eq!(a.get_ushort_(2), 26869_u16);
///     assert_eq!(a.get_ushort_(3), 48547_u16);
///     assert_eq!(a.get_ushort_(4), 34904_u16);
///     assert_eq!(a.get_ushort_(5), 32462_u16);
///     assert_eq!(a.get_ushort_(6), 15130_u16);
///     assert_eq!(a.get_ushort_(7), 65298_u16);
///     assert_eq!(a.get_sshort_(0), 2895_i16);
///     assert_eq!(a.get_sshort_(1), -3766_i16);
///     assert_eq!(a.get_sshort_(2), 26869_i16);
///     assert_eq!(a.get_sshort_(3), -16989_i16);
///     assert_eq!(a.get_sshort_(4), -30632_i16);
///     assert_eq!(a.get_sshort_(5), 32462_i16);
///     assert_eq!(a.get_sshort_(6), 15130_i16);
///     assert_eq!(a.get_sshort_(7), -238_i16);
///     assert_eq!(a.get_ubyte_(0), 79_u8);
///     assert_eq!(a.get_ubyte_(1), 11_u8);
///     assert_eq!(a.get_ubyte_(2), 74_u8);
///     assert_eq!(a.get_ubyte_(3), 241_u8);
///     assert_eq!(a.get_ubyte_(4), 245_u8);
///     assert_eq!(a.get_ubyte_(5), 104_u8);
///     assert_eq!(a.get_ubyte_(6), 163_u8);
///     assert_eq!(a.get_ubyte_(7), 189_u8);
///     assert_eq!(a.get_ubyte_(8), 88_u8);
///     assert_eq!(a.get_ubyte_(9), 136_u8);
///     assert_eq!(a.get_ubyte_(10), 206_u8);
///     assert_eq!(a.get_ubyte_(11), 126_u8);
///     assert_eq!(a.get_ubyte_(12), 26_u8);
///     assert_eq!(a.get_ubyte_(13), 59_u8);
///     assert_eq!(a.get_ubyte_(14), 18_u8);
///     assert_eq!(a.get_ubyte_(15), 255_u8);
///     assert_eq!(a.get_sbyte_(0), 79_i8);
///     assert_eq!(a.get_sbyte_(1), 11_i8);
///     assert_eq!(a.get_sbyte_(2), 74_i8);
///     assert_eq!(a.get_sbyte_(3), -15_i8);
///     assert_eq!(a.get_sbyte_(4), -11_i8);
///     assert_eq!(a.get_sbyte_(5), 104_i8);
///     assert_eq!(a.get_sbyte_(6), -93_i8);
///     assert_eq!(a.get_sbyte_(7), -67_i8);
///     assert_eq!(a.get_sbyte_(8), 88_i8);
///     assert_eq!(a.get_sbyte_(9), -120_i8);
///     assert_eq!(a.get_sbyte_(10), -50_i8);
///     assert_eq!(a.get_sbyte_(11), 126_i8);
///     assert_eq!(a.get_sbyte_(12), 26_i8);
///     assert_eq!(a.get_sbyte_(13), 59_i8);
///     assert_eq!(a.get_sbyte_(14), 18_i8);
///     assert_eq!(a.get_sbyte_(15), -1_i8);
/// 
///     #[cfg(target_pointer_width = "128")]
///     {
///         println!("a.get_usize() = {}", a.get_usize());
///         println!("a.get_ssize() = {}", a.get_ssize());
///         assert_eq!(a.get_usize(), 339047799029950809142362261752780557135_usize);
///         assert_eq!(a.get_ssize(), 1234567890987654321012345678987654321_isize);
///     }
///     #[cfg(target_pointer_width = "64")]
///     {
///         const N: usize = 2;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 13664881099896654671_usize);
///         assert_eq!(a.get_usize_(1), 18379818014235068504_usize);
///         assert_eq!(a.get_ssize_(0), -4781862973812896945_isize);
///         assert_eq!(a.get_ssize_(1), -66926059474483112_isize);
///     }
///     #[cfg(target_pointer_width = "32")]
///     {
///         const N: usize = 4;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 4048161615_usize);
///         assert_eq!(a.get_usize_(1), 3181603061_usize);
///         assert_eq!(a.get_usize_(2), 2127464536_usize);
///         assert_eq!(a.get_usize_(3), 4279384858_usize);
///         assert_eq!(a.get_ssize_(0), -246805681_isize);
///         assert_eq!(a.get_ssize_(1), -1113364235_isize);
///         assert_eq!(a.get_ssize_(2), 2127464536_isize);
///         assert_eq!(a.get_ssize_(3), -15582438_isize);
///     }
///     #[cfg(target_pointer_width = "16")]
///     {
///         const N: usize = 8;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 2895_usize);
///         assert_eq!(a.get_usize_(1), 61770_usize);
///         assert_eq!(a.get_usize_(2), 26869_usize);
///         assert_eq!(a.get_usize_(3), 48547_usize);
///         assert_eq!(a.get_usize_(4), 34904_usize);
///         assert_eq!(a.get_usize_(5), 32462_usize);
///         assert_eq!(a.get_usize_(6), 15130_usize);
///         assert_eq!(a.get_usize_(7), 65298_usize);
///         assert_eq!(a.get_ssize_(0), 2895_isize);
///         assert_eq!(a.get_ssize_(1), -3766_isize);
///         assert_eq!(a.get_ssize_(2), 26869_isize);
///         assert_eq!(a.get_ssize_(3), -16989_isize);
///         assert_eq!(a.get_ssize_(4), -30632_isize);
///         assert_eq!(a.get_ssize_(5), 32462_isize);
///         assert_eq!(a.get_ssize_(6), 15130_isize);
///         assert_eq!(a.get_ssize_(7), -238_isize);
///     }
///     #[cfg(target_pointer_width = "8")]
///     {
///         const N: usize = 16;
///         for i in 0..N
///             { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///         for i in 0..N
///             { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///         assert_eq!(a.get_usize_(0), 79_usize);
///         assert_eq!(a.get_usize_(1), 11_usize);
///         assert_eq!(a.get_usize_(2), 74_usize);
///         assert_eq!(a.get_usize_(3), 241_usize);
///         assert_eq!(a.get_usize_(4), 245_usize);
///         assert_eq!(a.get_usize_(5), 104_usize);
///         assert_eq!(a.get_usize_(6), 163_usize);
///         assert_eq!(a.get_usize_(7), 189_usize);
///         assert_eq!(a.get_usize_(8), 88_usize);
///         assert_eq!(a.get_usize_(9), 136_usize);
///         assert_eq!(a.get_usize_(10), 206_usize);
///         assert_eq!(a.get_usize_(11), 126_usize);
///         assert_eq!(a.get_usize_(12), 26_usize);
///         assert_eq!(a.get_usize_(13), 59_usize);
///         assert_eq!(a.get_usize_(14), 18_usize);
///         assert_eq!(a.get_usize_(15), 255_usize);
///         assert_eq!(a.get_ssize_(0), 79_isize);
///         assert_eq!(a.get_ssize_(1), 11_isize);
///         assert_eq!(a.get_ssize_(2), 74_isize);
///         assert_eq!(a.get_ssize_(3), -15_isize);
///         assert_eq!(a.get_ssize_(4), -11_isize);
///         assert_eq!(a.get_ssize_(5), 104_isize);
///         assert_eq!(a.get_ssize_(6), -93_isize);
///         assert_eq!(a.get_ssize_(7), -67_isize);
///         assert_eq!(a.get_ssize_(8), 88_isize);
///         assert_eq!(a.get_ssize_(9), -120_isize);
///         assert_eq!(a.get_ssize_(10), -50_isize);
///         assert_eq!(a.get_ssize_(11), 126_isize);
///         assert_eq!(a.get_ssize_(12), 26_isize);
///         assert_eq!(a.get_ssize_(13), 59_isize);
///         assert_eq!(a.get_ssize_(14), 18_isize);
///         assert_eq!(a.get_ssize_(15), -1_isize);
///     }
/// }
/// ```
/// 
/// Note that `get_usize()` and `get_ssize()` (including their indexed 
/// variants) are available only on architectures with supported pointer 
/// widths, such as 128-bit, 64-bit, 32-bit, 16-bit, or 8-bit systems.
/// 
/// `LongerUnion` can be used just like a `u128`, supporting all standard 
/// arithmetic operations, including addition, subtraction, 
/// multiplication, and division. Integrating it with `SmallUInt` 
/// provides enhanced functionality and convenience, often eliminating 
/// the need for an explicit `LongerUnion` import.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_longerunion = 123456789876543212345678987654321_u128.into_longerunion();
/// let b_longerunion = 876543210123456787654321012345678_u128.into_longerunion();
/// let c_longerunion = a_longerunion.wrapping_add(b_longerunion);
/// println!("{} + {} = {}", a_longerunion, b_longerunion, c_longerunion);
/// assert_eq!(c_longerunion.get(), 999999999999999999999999999999999_u128);
/// for i in 0..2
///     { println!("c_longerunion.get_ulong_({}) = {}", i, c_longerunion.get_ulong_(i)); }
/// assert_eq!(c_longerunion.get_ulong_(0), 4089650035136921599_u64);
/// assert_eq!(c_longerunion.get_ulong_(1), 54210108624275_u64);
/// for i in 0..4
///     { println!("c_longerunion.get_uint_({}) = {}", i, c_longerunion.get_uint_(i)); }
/// assert_eq!(c_longerunion.get_uint_(0), 4294967295_u32);
/// assert_eq!(c_longerunion.get_uint_(1), 952195849_u32);
/// assert_eq!(c_longerunion.get_uint_(2), 3326381459_u32);
/// assert_eq!(c_longerunion.get_uint_(3), 12621_u32);
/// for i in 0..8
///     { println!("c_longerunion.get_ushort_({}) = {}", i, c_longerunion.get_ushort_(i)); }
/// assert_eq!(c_longerunion.get_ushort_(0), 65535_u16);
/// assert_eq!(c_longerunion.get_ushort_(1), 65535_u16);
/// assert_eq!(c_longerunion.get_ushort_(2), 23305_u16);
/// assert_eq!(c_longerunion.get_ushort_(3), 14529_u16);
/// assert_eq!(c_longerunion.get_ushort_(4), 36243_u16);
/// assert_eq!(c_longerunion.get_ushort_(5), 50756_u16);
/// assert_eq!(c_longerunion.get_ushort_(6), 12621_u16);
/// assert_eq!(c_longerunion.get_ushort_(7), 0_u16);
/// for i in 0..16
///     { println!("c_longerunion.get_ubyte_({}) = {}", i, c_longerunion.get_ubyte_(i)); }
/// assert_eq!(c_longerunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(1), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(2), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(3), 255_u8);
/// assert_eq!(c_longerunion.get_ubyte_(4), 9_u8);
/// assert_eq!(c_longerunion.get_ubyte_(5), 91_u8);
/// assert_eq!(c_longerunion.get_ubyte_(6), 193_u8);
/// assert_eq!(c_longerunion.get_ubyte_(7), 56_u8);
/// assert_eq!(c_longerunion.get_ubyte_(8), 147_u8);
/// assert_eq!(c_longerunion.get_ubyte_(9), 141_u8);
/// assert_eq!(c_longerunion.get_ubyte_(10), 68_u8);
/// assert_eq!(c_longerunion.get_ubyte_(11), 198_u8);
/// assert_eq!(c_longerunion.get_ubyte_(12), 77_u8);
/// assert_eq!(c_longerunion.get_ubyte_(13), 49_u8);
/// assert_eq!(c_longerunion.get_ubyte_(14), 0_u8);
/// assert_eq!(c_longerunion.get_ubyte_(15), 0_u8);
/// 
/// let d_longerunion = b_longerunion - a_longerunion;
/// println!("{} - {} = {}", b_longerunion, a_longerunion, d_longerunion);
/// assert_eq!(d_longerunion.get(), 753086420246913575308642024691357_u128);
/// for i in 0..2
///     { println!("d_longunion.get_ulong_({}) = {}", i, d_longerunion.get_ulong_(i)); }
/// assert_eq!(d_longerunion.get_ulong_(0), 14084888390109238941_u64);
/// assert_eq!(d_longerunion.get_ulong_(1), 40824896645051_u64);
/// for i in 0..4
///     { println!("d_longunion.get_uint_({}) = {}", i, d_longerunion.get_uint_(i)); }
/// assert_eq!(d_longerunion.get_uint_(0), 2843481757_u32);
/// assert_eq!(d_longerunion.get_uint_(1), 3279393629_u32);
/// assert_eq!(d_longerunion.get_uint_(2), 1232496571_u32);
/// assert_eq!(d_longerunion.get_uint_(3), 9505_u32);
/// for i in 0..8
///     { println!("d_longunion.get_ushort_({}) = {}", i, d_longerunion.get_ushort_(i)); }
/// assert_eq!(d_longerunion.get_ushort_(0), 5789_u16);
/// assert_eq!(d_longerunion.get_ushort_(1), 43388_u16);
/// assert_eq!(d_longerunion.get_ushort_(2), 37725_u16);
/// assert_eq!(d_longerunion.get_ushort_(3), 50039_u16);
/// assert_eq!(d_longerunion.get_ushort_(4), 26555_u16);
/// assert_eq!(d_longerunion.get_ushort_(5), 18806_u16);
/// assert_eq!(d_longerunion.get_ushort_(6), 9505_u16);
/// assert_eq!(d_longerunion.get_ushort_(7), 0_u16);
/// for i in 0..16
///     { println!("d_longunion.get_ubyte_({}) = {}", i, d_longerunion.get_ubyte_(i)); }
/// assert_eq!(d_longerunion.get_ubyte_(0), 157_u8);
/// assert_eq!(d_longerunion.get_ubyte_(1), 22_u8);
/// assert_eq!(d_longerunion.get_ubyte_(2), 124_u8);
/// assert_eq!(d_longerunion.get_ubyte_(3), 169_u8);
/// assert_eq!(d_longerunion.get_ubyte_(4), 93_u8);
/// assert_eq!(d_longerunion.get_ubyte_(5), 147_u8);
/// assert_eq!(d_longerunion.get_ubyte_(6), 119_u8);
/// assert_eq!(d_longerunion.get_ubyte_(7), 195_u8);
/// assert_eq!(d_longerunion.get_ubyte_(8), 187_u8);
/// assert_eq!(d_longerunion.get_ubyte_(9), 103_u8);
/// assert_eq!(d_longerunion.get_ubyte_(10), 118_u8);
/// assert_eq!(d_longerunion.get_ubyte_(11), 73_u8);
/// assert_eq!(d_longerunion.get_ubyte_(12), 33_u8);
/// assert_eq!(d_longerunion.get_ubyte_(13), 37_u8);
/// assert_eq!(d_longerunion.get_ubyte_(14), 0_u8);
/// assert_eq!(d_longerunion.get_ubyte_(15), 0_u8);
/// 
/// let e_longerunion = d_longerunion * 3_u128.into_longerunion();
/// println!("{} * {} = {}", d_longerunion, 3_u128.into_longerunion(), e_longerunion);
/// assert_eq!(e_longerunion.get(), 2259259260740740725925926074074071_u128);
/// 
/// let f_longerunion = c_longerunion / 10_u128.into_longerunion();
/// println!("{} / {} = {}", c_longerunion, 10_u128.into_longerunion(), f_longerunion);
/// assert_eq!(f_longerunion.get(), 99999999999999999999999999999999_u128);
/// 
/// let g_longerunion = c_longerunion % 10_u128.into_longerunion();
/// println!("{} % {} = {}", c_longerunion, 10_u128.into_longerunion(), g_longerunion);
/// assert_eq!(g_longerunion.get(), 9_u128);
/// ```
///  
/// # Big-endian Support
/// Support for Big-Endian architectures is currently experimental and not 
/// recommended for production environments. Users assume all 
/// responsibility for any issues that may arise when using this crate 
/// on Big-Endian systems.
#[derive(Copy, Clone, Eq)]
#[allow(dead_code)]
pub union LongerUnion
{
    /// The 128-bit unsigned representation (for compatibility).
    this: u128,

    /// The 128-bit signed representation (for compatibility).
    that: i128,

    /// The primary 128-bit unsigned integer field.
    ulonger: u128,

    /// The primary 128-bit signed integer field.
    slonger: i128,

    /// Array of two 64-bit unsigned integers.
    ulong: [u64; 2],

    /// Array of two 64-bit signed integers.
    slong: [i64; 2],

    /// Array of four 32-bit unsigned integers.
    uint: [u32; 4],

    /// Array of four 32-bit signed integers.
    sint: [i32; 4],

    /// Array of eight 16-bit unsigned integers.
    ushort: [u16; 8],

    /// Array of eight 16-bit signed integers.
    sshort: [i16; 8],

    /// Array of sixteen 8-bit unsigned integers.
    ubyte: [u8; 16],

    /// Array of sixteen 8-bit signed integers.
    sbyte: [i8; 16],

    // / Pointer-sized representation (128-bit architectures).
    // #[cfg(target_pointer_width = "128")] u_size: usize,

    // / Pointer-sized signed representation (128-bit architectures).
    // #[cfg(target_pointer_width = "128")] s_size: isize,

    /// Array of two pointer-sized unsigned integers (64-bit architectures).
    #[cfg(target_pointer_width = "64")] u_size: [usize; 2],

    /// Array of two pointer-sized signed integers (64-bit architectures).
    #[cfg(target_pointer_width = "64")] s_size: [isize; 2],

    /// Array of four pointer-sized unsigned integers (32-bit architectures).
    #[cfg(target_pointer_width = "32")] u_size: [usize; 4],

    /// Array of four pointer-sized signed integers (32-bit architectures).
    #[cfg(target_pointer_width = "32")] s_size: [isize; 4],

    /// Array of eight pointer-sized unsigned integers (16-bit architectures).
    #[cfg(target_pointer_width = "16")] u_size: [usize; 8],

    /// Array of eight pointer-sized signed integers (16-bit architectures).
    #[cfg(target_pointer_width = "16")] s_size: [isize; 8],

    // / Array of sixteen pointer-sized unsigned integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] u_size: [usize; 16],

    // / Array of sixteen pointer-sized signed integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] s_size: [isize; 16],
}


impl LongerUnion
{
    // pub const fn new() -> Self
    /// Constructs a new `LongerUnion` with all fields initialized to zero.
    /// 
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let a = LongerUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u128);
    /// ```
    #[inline] pub const fn new() -> Self    { Self { ulonger: 0 } }

    // pub const fn new_with(ulonger: u128) -> Self
    /// Constructs a new `LongerUnion` initialized with the given `u128` value.
    /// 
    /// # Arguments
    /// * `ulonger`: The 128-bit unsigned integer value to initialize the union.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let a = LongerUnion::new_with(1234567890987654321012345678987654321_u128);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 1234567890987654321012345678987654321_u128);
    /// ```
    #[inline] pub const fn new_with(ulonger: u128) -> Self  { Self { ulonger } }

    // pub const fn new_with_signed(slonger: i128) -> Self
    /// Constructs a new `LongerUnion` initialized with the given `i128` value.
    /// 
    /// # Arguments
    /// * `slonger`: The 128-bit signed integer value to initialize the union.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let a = LongerUnion::new_with_signed(-1234567890987654321012345678987654321_i128);
    /// println!("a.get_signed() = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1234567890987654321012345678987654321_i128);
    /// ```
    #[inline] pub const fn new_with_signed(slonger: i128) -> Self   { Self { slonger } }

    // pub const fn new_with_ubytes(ubyte: [u8; 16]) -> Self
    /// Constructs a new `LongerUnion` initialized with the given byte array.
    /// 
    /// # Arguments
    /// * `ubyte`: An array of sixteen 8-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let arr = [79_u8, 11_u8, 74_u8, 241_u8, 245_u8, 104_u8, 163_u8, 189_u8, 88_u8, 136_u8, 206_u8, 126_u8, 26_u8, 59_u8, 18_u8, 255_u8];
    /// let a = LongerUnion::new_with_ubytes(arr);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    /// ```
    #[inline] pub const fn new_with_ubytes(ubyte: [u8; 16]) -> Self { Self { ubyte } }

    // pub const fn new_with_ushorts(ushort: [u16; 8]) -> Self
    /// Constructs a new `LongerUnion` initialized with the given 16-bit 
    /// word array.
    /// 
    /// # Arguments
    /// * `ushort`: An array of eight 16-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let arr = [2895_u16, 61770_u16, 26869_u16, 48547_u16, 34904_u16, 32462_u16, 15130_u16, 65298_u16];
    /// let a = LongerUnion::new_with_ushorts(arr);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    /// ```
    #[inline] pub const fn new_with_ushorts(ushort: [u16; 8]) -> Self   { Self { ushort } }

    // pub const fn new_with_uints(uint: [u32; 4]) -> Self
    /// Constructs a new `LongerUnion` initialized with the given 32-bit 
    /// word array.
    /// 
    /// # Arguments
    /// * `uint`: An array of four 32-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let arr = [4048161615_u32, 3181603061_u32, 2127464536_u32, 4279384858_u32];
    /// let a = LongerUnion::new_with_uints(arr);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    /// ```
    #[inline] pub const fn new_with_uints(uint: [u32; 4]) -> Self   { Self { uint } }

    // pub const fn new_with_ulongs(ulong: [u64; 2]) -> Self
    /// Constructs a new `LongerUnion` initialized with the given 64-bit 
    /// word array.
    /// 
    /// # Arguments
    /// * `ulong`: An array of two 64-bit unsigned integers.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let a = LongerUnion::new_with_ulongs([13664881099896654671_u64, 18379818014235068504_u64]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 339047799029950809142362261752780557135_u128);
    /// ```
    #[inline] pub const fn new_with_ulongs(ulong: [u64; 2]) -> Self { Self { ulong } }

    // pub const fn new_with_u128(num: u128) -> Self
    /// Constructs a new `LongerUnion` initialized with the given `u128` value.
    /// 
    /// # Arguments
    /// * `num`: The 128-bit unsigned integer value to initialize the union.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let a = LongerUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 123456789012345678901234567890123456789_u128);
    /// ```
    #[inline] pub const fn new_with_u128(num: u128) -> Self { Self { ulonger: num } }

    // pub const fn new_with_bool(b: bool) -> Self
    /// Constructs a new `LongerUnion` initialized based on the given boolean 
    /// value.
    /// 
    /// # Arguments
    /// * `b`: The boolean value. `true` becomes `1`, and `false` becomes `0`.
    ///
    /// # Returns
    /// A new `LongerUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let a = LongerUnion::new_with_bool(true);
    /// let b = LongerUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u128);
    /// assert_eq!(b.get(), 0_u128);
    /// ```
    #[inline] pub const fn new_with_bool(b: bool) -> Self   { Self { ulonger: b as u128 } }

    // pub fn get(self) -> u128
    /// Returns the union's value as a 128-bit unsigned integer.
    /// 
    /// # Returns
    /// The 128-bit unsigned integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let a = LongerUnion::new_with(98765432101234567898765432101234546789_u128);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 98765432101234567898765432101234546789_u128);
    /// ```
    #[inline] pub fn get(self) -> u128          { unsafe { self.ulonger } }

    // pub fn set(&mut self, val: u128)
    /// Sets the union's value from a 128-bit unsigned integer.
    /// 
    /// # Arguments
    /// * `val`: The 128-bit unsigned integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let mut a = LongerUnion::new();
    /// a.set(98765432101234567898765432101234546789_u128);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 98765432101234567898765432101234546789_u128);
    /// ```
    #[inline] pub fn set(&mut self, val: u128)  { self.ulonger = val; }

    // pub fn get_signed(self) -> i128
    /// Returns the union's value as a 128-bit signed integer.
    /// 
    /// # Returns
    /// The 128-bit signed integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let a = LongerUnion::new_with(234567890987654321012345678987654321234_u128);
    /// println!("a.get_signed() = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -105714475933284142451028928444113890222_i128);
    /// ```
    #[inline] pub fn get_signed(self) -> i128   { unsafe { self.slonger } }

    // pub fn set_signed(&mut self, val: i128)
    /// Sets the union's value from a 128-bit signed integer.
    /// 
    /// # Arguments
    /// * `val`: The 128-bit signed integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;    
    /// let mut a = LongerUnion::new();
    /// a.set_signed(-105714475933284142451028928444113890222_i128);
    /// println!("a.get_signed() = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -105714475933284142451028928444113890222_i128);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i128)   { self.slonger = val; }

    crate::number::get_set_longer_fit!();

    crate::number::get_set_byte!(16);

    crate::number::get_set_short!(8);

    crate::number::get_set_int!(4);

    crate::number::get_set_long!(2);

    // #[cfg(target_pointer_width = "128")]    crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "64")]     crate::number::get_set_size!(2);
    #[cfg(target_pointer_width = "32")]     crate::number::get_set_size!(4);
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_size!(8);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_size!(16);
    
    crate::number::integer_union_methods!(u128);

    // pub fn as_ptr(&self) -> *const u128
    /// Returns a raw pointer to the union's memory as a `u128`.
    /// 
    /// # Returns
    /// A `*const u128` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let a = LongerUnion::new_with(0x12345678876543211234567887654321_u128);
    /// let ptr = a.as_ptr();
    /// unsafe { assert_eq!(*ptr, 0x12345678876543211234567887654321_u128); }
    /// ```
    #[inline] pub fn as_ptr(&self) -> *const u128 { unsafe { self.ubyte.as_ptr() as *const u128 } }

    // pub fn as_mut_ptr(&mut self) -> *mut u128
    /// Returns a mutable raw pointer to the union's memory as a `u128`.
    /// 
    /// # Returns
    /// A `*mut u128` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::LongerUnion;
    /// let mut a = LongerUnion::new();
    /// let ptr = a.as_mut_ptr();
    /// unsafe { *ptr = 0x87654321123456788765432112345678_u128; }
    /// assert_eq!(a.get(), 0x87654321123456788765432112345678_u128);
    /// ```
    #[inline] pub fn as_mut_ptr(&mut self) -> *mut u128 { unsafe { self.ubyte.as_mut_ptr() as *mut u128 } }
}



crate::number::operators_for_integer_unions_impl! { LongerUnion }

crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { LongerUnion, usize }

crate::number::shift_ops_for_integer_unions_by_union_impl! { LongerUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongerUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongerUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongerUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { LongerUnion, SizeUnion }

crate::number::format_for_integer_unions_impl! { LongerUnion }



impl Ord for LongerUnion
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


impl Debug for LongerUnion
{
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    /// Formats the value using the given formatter for debugging purposes.
    /// 
    /// # Formatting Options
    /// The `:?` format specifier provides standard debug output, while the 
    /// alternate `:#?` specifier produces pretty-printed output for 
    /// enhanced readability.
    /// 
    /// # Example for the format specifier :?
    /// ```
    /// use cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:?}"), "LongerUnion { this: 216825577908592784562140039541644754667, that: -123456789012345678901234567890123456789, ulonger: 216825577908592784562140039541644754667, slonger: -123456789012345678901234567890123456789, ulong: [6134004772338302699, 11754138130946064698], slong: [6134004772338302699, -6692605942763486918], uint: [1371963115, 1428184279, 2682913082, 2736723546], sint: [1371963115, 1428184279, -1612054214, -1558243750], ushort: [32491, 20934, 23767, 21792, 314, 40938, 5722, 41759], sshort: [32491, 20934, 23767, 21792, 314, -24598, 5722, -23777], ubyte: [235, 126, 198, 81, 215, 92, 32, 85, 58, 1, 234, 159, 90, 22, 31, 163], sbyte: [-21, 126, -58, 81, -41, 92, 32, 85, 58, 1, -22, -97, 90, 22, 31, -93], u_size: [6134004772338302699, 11754138130946064698], s_size: [6134004772338302699, -6692605942763486918] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_longer = LongerUnion::new_with_signed(-123456789012345678901234567890123456789_i128);
    /// println!("a_longer = {:#?}", a_longer);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_longer:#?}"), r#"LongerUnion {
    ///     this: 216825577908592784562140039541644754667,
    ///     that: -123456789012345678901234567890123456789,
    ///     ulonger: 216825577908592784562140039541644754667,
    ///     slonger: -123456789012345678901234567890123456789,
    ///     ulong: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     slong: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
    ///     ],
    ///     uint: [
    ///         1371963115,
    ///         1428184279,
    ///         2682913082,
    ///         2736723546,
    ///     ],
    ///     sint: [
    ///         1371963115,
    ///         1428184279,
    ///         -1612054214,
    ///         -1558243750,
    ///     ],
    ///     ushort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         40938,
    ///         5722,
    ///         41759,
    ///     ],
    ///     sshort: [
    ///         32491,
    ///         20934,
    ///         23767,
    ///         21792,
    ///         314,
    ///         -24598,
    ///         5722,
    ///         -23777,
    ///     ],
    ///     ubyte: [
    ///         235,
    ///         126,
    ///         198,
    ///         81,
    ///         215,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         234,
    ///         159,
    ///         90,
    ///         22,
    ///         31,
    ///         163,
    ///     ],
    ///     sbyte: [
    ///         -21,
    ///         126,
    ///         -58,
    ///         81,
    ///         -41,
    ///         92,
    ///         32,
    ///         85,
    ///         58,
    ///         1,
    ///         -22,
    ///         -97,
    ///         90,
    ///         22,
    ///         31,
    ///         -93,
    ///     ],
    ///     u_size: [
    ///         6134004772338302699,
    ///         11754138130946064698,
    ///     ],
    ///     s_size: [
    ///         6134004772338302699,
    ///         -6692605942763486918,
    ///     ],
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
        let mut ff = f.debug_struct("LongerUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ulonger", &self.get_ulonger())
            .field("slonger", &self.get_slonger())
            .field("ulong", &[self.get_ulong_(0), self.get_ulong_(1)])
            .field("slong", &[self.get_slong_(0), self.get_slong_(1)])
            .field("uint", &[self.get_uint_(0), self.get_uint_(1), self.get_uint_(2), self.get_uint_(3)])
            .field("sint", &[self.get_sint_(0), self.get_sint_(1), self.get_sint_(2), self.get_sint_(3)])
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1), self.get_ushort_(2), self.get_ushort_(3), self.get_ushort_(4), self.get_ushort_(5), self.get_ushort_(6), self.get_ushort_(7)])
            .field("sshort", &[self.get_sshort_(0), self.get_sshort_(1), self.get_sshort_(2), self.get_sshort_(3), self.get_sshort_(4), self.get_sshort_(5), self.get_sshort_(6), self.get_sshort_(7)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3), self.get_ubyte_(4), self.get_ubyte_(5), self.get_ubyte_(6), self.get_ubyte_(7), self.get_ubyte_(8), self.get_ubyte_(9), self.get_ubyte_(10), self.get_ubyte_(11), self.get_ubyte_(12), self.get_ubyte_(13), self.get_ubyte_(14), self.get_ubyte_(15)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3), self.get_sbyte_(4), self.get_sbyte_(5), self.get_sbyte_(6), self.get_sbyte_(7), self.get_sbyte_(8), self.get_sbyte_(9), self.get_sbyte_(10), self.get_sbyte_(11), self.get_sbyte_(12), self.get_sbyte_(13), self.get_sbyte_(14), self.get_sbyte_(15)]);
        // #[cfg(target_pointer_width = "128")] ff.field("u_size",  &self.get_usize())
                                                // .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "64")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        #[cfg(target_pointer_width = "32")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7)]);
        // #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3), self.get_usize_(4), self.get_usize_(5), self.get_usize_(6), self.get_usize_(7), self.get_usize_(8), self.get_usize_(9), self.get_usize_(10), self.get_usize_(11), self.get_usize_(12), self.get_usize_(13), self.get_usize_(14), self.get_usize_(15)])
                                                // .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3), self.get_ssize_(4), self.get_ssize_(5), self.get_ssize_(6), self.get_ssize_(7), self.get_ssize_(8), self.get_ssize_(9), self.get_ssize_(10), self.get_ssize_(11), self.get_ssize_(12), self.get_ssize_(13), self.get_ssize_(14), self.get_ssize_(15)]);
        ff.finish()
    }
}
