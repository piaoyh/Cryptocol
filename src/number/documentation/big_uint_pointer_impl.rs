// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]


use std::fmt::{ Error, Formatter, Display, Debug, Pointer };
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
pub struct BigUInt<T, const N: usize>
where T: SmallUInt
{
    // Dummy struct for documentation
    #[allow(dead_code)] number: [T; N],
    #[allow(dead_code)] flag: u8,
}



/// p formatting.
/// - The Pointer trait should format its output as a memory location.
/// - This is commonly presented as hexadecimal.
/// - Printing of pointers is not a reliable way to discover how Rust programs
///   are implemented.
/// - The act of reading an address changes the program itself, and may change
///   how the data is represented in memory, and may affect which optimizations
///   are applied to the code.
/// - The printed pointer values are not guaranteed to be stable nor unique
///   identifiers of objects.
/// - Rust allows moving values to different memory locations, and may reuse
///   the same memory locations for different purposes.
/// - There is no guarantee that the printed value can be converted back to a pointer.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Pointer.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Pointer for BigUInt<T, N>
where T: SmallUInt
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
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:p}", a_biguint);
    /// let txt = format!("{:p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958aab0"); // can be different everytime
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:20p}", a_biguint);
    /// let txt = format!("{:20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b0b0      "); // can be different everytime
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:020p}", a_biguint);
    /// let txt = format!("{:020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958aae0"); // can be different everytime
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<p}", a_biguint);
    /// let txt = format!("{:<p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b0e0"); // can be different everytime
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<20p}", a_biguint);
    /// let txt = format!("{:<20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958ab10      "); // can be different everytime
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:<020p}", a_biguint);
    /// let txt = format!("{:<020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b1a0"); // can be different everytime
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:!<p}", a_biguint);
    /// let txt = format!("{:!<p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958ab40"); // can be different everytime
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:@<20p}", a_biguint);
    /// let txt = format!("{:@<20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b1d0@@@@@@"); // can be different everytime
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:#<020p}", a_biguint);
    /// let txt = format!("{:#<020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958ab70"); // can be different everytime
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>p}", a_biguint);
    /// let txt = format!("{:>p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958b200"); // can be different everytime
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>20p}", a_biguint);
    /// let txt = format!("{:>20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "      0x7ffcd958aba0"); // can be different everytime
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:>020p}", a_biguint);
    /// let txt = format!("{:>020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b230"); // can be different everytime
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:$>p}", a_biguint);
    /// let txt = format!("{:$>p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958abd0"); // can be different everytime
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:%>20p}", a_biguint);
    /// let txt = format!("{:%>20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "%%%%%%0x7ffcd958b110"); // can be different everytime
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^>020p}", a_biguint);
    /// let txt = format!("{:^>020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958a750"); // can be different everytime
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^p}", a_biguint);
    /// let txt = format!("{:^p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958a850"); // can be different everytime
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^20p}", a_biguint);
    /// let txt = format!("{:^20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "   0x7ffcd958b140   "); // can be different everytime
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:^020p}", a_biguint);
    /// let txt = format!("{:^020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958b170"); // can be different everytime
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:&^p}", a_biguint);
    /// let txt = format!("{:&^p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x7ffcd958af40"); // can be different everytime
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:*^20p}", a_biguint);
    /// let txt = format!("{:*^20p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "***0x7ffcd958af70***"); // can be different everytime
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u32);
    /// 
    /// let a_biguint = U256::from_str("12345678901234567890123456789012345678901234567890123456789012345678901234567").unwrap();
    /// println!("{:_^020p}", a_biguint);
    /// let txt = format!("{:_^020p}", a_biguint);
    /// println!("{}", txt);
    /// // assert_eq!(txt, "0x0000007ffcd958afa0"); // can be different everytime
    /// ```
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error>
    {
        unimplemented!(); // Dummy code for documentation
    }
}