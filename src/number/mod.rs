// Copyright 2023, 2024, 2025, 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides various number types, including small primitive-based integers,
//! memory-sharing integer unions, and fixed-size big integers.
//!
//! # Introduction
//!
//! This module defines the foundational numeric types used throughout the
//! `cryptocol` ecosystem, categorized by their size and intended use:
//!
//! - **Small Numbers:** Primitive-sized integers (up to 128-bit) and specialized
//!   unions for bit manipulation.
//! - **Big Numbers:** High-precision, fixed-size unsigned integers designed for
//!   cryptographic operations.
//!
//! # Architectural Background
//!
//! ## Generic Programming for Primitives
//!
//! Rust lacks a unified trait for all primitive numeric types (e.g., `u8`
//! through `u128`, `usize`). The [`SmallUInt`] trait provides this abstraction,
//! enabling the implementation of generic algorithms across unsigned primitive
//! types.
//!
//! ## Extended Primitive Functionality
//!
//! Beyond standard arithmetic, [`SmallUInt`] introduces auxiliary methods for
//! common cryptographic and low-level tasks not natively provided by Rust
//! primitives.
//!
//! ## Arithmetic for Big Numbers
//!
//! Cryptographic algorithms, such as RSA, require integers far exceeding the
//! 128-bit limit of standard types. This module provides highly optimized
//! algorithms for calculating 256-bit, 1024-bit, and even larger integers.
//!
//! # Documentation Policy
//!
//! Parts of the documentation for this module are adapted from the Rust
//! standard library. This applies specifically to methods that implement
//! standard operations (e.g., arithmetic operators, `from_str`, `to_be`) to
//! maintain consistency with familiar interfaces and semantics.
//!
//! # 1. Small Numbers
//! _Foundational types for Big Numbers and other cryptographic modules._
//!
//! - **[`SmallUInt`]**: Trait providing a generic interface and additional
//!   methods for unsigned primitive integers.
// ! - [ ] **`SmallSInt` Trait**: (Planned) Core implementation for primitive
// !       signed data types. -- `SmallSInt`
// !       ===> Moved to Roadmap for ver. 2.0
//!
//! ## Integer Unions
//!
//! Unions designed for efficient type conversion and byte-level manipulation
//! between different integer sizes and arrays.
//!
//! - **[`ShortUnion`]**: Interop between `u16`, `i16`, `[u8; 2]`, and `[i8; 2]`.
//! - **[`IntUnion`]**: Interop between `u32`, `i32`, and 16/8-bit components.
//! - **[`LongUnion`]**: Interop between `u64`, `i64`, and 32/16/8-bit components.
//! - **[`LongerUnion`]**: Interop between `u128`, `i128`, and 64/32/16/8-bit
//!   components.
//! - **[`SizeUnion`]**: Interop between `usize`, `isize`, and platform-dependent
//!   components.
//! - **[`SharedValues`]**: Memory-sharing for cross-type truncation or
//!   zero-filling.
//! - **[`SharedArrays`]**: Memory-sharing for cross-type array conversions.
//!
//! # 2. Big Numbers
//! _Essential for Asymmetric-Key Algorithms and high-precision calculations._
//!
//! - **[`BigUInt`]**: Fixed-size big unsigned integers with user-defined
//!   bit-widths.
//! - **[`BigUInt_More`]**: Auxiliary methods for advanced calculations.
//! - **[`BigUInt_Modular`]**: Specialized modular arithmetic operations.
//! - **[`BigUInt_Panic_Free`]**: Safe, non-panicking arithmetic operations.
//! - **[`BigUInt_Prime`]**: Methods for primality testing and generation.
// ! - [ ] **Fixed-Size Big Signed Integers**: (Planned) Standard operations
// !       for large signed integers. -- `BigSInt`
// !       ===> Moved to Roadmap for ver. 2.0
// ! - [ ] **Variable-Size Big Signed Integers**: (Planned) Standard operations
// !       for large signed integers. -- `LargeInt`
// !       ===> Moved to Roadmap for ver. 2.0 or higher
//!
//! ## Predefined Big Unsigned Types
//!
//! Commonly used bit-widths are available as aliases:
//! - `U256`, `U512`, `U1024`, `U2048`, `U3072`, `U4096`, `U5120`, `U6144`,
//!   `U7168`, `U8192`, and `U16384`.
//! - Synonyms like `UU32` (256-bit) through `UU2048` (16384-bit) are also
//!   provided.
//!
//! # Quick Start
//!
//! - For `SmallUInt`, see [here](trait@SmallUInt#quick-start).
// ! - For `SmallSInt`, see [here](trait@SmallSInt#quick-start).
//! - For integer unions, see their respective `#quick-start` sections (e.g.,
//!   [`ShortUnion#quick-start`](union@ShortUnion#quick-start)).
//! - For `BigUInt`, see [here](struct@BigUInt#quick-start).
// ! - For `BigSInt`, see [here](struct@BigSInt#quick-start).
// ! - For `LargeInt`, see [here](struct@LargeInt#quick-start).



mod small_uint;
mod small_sint;
mod short_union;
mod int_union;
mod long_union;
mod longer_union;
mod size_union;
mod shared_values;
mod shared_arrays;
mod big_uint;

/// Additional methods for BigUInt
mod trait_big_uint_modular;
mod trait_big_uint_panic_free;
mod trait_big_uint_more;
mod trait_big_uint_prime;
mod number_errors;
mod macros_for_types;
mod macros_for_integer_unions;

/// Implementaion of trait SmallUInt for u8, u16, u32, u64, u128, and usize
mod trait_small_uint_for_unsigned_impl;

/// Implementaion of trait SmallUInt for ShortUnion, IntUnion, LongUnion, LongerUnion, and SizeUnion
mod trait_small_uint_for_integer_unions_impl;

/// Implementaion of various traits for BigUInt
mod traits_for_big_uint_impl;

/// Implementaion of BigUInt_Modular trait for BigUInt
mod trait_big_uint_modular_impl;

/// Implementaion of BigUInt_Panic_Free trait for BigUInt
mod trait_big_uint_panic_free_impl;

/// Implementaion of BigUInt_More trait for BigUInt
mod trait_big_uint_more_impl;

/// Implementaion of BigUInt_Prime trait for BigUInt
mod trait_big_uint_prime_impl;

pub use small_uint::*;
pub use small_sint::*;
pub use trait_big_uint_modular::*;
pub use trait_big_uint_panic_free::*;
pub use trait_big_uint_more::*;
pub use trait_big_uint_prime::*;
pub use short_union::*;
pub use int_union::*;
pub use long_union::*;
pub use longer_union::*;
pub use size_union::*;
pub use shared_values::*;
pub use shared_arrays::*;
pub use big_uint::*;
pub use number_errors::*;
use macros_for_integer_unions::*;
pub(crate) use trait_big_uint_prime_impl::A_LIST;



/// many *.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to all the *.rs in documentation folder.
mod documentation;
pub use documentation::*;


/********** FOR BIG-ENDIANNESS ONLY **********/

#[cfg(target_endian = "big")]
mod traits_for_big_uint_for_big_endian_impl;

#[cfg(target_endian = "big")]
use traits_for_big_uint_for_big_endian_impl::*;
