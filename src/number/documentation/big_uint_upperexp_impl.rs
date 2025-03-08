// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]


use std::fmt::{ Error, Formatter, Display, Debug, UpperExp };
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



/// E formatting.
/// - The UpperExp trait should format its output in scientific notation
///   with a upper-case E.
/// - For more information on the trait UpperExp,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.UpperExp.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> UpperExp for BigUInt<T, N>
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
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:E}", a_biguint);
    /// let txt = format!("{:E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:100E}", a_biguint);
    /// let txt = format!("{:100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:0100E}", a_biguint);
    /// let txt = format!("{:0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:20.9E}", a_biguint);
    /// let txt = format!("{:20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76      ");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:020.9E}", a_biguint);
    /// let txt = format!("{:020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:21.13E}", a_biguint);
    /// let txt = format!("{:21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76   ");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:021.13E}", a_biguint);
    /// let txt = format!("{:021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<E}", a_biguint);
    /// let txt = format!("{:<E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<100E}", a_biguint);
    /// let txt = format!("{:<100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76                   ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<0100E}", a_biguint);
    /// let txt = format!("{:<0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<20.9E}", a_biguint);
    /// let txt = format!("{:<20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76      ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<020.9E}", a_biguint);
    /// let txt = format!("{:<020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<21.13E}", a_biguint);
    /// let txt = format!("{:<21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76   ");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<021.13E}", a_biguint);
    /// let txt = format!("{:<021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!<E}", a_biguint);
    /// let txt = format!("{:<E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@<100E}", a_biguint);
    /// let txt = format!("{:@<100E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#<0100E}", a_biguint);
    /// let txt = format!("{:#<0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$<20.9E}", a_biguint);
    /// let txt = format!("{:$<20.9E}", a_biguint);
    /// assert_eq!(txt, "1.234567890E76$$$$$$");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%<020.9E}", a_biguint);
    /// let txt = format!("{:%<020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^<21.13E}", a_biguint);
    /// let txt = format!("{:^<21.13E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901235E76^^^");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&<021.13E}", a_biguint);
    /// let txt = format!("{:&<021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>E}", a_biguint);
    /// let txt = format!("{:>E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>100E}", a_biguint);
    /// let txt = format!("{:>100E}", a_biguint);
    /// assert_eq!(txt, "                   1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>0100E}", a_biguint);
    /// let txt = format!("{:>0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>20.9E}", a_biguint);
    /// let txt = format!("{:>20.9E}", a_biguint);
    /// assert_eq!(txt, "      1.234567890E76");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>020.9E}", a_biguint);
    /// let txt = format!("{:>020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>21.13E}", a_biguint);
    /// let txt = format!("{:>21.13E}", a_biguint);
    /// assert_eq!(txt, "   1.2345678901235E76");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>021.13E}", a_biguint);
    /// let txt = format!("{:>021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!>E}", a_biguint);
    /// let txt = format!("{:>E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@>100E}", a_biguint);
    /// let txt = format!("{:@>100E}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@@@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 31
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#>0100E}", a_biguint);
    /// let txt = format!("{:#>0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 32
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$>20.9E}", a_biguint);
    /// let txt = format!("{:$>20.9E}", a_biguint);
    /// assert_eq!(txt, "$$$$$$1.234567890E76");
    /// ```
    /// 
    /// # Example 33
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%>020.9E}", a_biguint);
    /// let txt = format!("{:%>020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 34
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^>21.13E}", a_biguint);
    /// let txt = format!("{:^>21.13E}", a_biguint);
    /// assert_eq!(txt, "^^^1.2345678901235E76");
    /// ```
    /// 
    /// # Example 35
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&>021.13E}", a_biguint);
    /// let txt = format!("{:&>021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 36
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^E}", a_biguint);
    /// let txt = format!("{:^E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 37
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^100E}", a_biguint);
    /// let txt = format!("{:^100E}", a_biguint);
    /// assert_eq!(txt, "         1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76          ");
    /// ```
    /// 
    /// # Example 38
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^0100E}", a_biguint);
    /// let txt = format!("{:^0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 39
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^20.9E}", a_biguint);
    /// let txt = format!("{:^20.9E}", a_biguint);
    /// assert_eq!(txt, "   1.234567890E76   ");
    /// ```
    /// 
    /// # Example 40
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^020.9E}", a_biguint);
    /// let txt = format!("{:^020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 41
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^21.13E}", a_biguint);
    /// let txt = format!("{:^21.13E}", a_biguint);
    /// assert_eq!(txt, " 1.2345678901235E76  ");
    /// ```
    /// 
    /// # Example 42
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^021.13E}", a_biguint);
    /// let txt = format!("{:^021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    /// 
    /// # Example 43
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!^E}", a_biguint);
    /// let txt = format!("{:^E}", a_biguint);
    /// assert_eq!(txt, "1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 44
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@^100E}", a_biguint);
    /// let txt = format!("{:@^100E}", a_biguint);
    /// assert_eq!(txt, "@@@@@@@@@1.2345678901234567890123456789012345678901234567890123456789012345678901234567E76@@@@@@@@@@");
    /// ```
    /// 
    /// # Example 45
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#^0100E}", a_biguint);
    /// let txt = format!("{:#^0100E}", a_biguint);
    /// assert_eq!(txt, "00000000000000000001.2345678901234567890123456789012345678901234567890123456789012345678901234567E76");
    /// ```
    /// 
    /// # Example 46
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$^20.9E}", a_biguint);
    /// let txt = format!("{:$^20.9E}", a_biguint);
    /// assert_eq!(txt, "$$$1.234567890E76$$$");
    /// ```
    /// 
    /// # Example 47
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%^020.9E}", a_biguint);
    /// let txt = format!("{:%^020.9E}", a_biguint);
    /// assert_eq!(txt, "0000001.234567890E76");
    /// ```
    /// 
    /// # Example 48
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^^21.13E}", a_biguint);
    /// let txt = format!("{:^^21.13E}", a_biguint);
    /// assert_eq!(txt, "^1.2345678901235E76^^");
    /// ```
    /// 
    /// # Example 49
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u8);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&^021.13E}", a_biguint);
    /// let txt = format!("{:&^021.13E}", a_biguint);
    /// assert_eq!(txt, "0001.2345678901235E76");
    /// ```
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error>
    {
        unimplemented!(); // Dummy code for documentation
    }
}