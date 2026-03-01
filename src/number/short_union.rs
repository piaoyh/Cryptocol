// Copyright 2023, 2024, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides a 16-bit integer union for efficient memory sharing and byte-level
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

use crate::number::{ SmallUInt, IntUnion, LongUnion, LongerUnion, SizeUnion };
use crate::number::{ union_calc_assign_to_calc, union_fmt_with_radix, union_fmt_with_exponent };

/// A 16-bit integer union that enables bit-level slicing and seamless 
/// conversion between various primitive types, including `u16`, `i16`, 
/// `u8`, and `i8`.
/// 
/// # Introduction
/// `ShortUnion` provides efficient, bit-level access to 16-bit values. 
/// It allows manipulating the underlying memory as a single 16-bit word, 
/// or two 8-bit bytes, in both signed and unsigned formats.
/// 
/// # Quick Start
/// To use this union, import `cryptocol::number::ShortUnion` 
/// as follows.
/// 
/// ## Example 1
/// ```
/// use cryptocol::number::ShortUnion;
/// ```
/// 
/// ## Example 2
/// ```
/// use cryptocol::number::ShortUnion;
/// let a = ShortUnion::new_with(55468_u16);
/// println!("a.get() = {}", a.get());
/// println!("a.get_signed() = {}", a.get_signed());
/// println!("a.get_ushort() = {}", a.get_ushort());
/// println!("a.get_sshort() = {}", a.get_sshort());
/// assert_eq!(a.get(), 55468_u16);
/// assert_eq!(a.get_signed(), -10068_i16);
/// assert_eq!(a.get_ushort(), 55468_u16);
/// assert_eq!(a.get_sshort(), -10068_i16);
/// 
/// for i in 0..2
///     { println!("a.get_ubyte_({}) = {}", i, a.get_ubyte_(i)); }
/// for i in 0..2
///     { println!("a.get_sbyte_({}) = {}", i, a.get_sbyte_(i)); }
/// assert_eq!(a.get_ubyte_(0), 172_u8);
/// assert_eq!(a.get_ubyte_(1), 216_u8);
/// assert_eq!(a.get_sbyte_(0), -84_i8);
/// assert_eq!(a.get_sbyte_(1), -40_i8);
/// 
/// #[cfg(target_pointer_width = "8")]
/// {
///     const N: usize = 2;
///     for i in 0..N
///         { println!("a.get_usize_({}) = {}", i, a.get_usize_(i)); }
///     for i in 0..N
///         { println!("a.get_ssize_({}) = {}", i, a.get_ssize_(i)); }
///     assert_eq!(a.get_usize_(0), 172_u8);
///     assert_eq!(a.get_usize_(1), 216_u8);
///     assert_eq!(a.get_usize_(0), -84_i8);
///     assert_eq!(a.get_usize_(1), -40_i8);
/// }
/// #[cfg(target_pointer_width = "16")]
/// {
///     println!("a.get_usize() = {}", a.get_usize());
///     println!("a.get_ssize() = {}", a.get_ssize());
///     assert_eq!(a.get_usize(), 55468_u16);
///     assert_eq!(a.get_ssize(), -10068_i16);
/// }
/// ```
/// 
/// Note that `get_usize()` and `get_ssize()` (including their indexed 
/// variants) are available only on architectures with supported pointer 
/// widths, such as 16-bit or 8-bit systems.
/// 
/// `ShortUnion` can be used just like a `u16`, supporting all standard 
/// arithmetic operations, including addition, subtraction, 
/// multiplication, and division. Integrating it with `SmallUInt` 
/// provides enhanced functionality and convenience, often eliminating 
/// the need for an explicit `ShortUnion` import.
/// 
/// ## Example 3
/// ```
/// use cryptocol::number::SmallUInt;
/// 
/// let a_shortunion = 1234_u16.into_shortunion();
/// let b_shortunion = 4321_u16.into_shortunion();
/// let c_shortunion = a_shortunion.wrapping_add(b_shortunion);
/// println!("{} + {} = {}", a_shortunion, b_shortunion, c_shortunion);
/// assert_eq!(c_shortunion.get(), 5555_u16);
/// for i in 0..2
///     { println!("c_shortunion.get_ubyte_({}) = {}", i, c_shortunion.get_ubyte_(i)); }
/// assert_eq!(c_shortunion.get_ubyte_(0), 179_u8);
/// assert_eq!(c_shortunion.get_ubyte_(1), 21_u8);
/// 
/// let d_shortunion = b_shortunion - a_shortunion;
/// println!("{} - {} = {}", b_shortunion, a_shortunion, d_shortunion);
/// assert_eq!(d_shortunion.get(), 3087_u16);
/// for i in 0..2
///     { println!("d_shortunion.get_ubyte_({}) = {}", i, d_shortunion.get_ubyte_(i)); }
/// assert_eq!(d_shortunion.get_ubyte_(0), 15_u8);
/// assert_eq!(d_shortunion.get_ubyte_(1), 12_u8);
/// 
/// let e_shortunion = d_shortunion * 3_u16.into_shortunion();
/// println!("{} * {} = {}", d_shortunion, 3_u16.into_shortunion(), e_shortunion);
/// assert_eq!(e_shortunion.get(), 9261_u16);
/// 
/// let f_shortunion = c_shortunion / 10_u16.into_shortunion();
/// println!("{} / {} = {}", c_shortunion, 10_u16.into_shortunion(), f_shortunion);
/// assert_eq!(f_shortunion.get(), 555_u16);
/// 
/// let g_shortunion = c_shortunion % 10_u16.into_shortunion();
/// println!("{} % {} = {}", c_shortunion, 10_u16.into_shortunion(), g_shortunion);
/// assert_eq!(g_shortunion.get(), 5_u16);
/// ```
/// 
/// # Big-endian Support
/// Support for Big-Endian architectures is currently experimental and not 
/// recommended for production environments. Users assume all 
/// responsibility for any issues that may arise when using this crate 
/// on Big-Endian systems.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub union ShortUnion
{
    /// The 16-bit unsigned representation (for compatibility).
    this: u16,

    /// The 16-bit signed representation (for compatibility).
    that: i16,

    /// The primary 16-bit unsigned integer field.
    ushort: u16,

    /// The primary 16-bit signed integer field.
    sshort: i16,

    /// Array of two 8-bit unsigned integers.
    ubyte: [u8; 2],

    /// Array of two 8-bit signed integers.
    sbyte: [i8; 2],

    /// Pointer-sized unsigned representation (16-bit architectures).
    #[cfg(target_pointer_width = "16")] u_size: usize,

    /// Pointer-sized signed representation (16-bit architectures).
    #[cfg(target_pointer_width = "16")] s_size: isize,

    // / Array of two pointer-sized unsigned integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] u_size: [usize; 2],

    // / Array of two pointer-sized signed integers (8-bit architectures).
    // #[cfg(target_pointer_width = "8")] s_size: [isize; 2],
}


impl ShortUnion
{
    // pub const fn new() -> Self
    /// Constructs a new `ShortUnion` with all fields initialized to zero.
    /// 
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new();
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 0_u16);
    /// ```
    #[inline] pub const fn new() -> Self    { Self { ushort: 0 } }

    // pub const fn new_with(ushort: u16) -> Self
    /// Constructs a new `ShortUnion` initialized with the given `u16` value.
    /// 
    /// # Arguments
    /// * `ushort`: The 16-bit unsigned integer value to initialize the union.
    ///
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with(1234_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 1234_u16);
    /// ```
    #[inline] pub const fn new_with(ushort: u16) -> Self    { Self { ushort } }

    // pub const fn new_with_signed(sshort: i16) -> Self
    /// Constructs a new `ShortUnion` initialized with the given `i16` value.
    /// 
    /// # Arguments
    /// * `sshort`: The 16-bit signed integer value to initialize the union.
    ///
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with_signed(-1234_i16);
    /// println!("a.get_signed() = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -1234_i16);
    /// ```
    #[inline] pub const fn new_with_signed(sshort: i16) -> Self { Self { sshort } }

    // pub const fn new_with_ubytes(ubyte: [u8; 2]) -> Self
    /// Constructs a new `ShortUnion` initialized with the given byte array.
    /// 
    /// # Arguments
    /// * `ubyte`: An array of two 8-bit unsigned integers.
    ///
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_ubytes([172_u8, 216_u8]);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// ```
    #[inline] pub const fn new_with_ubytes(ubyte: [u8; 2]) -> Self  { Self { ubyte } }

    // pub const fn new_with_u128(num: u128) -> Self
    /// Constructs a new `ShortUnion` initialized with the lowest 16 bits of
    /// the given `u128` value.
    /// 
    /// # Arguments
    /// * `num`: The 128-bit unsigned integer to initialize from.
    ///
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_u128(55468_u128);
    /// let b = ShortUnion::new_with_u128(123456789012345678901234567890123456789_u128);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// assert_eq!(b.get(), 33045_u16);
    /// ```
    #[inline] pub const fn new_with_u128(num: u128) -> Self { Self { ushort: num as u16 } }

    // pub const fn new_with_bool(b: bool) -> Self
    /// Constructs a new `ShortUnion` initialized based on the given boolean 
    /// value.
    /// 
    /// # Arguments
    /// * `b`: The boolean value. `true` becomes `1`, and `false` becomes `0`.
    ///
    /// # Returns
    /// A new `ShortUnion` instance.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with_bool(true);
    /// let b = ShortUnion::new_with_bool(false);
    /// println!("a = {}", a.get());
    /// println!("b = {}", b.get());
    /// assert_eq!(a.get(), 1_u16);
    /// assert_eq!(b.get(), 0_u16);
    /// ```
    #[inline] pub const fn new_with_bool(b: bool) -> Self   { Self { ushort: b as u16 } }

    // pub fn get(self) -> u16
    /// Returns the union's value as a 16-bit unsigned integer.
    /// 
    /// # Returns
    /// The 16-bit unsigned integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with(55468_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 55468_u16);
    /// ```
    #[inline] pub fn get(self) -> u16   { unsafe { self.this } }

    // pub fn set(&mut self, val: u16)
    /// Sets the union's value from a 16-bit unsigned integer.
    /// 
    /// # Arguments
    /// * `val`: The 16-bit unsigned integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let mut a = ShortUnion::new();
    /// a.set(54321_u16);
    /// println!("a = {}", a.get());
    /// assert_eq!(a.get(), 54321_u16);
    /// ```
    #[inline] pub fn set(&mut self, val: u16)   { self.this = val; }

    // pub fn get_signed(self) -> i16
    /// Returns the union's value as a 16-bit signed integer.
    /// 
    /// # Returns
    /// The 16-bit signed integer representation.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let a = ShortUnion::new_with(54321_u16);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -11215_i16);
    /// ```
    #[inline] pub fn get_signed(self) -> i16    { unsafe { self.that } }

    // pub fn set_signed(&mut self, val: i16)
    /// Sets the union's value from a 16-bit signed integer.
    /// 
    /// # Arguments
    /// * `val`: The 16-bit signed integer value to set.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;    
    /// let mut a = ShortUnion::new();
    /// a.set_signed(-11215_i16);
    /// println!("a = {}", a.get_signed());
    /// assert_eq!(a.get_signed(), -11215_i16);
    /// ```
    #[inline] pub fn set_signed(&mut self, val: i16)    { self.that = val; }

    crate::number::get_set_short_fit!();

    crate::number::get_set_byte!(2);

    // #[cfg(target_pointer_width = "8")]  crate::number::get_set_size!(2);
    #[cfg(target_pointer_width = "16")] crate::number::get_set_size_fit!();

    crate::number::integer_union_methods!(u16);

    // pub fn as_ptr(&self) -> *const u16
    /// Returns a raw pointer to the union's memory as a `u16`.
    /// 
    /// # Returns
    /// A `*const u16` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let a = ShortUnion::new_with(0x1234_u16);
    /// let ptr = a.as_ptr();
    /// unsafe { assert_eq!(*ptr, 0x1234_u16); }
    /// ```
    #[inline] pub fn as_ptr(&self) -> *const u16 { unsafe { self.ubyte.as_ptr() as *const u16 } }

    // pub fn as_mut_ptr(&mut self) -> *mut u16
    /// Returns a mutable raw pointer to the union's memory as a `u16`.
    /// 
    /// # Returns
    /// A `*mut u16` pointer to the underlying memory.
    /// 
    /// # Examples
    /// ```
    /// use cryptocol::number::ShortUnion;
    /// let mut a = ShortUnion::new();
    /// let ptr = a.as_mut_ptr();
    /// unsafe { *ptr = 0x8765_u16; }
    /// assert_eq!(a.get(), 0x8765_u16);
    /// ```
    #[inline] pub fn as_mut_ptr(&mut self) -> *mut u16 { unsafe { self.ubyte.as_mut_ptr() as *mut u16 } }
}



crate::number::operators_for_integer_unions_impl! { ShortUnion }

crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i8 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i16 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i32 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i64 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, i128 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, isize }

crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u8 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u16 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u32 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u64 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, u128 }
crate::number::shift_ops_for_integer_unions_impl! { ShortUnion, usize }

crate::number::shift_ops_for_integer_unions_by_union_impl! { ShortUnion, ShortUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { ShortUnion, IntUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { ShortUnion, LongUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { ShortUnion, LongerUnion }
crate::number::shift_ops_for_integer_unions_by_union_impl! { ShortUnion, SizeUnion }

crate::number::format_for_integer_unions_impl! { ShortUnion }



impl Debug for ShortUnion
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
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:?}"), "ShortUnion { this: 53191, that: -12345, ushort: 53191, sshort: -12345, ubyte: [199, 207], sbyte: [-57, -49] }");
    /// ```
    /// 
    /// # Example for the format specifier :#?
    /// ```
    /// use cryptocol::number::*;
    /// let a_short = ShortUnion::new_with_signed(-12345_i16);
    /// println!("a_short = {:#?}", a_short);
    /// #[cfg(target_pointer_width = "64")] assert_eq!(format!("{a_short:#?}"), r#"ShortUnion {
    ///     this: 53191,
    ///     that: -12345,
    ///     ushort: 53191,
    ///     sshort: -12345,
    ///     ubyte: [
    ///         199,
    ///         207,
    ///     ],
    ///     sbyte: [
    ///         -57,
    ///         -49,
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
        let mut ff = f.debug_struct("ShortUnion");
        ff.field("this", &self.get())
            .field("that", &self.get_signed())
            .field("ushort", &self.get_ushort())
            .field("sshort", &self.get_sshort())
            .field("ubyte",  &[self.get_ubyte_(0), self.get_ubyte_(1)])
            .field("sbyte",  &[self.get_sbyte_(0), self.get_sbyte_(1)]);
         #[cfg(target_pointer_width = "16")] ff.field("u_size", unsafe { &self.get_usize() } )
                                                .field("s_size", unsafe { &self.get_ssize() } );
        //  #[cfg(target_pointer_width = "8")] ff.field("u_size", unsafe { &[self.get_usize(0), self.get_usize(1)] } )
        //                                         .field("s_size", unsafe { &[self.get_ssize(0), self.get_isize(1)] } );
         ff.finish()
    }
}
