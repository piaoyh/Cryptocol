// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]


use std::fmt::{ Error, Formatter, Display, Debug };
use std::cmp::{ PartialEq, PartialOrd };
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
                Shl, ShlAssign, Shr, ShrAssign, 
                Add, AddAssign, Sub, SubAssign, Mul, MulAssign,
                Div, DivAssign, Rem, RemAssign };

use crate::number::SmallUInt;

/// big_uint.rs was too big because of documentation and plenty of examples
/// So, in order to provide documentation without `docs.rs`'s failing
/// generating documentation, dummy codes were made and documentation and
/// examples were moved to big_uint_arithmetic.rs.
struct BigUInt<T, const N: usize>
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



/// Format trait for an empty format, {}.
/// - Implementing this trait for a type will automatically implement the
///   ToString trait for the type, allowing the usage of the .to_string()
///   method.
/// - Prefer implementing the Display trait for a type, rather than ToString.
/// - Display is similar to Debug, but Display is for user-facing output,
///   and so cannot be derived.
/// - For more information on the trait Display,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Display.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Display for BigUInt<T, N>
where T: SmallUInt + Copy + Clone + Display + Debug + ToString
        + Add<Output=T> + AddAssign + Sub<Output=T> + SubAssign
        + Mul<Output=T> + MulAssign + Div<Output=T> + DivAssign
        + Rem<Output=T> + RemAssign
        + Shl<Output=T> + ShlAssign + Shr<Output=T> + ShrAssign
        + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign
        + BitXor<Output=T> + BitXorAssign + Not<Output=T>
        + PartialEq + PartialOrd
{
    // fn fmt(&self, f: &mut Formatter) -> fmt::Result
    /// Formats the value using the given formatter.
    /// 
    /// # Arguments
    /// `f` is a buffer, this method must write the formatted string into it,
    /// and is of the type `&mut Formatter`.
    /// 
    /// # Features
    /// Automagically the function `to_string()` will be implemented. So, you
    /// can use the function `to_string()` and the macro `println!()`.
    /// [Read more](https://doc.rust-lang.org/core/fmt/trait.Display.html#tymethod.fmt)
    /// 
    /// # Errors
    /// This function should return Err if, and only if, the provided Formatter
    /// returns Err. String formatting is considered an infallible operation;
    /// this function only returns a Result because writing to the underlying
    /// stream might fail and it must provide a way to propagate the fact that
    /// an error has occurred back up the stack.
    /// 
    /// # Example 1
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{}", a_biguint);
    /// assert_eq!(a_biguint.to_string(), "69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>80}", a_biguint);
    /// let txt = format!("{:>80}", a_biguint);
    /// assert_eq!(txt, "   69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>080}", a_biguint);
    /// let txt = format!("{:>080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^80}", a_biguint);
    /// let txt = format!("{:^80}", a_biguint);
    /// assert_eq!(txt, " 69743176821145534028236692093846345739169743176821145534028236692093846345739  ");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^080}", a_biguint);
    /// let txt = format!("{:^080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<80}", a_biguint);
    /// let txt = format!("{:<80}", a_biguint);
    /// assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739   ");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<080}", a_biguint);
    /// let txt = format!("{:<080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>80}", a_biguint);
    /// let txt = format!("{:!>80}", a_biguint);
    /// assert_eq!(txt, "!!!69743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>080}", a_biguint);
    /// let txt = format!("{:@>080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^80}", a_biguint);
    /// let txt = format!("{:#^80}", a_biguint);
    /// assert_eq!(txt, "#69743176821145534028236692093846345739169743176821145534028236692093846345739##");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^080}", a_biguint);
    /// let txt = format!("{:$^080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<80}", a_biguint);
    /// let txt = format!("{:%<80}", a_biguint);
    /// assert_eq!(txt, "69743176821145534028236692093846345739169743176821145534028236692093846345739%%%");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<080}", a_biguint);
    /// let txt = format!("{:^<080}", a_biguint);
    /// assert_eq!(txt, "00069743176821145534028236692093846345739169743176821145534028236692093846345739");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("1234567_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890_1234567890").unwrap();
    /// let txt = a_biguint.to_string();
    /// println!("{}", txt);
    /// assert_eq!(txt, "12345671234567890123456789012345678901234567890123456789012345678901234567890");
    /// ```
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error>
    {
        unimplemented!(); // Dummy code for documentation
    }
}