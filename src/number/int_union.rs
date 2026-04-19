// Copyright 2023, 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a 32-bit integer union for efficient memory sharing and byte-level
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

use crate::number::{ SmallUInt, ShortUnion, LongUnion, LongerUnion, SizeUnion };
use crate::number::{ union_calc_assign_to_calc, union_fmt_with_radix, union_fmt_with_exponent };

/// A 32-bit integer union that enables bit-level slicing and seamless 
/// conversion between various primitive types, including `u32`, `i32`, 
/// `u16`, `i16`, `u8`, and `i8`.
/// 
/// # Introduction
/// `IntUnion` provides efficient, bit-level access to 32-bit values. 
/// It allows manipulating the underlying memory as a single 32-bit word, 
/// two 16-bit words, or four 8-bit bytes, in both signed and unsigned formats.
/// 
/// # Quick Start
/// To use this union, import `cryptocol::number::IntUnion` 
/// as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::IntUnion;
/// ```
/// 
/// Use methods such as `get()`, `get_signed()`, `get_uint()`, and 
/// `get_sint()` to retrieve the underlying 32-bit value in various 
/// integer formats. You can also slice the data into two `u16` 
/// values using `get_ushort()` and `get_sshort()` (including their 
/// indexed variants), or into four `u8` values using `get_ubyte()` 
/// and `get_sbyte()`.
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::IntUnion;
/// 
/// let a = IntUnion::new_with_signed(-454688546_i32);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_uint() = {}", a.get_uint());
/// println!("a.get_sint() = {}", a.get_sint());
/// assert_eq!(a.get(), 3840278750_u32);
/// assert_eq!(a.get_signed(), -454688546_i32);
/// assert_eq!(a.get_uint(), 3840278750_u32);
/// assert_eq!(a.get_sint(), -454688546_i32);
/// 
/// for i in 0..2
///     { println!("a.get_ushort_({}) = {}", i, a.get_ushort_(i)); }
/// for i in 0..2
///     { println!("a.get_sshort_({}) = {}", i, a.get_sshort_(i)); }
/// for i in 0..4
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..4
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ushort_(0), 222_u16);
/// assert_eq!(a.get_ushort_(1), 58598_u16);
/// assert_eq!(a.get_sshort_(0), 222_i16);
/// assert_eq!(a.get_sshort_(1), -6938_i16);
/// assert_eq!(a.get_ubyte_(0), 222_u8);
/// assert_eq!(a.get_ubyte_(1), 0_u8);
/// assert_eq!(a.get_ubyte_(2), 230_u8);
/// assert_eq!(a.get_ubyte_(3), 228_u8);
/// assert_eq!(a.get_sbyte_(0), -34_i8);
/// assert_eq!(a.get_sbyte_(1), 0_i8);
/// assert_eq!(a.get_sbyte_(2), -26_i8);
/// assert_eq!(a.get_sbyte_(3), -28_i8);
/// #[cfg(target_pointer_width = "16")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 222_u16);
///     assert_eq!(a.get_usize_(1), 58598_u16);
///     assert_eq!(a.get_ssize_(0), 222_i16);
///     assert_eq!(a.get_ssize_(1), -6938_i16);
/// }
/// // #[cfg(target_pointer_width = "8")]
/// // {
/// //     const N: usize = 4;
/// //     for i in 0..N
/// //         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
/// //     for i in 0..N
/// //         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
/// //     assert_eq!(a.get_usize_(0), 222_u8);
/// //     assert_eq!(a.get_usize_(1), 0_u8);
/// //     assert_eq!(a.get_usize_(2), 230_u8);
/// //     assert_eq!(a.get_usize_(3), 228_u8);
/// //     assert_eq!(a.get_ssize_(0), -34_i8);
/// //     assert_eq!(a.get_ssize_(1), 0_i8);
/// //     assert_eq!(a.get_ssize_(2), -26_i8);
/// //     assert_eq!(a.get_ssize_(3), -28_i8);
/// // }
/// #[cfg(any(target_pointer_width = "32", target_arch = "wasm32"))]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 3840278750_u32);
///     assert_eq!(a.get_ssize(), -454688546_i32);
/// }
/// ```
/// 
/// Note that `get_usize()` and `get_ssize()` (including their indexed 
/// variants) are available only on architectures with supported pointer 
/// widths, such as 32-bit, 16-bit, or 8-bit systems.
/// 
/// `IntUnion` can be used just like a `u32`, supporting all standard 
/// arithmetic operations, including addition, subtraction, 
/// multiplication, and division. Integrating it with `SmallUInt` 
/// provides enhanced functionality and convenience, often eliminating 
/// the need for an explicit `IntUnion` import.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_intunion = 12345678_u32.into_intunion();
/// let b_intunion = 87654321_u32.into_intunion();
/// let c_intunion = a_intunion.wrapping_add(b_intunion);
/// println!("{} + {} = {}", a_intunion, b_intunion, c_intunion);
/// assert_eq!(c_intunion.get(), 99999999_u32);
/// for i in 0..2
///     { println!("c_intunion.get_ushort_({}) = {}", i, c_intunion.get_ushort_(i)); }
/// assert_eq!(c_intunion.get_ushort_(0), 57599_u16);
/// assert_eq!(c_intunion.get_ushort_(1), 1525_u16);
/// for i in 0..4
///     { println!("c_intunion.get_ubyte_({}) = {}", i, c_intunion.get_ubyte_(i)); }
/// assert_eq!(c_intunion.get_ubyte_(0), 255_u8);
/// assert_eq!(c_intunion.get_ubyte_(1), 224_u8);
/// assert_eq!(c_intunion.get_ubyte_(2), 245_u8);
/// assert_eq!(c_intunion.get_ubyte_(3), 5_u8);
/// 
/// let d_intunion = b_intunion - a_intunion;
/// println!("{} - {} = {}", b_intunion, a_intunion, d_intunion);
/// assert_eq!(d_intunion.get(), 75308643_u32);
/// for i in 0..2
///     { println!("d_shortunion.get_ushort_({}) = {}", i, d_intunion.get_ushort_(i)); }
/// assert_eq!(d_intunion.get_ushort_(0), 7779_u16);
/// assert_eq!(d_intunion.get_ushort_(1), 1149_u16);
/// for i in 0..4
///     { println!("d_shortunion.get_ubyte_({}) = {}", i, d_intunion.get_ubyte_(i)); }
/// assert_eq!(d_intunion.get_ubyte_(0), 99_u8);
/// assert_eq!(d_intunion.get_ubyte_(1), 30_u8);
/// assert_eq!(d_intunion.get_ubyte_(2), 125_u8);
/// assert_eq!(d_intunion.get_ubyte_(3), 4_u8);
/// 
/// let e_intunion = d_intunion * 3_u32.into_intunion();
/// println!("{} * {} = {}", d_intunion, 3_u32.into_intunion(), e_intunion);
/// assert_eq!(e_intunion.get(), 225925929_u32);
/// 
/// let f_intunion = c_intunion / 10_u32.into_intunion();
/// println!("{} / {} = {}", c_intunion, 10_u16.into_intunion(), f_intunion);
/// assert_eq!(f_intunion.get(), 9999999_u32);
/// 
/// let g_intunion = c_intunion % 10_u32.into_intunion();
/// println!("{} % {} = {}", c_intunion, 10_u16.into_intunion(), g_intunion);
/// assert_eq!(g_intunion.get(), 9_u32);
/// ```
///  
/// # Big-endian Support
/// Support for Big-Endian architectures is currently experimental and not 
/// recommended for production environments. Users assume all 
/// responsibility for any issues that may arise when using this crate 
/// on Big-Endian systems.
#[derive(Copy, Clone, Eq)]
#[allow(dead_code)]
pub union IntUnion
{
    /// The 32-bit unsigned representation (for compatibility).
    this: u32,

    /// The 32-bit signed representation (for compatibility).
    that: i32,

    /// The primary 32-bit unsigned integer field.
    uint: u32,

    /// The primary 32-bit signed integer field.
    sint: i32,

    /// Array of two 16-bit unsigned integers.
    ushort: [u16; 2],

    /// Array of two 16-bit signed integers.
    sshort: [i16; 2],

    /// Array of four 8-bit unsigned integers.
    ubyte: [u8; 4],

    /// Array of four 8-bit signed integers.
    sbyte: [i8; 4],

    /// Pointer-sized unsigned representation (for compatibility).
    #[cfg(any(target_pointer_width = "32", target_arch = "wasm32"))] pub u_size: usize,

    /// Pointer-sized signed representation (for compatibility).
    #[cfg(any(target_pointer_width = "32", target_arch = "wasm32"))] pub s_size: isize,

    /// Array of two pointer-sized unsigned integers (for compatibility).
    #[cfg(target_pointer_width = "16")] pub u_size: [usize; 2],

    /// Array of two pointer-sized signed integers (for compatibility).
    #[cfg(target_pointer_width = "16")] pub s_size: [isize; 2],

    // /// The usize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] pub u_size: [usize; 4],

    // /// The isize type array whose elements's size is 8-bit size
    // #[cfg(target_pointer_width = "8")] pub s_size: [isize; 4],
}



impl IntUnion
{
    // pub const fn new() -> Self
    /// Constructs a new `IntUnion` with all fields initialized to zero.
    /// 
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u32);
    /// ```
    #[inline] pub const fn new() -> Self    { Self { uint: 0 } }

    // pub const fn new_with(uint: u32) -> Self
    /// Constructs a new `IntUnion` initialized with the given `u32` value.
    /// 
    /// # Arguments
    /// * `uint`: The 32-bit unsigned integer value to initialize the union.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(1234567890_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 1234567890_u32);
    /// ```
    #[inline] pub const fn new_with(uint: u32) -> Self  { Self { uint } }

    // pub const fn new_with_signed(sint: i32) -> Self
    /// Constructs a new `IntUnion` initialized with the given `i32` value.
    /// 
    /// # Arguments
    /// * `sint`: The 32-bit signed integer value to initialize the union.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1234567890_i32);
    /// ```
    #[inline] pub const fn new_with_signed(sint: i32) -> Self   { Self { sint } }

    // pub const fn new_with_ubytes(ubyte: [u8; 4]) -> Self
    /// Constructs a new `IntUnion` initialized with the given byte array.
    /// 
    /// # Arguments
    /// * `ubyte`: An array of four 8-bit unsigned integers.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_ubytes([222_u8, 0_u8, 230_u8, 228_u8]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// ```
    #[inline] pub const fn new_with_ubytes(ubyte: [u8; 4]) -> Self  { Self { ubyte } }

    // pub const fn new_with_ushorts(ushort: [u16; 2]) -> Self
    /// Constructs a new `IntUnion` initialized with the given 16-bit word array.
    /// 
    /// # Arguments
    /// * `ushort`: An array of two 16-bit unsigned integers.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_ushorts([222_u16, 58598_u16]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// ```
    #[inline] pub const fn new_with_ushorts(ushort: [u16; 2]) -> Self   { Self { ushort } }

    // pub const fn new_with_u128(num: u128) -> Self
    /// Constructs a new `IntUnion` initialized with the lowest 32 bits of
    /// the given `u128` value.
    /// 
    /// # Arguments
    /// * `num`: The 128-bit unsigned integer to initialize from.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_u128(3840278750_u128);
    /// let b = IntUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 3840278750_u32);
    /// assert_eq!(b.get(), 2923004181_u32);
    /// ```
    #[inline] pub const fn new_with_u128(num: u128) -> Self { Self { uint: num as u32 } }

    // pub const fn new_with_bool(b: bool) -> Self
    /// Constructs a new `IntUnion` initialized based on the given boolean value.
    /// 
    /// # Arguments
    /// * `b`: The boolean value. `true` becomes `1`, and `false` becomes `0`.
    ///
    /// # Returns
    /// A new `IntUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with_bool(true);
    /// let b = IntUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u32);
    /// assert_eq!(b.get(), 0_u32);
    /// ```
    #[inline] pub const fn new_with_bool(b: bool) -> Self   { Self { uint: b as u32 } }

    // pub fn get(self) -> u32
    /// Returns the union's value as a 32-bit unsigned integer.
    /// 
    /// # Returns
    /// The 32-bit unsigned integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(987654321_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 987654321_u32);
    /// ```
    #[inline] pub fn get(self) -> u32           { unsafe { self.this } }

    // pub fn set(&mut self, val: u32)
    /// Sets the union's value from a 32-bit unsigned integer.
    /// 
    /// # Arguments
    /// * `val`: The 32-bit unsigned integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let mut a = IntUnion::new();
    /// a.set(987654321_u32);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 987654321_u32);
    /// ```
    #[inline] pub fn set(&mut self, val: u32)   { self.this = val; }

    // pub fn get_signed(self) -> i32
    /// Returns the union's value as a 32-bit signed integer.
    /// 
    /// # Returns
    /// The 32-bit signed integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let a = IntUnion::new_with(2345678901_u32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1949288395_i32);
    /// ```
    #[inline] pub fn get_signed(self) -> i32    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i32)
    /// Sets the union's value from a 32-bit signed integer.
    /// 
    /// # Arguments
    /// * `val`: The 32-bit signed integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;    
    /// let mut a = IntUnion::new();
    /// a.set_signed(-1949288395_i32);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1949288395_i32);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i32)    { self.that = val; }

    crate::number::get_set_int_fit!();

    crate::number::get_set_byte!(4);

    crate::number::get_set_short!(2);

    #[cfg(any(target_pointer_width = "32", target_arch = "wasm32"))]     crate::number::get_set_size_fit!();
    #[cfg(target_pointer_width = "16")]     crate::number::get_set_size!(2);
    // #[cfg(target_pointer_width = "8")]      crate::number::get_set_size!(4);

    crate::number::integer_union_methods!(u32);

    // pub fn as_ptr(&self) -> *const u32
    /// Returns a raw pointer to the union's memory as a `u32`.
    /// 
    /// # Returns
    /// A `*const u32` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let a = IntUnion::new_with(0x12345678);
    /// let ptr = a.as_ptr();
    /// unsafe { assert_eq!(*ptr, 0x12345678); }
    /// ```
    #[inline] pub fn as_ptr(&self) -> *const u32 { unsafe { self.ubyte.as_ptr() as *const u32 } }

    // pub fn as_mut_ptr(&mut self) -> *mut u32
    /// Returns a mutable raw pointer to the union's memory as a `u32`.
    /// 
    /// # Returns
    /// A `*mut u32` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::IntUnion;
    /// let mut a = IntUnion::new();
    /// let ptr = a.as_mut_ptr();
    /// unsafe { *ptr = 0x87654321; }
    /// assert_eq!(a.get(), 0x87654321);
    /// ```
    #[inline] pub fn as_mut_ptr(&mut self) -> *mut u32 { unsafe { self.ubyte.as_mut_ptr() as *mut u32 } }
}



crate::number::operators_for_integer_unions_impl! { IntUnion }

crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { IntUnion, usize }

crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { IntUnion, SizeUnion }

crate::number::format_for_integer_unions_impl! { IntUnion }



impl Ord for IntUnion
{
    // fn cmp(&self, other: &Self) -> Ordering
    /// This method returns an Ordering between self and other.
    /// 
    /// # Returns
    /// An Ordering between self and other.
    /// 
    /// # Features
    /// By convention, self.cmp(&other) returns the ordering matching
    /// the expression self `<operator>` other if true.
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.get().cmp(&other.get())
    }
}


impl Debug for IntUnion
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
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:?}", a_int);
    /// #[cfg(any(target_pointer_width = "64", target_arch = "wasm64"))]
    /// assert_eq!(format!("{a_int:?}"), "IntUnion { this: 3060399406, that: -1234567890, uint: 3060399406, sint: -1234567890, ushort: [64814, 46697], sshort: [-722, -18839], ubyte: [46, 253, 105, 182], sbyte: [46, -3, 105, -74] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// Using the alternate format specifier `:#?` for pretty-printed debug output:
    /// ```
    /// use cryptocol::number::*;
    /// let a_int = IntUnion::new_with_signed(-1234567890_i32);
    /// println!("a_int = {:#?}", a_int);
    /// #[cfg(any(target_pointer_width = "64", target_arch = "wasm64"))]
    /// assert_eq!(format!("{a_int:#?}"), r#"IntUnion {
    ///     this: 3060399406,
    ///     that: -1234567890,
    ///     uint: 3060399406,
    ///     sint: -1234567890,
    ///     ushort: [
    ///         64814,
    ///         46697,
    ///     ],
    ///     sshort: [
    ///         -722,
    ///         -18839,
    ///     ],
    ///     ubyte: [
    ///         46,
    ///         253,
    ///         105,
    ///         182,
    ///     ],
    ///     sbyte: [
    ///         46,
    ///         -3,
    ///         105,
    ///         -74,
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
        let mut ff = f.debug_struct("IntUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("uint", &self.get_uint())
            .field("sint", &self.get_sint())
            .field("ushort", &[self.get_ushort_(0), self.get_ushort_(1)])
            .field("sshort", &[self.get_sshort_(0),  self.get_sshort_(1)])
            .field("ubyte", &[self.get_ubyte_(0), self.get_ubyte_(1), self.get_ubyte_(2), self.get_ubyte_(3)])
            .field("sbyte", &[self.get_sbyte_(0), self.get_sbyte_(1), self.get_sbyte_(2), self.get_sbyte_(3)]);
        #[cfg(any(target_pointer_width = "32", target_arch = "wasm32"))] ff.field("u_size", &self.get_usize())
                                                .field("s_size", &self.get_ssize());
        #[cfg(target_pointer_width = "16")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1)])
                                                .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1)]);
        // #[cfg(target_pointer_width = "8")] ff.field("u_size", &[self.get_usize_(0), self.get_usize_(1), self.get_usize_(2), self.get_usize_(3)])
        //                                         .field("s_size", &[self.get_ssize_(0), self.get_ssize_(1), self.get_ssize_(2), self.get_ssize_(3)]);
        ff.finish()
    }
}
