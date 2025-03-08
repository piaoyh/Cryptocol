// Copyright 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(unused)]


use std::fmt::{ Error, Formatter, Display, Debug, Octal };
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



/// o formatting.
/// - The Octal trait should format its output as a number in base-8.
/// - The alternate flag, #, adds a 0o in front of the output.
/// - For more information on the trait LowerHex,
///   [read more](https://doc.rust-lang.org/std/fmt/trait.Octal.html).
/// - For more information on formatters,
///   see [the module-level documentation](https://doc.rust-lang.org/std/fmt/index.html).
impl<T, const N: usize> Octal for BigUInt<T, N>
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
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:o}", a_biguint);
    /// let txt = format!("{:o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 2
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:102o}", a_biguint);
    /// let txt = format!("{:102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");
    /// ```
    /// 
    /// # Example 3
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:0102o}", a_biguint);
    /// let txt = format!("{:0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 4
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#o}", a_biguint);
    /// let txt = format!("{:#o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 5
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#102o}", a_biguint);
    /// let txt = format!("{:#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");
    /// ```
    /// 
    /// # Example 6
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#0102o}", a_biguint);
    /// let txt = format!("{:#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 7
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>102o}", a_biguint);
    /// let txt = format!("{:>102o}", a_biguint);
    /// assert_eq!(txt, "                11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 8
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>0102o}", a_biguint);
    /// let txt = format!("{:>0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 9
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^102o}", a_biguint);
    /// let txt = format!("{:^102o}", a_biguint);
    /// assert_eq!(txt, "        11506117236125542215231050004463337025330420704216361264762044667420601552357042554013        ");
    /// ```
    /// 
    /// # Example 10
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^0102o}", a_biguint);
    /// let txt = format!("{:^0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 11
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<102o}", a_biguint);
    /// let txt = format!("{:<102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013                ");
    /// ```
    /// 
    /// # Example 12
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<0102o}", a_biguint);
    /// let txt = format!("{:<0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 13
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>102o}", a_biguint);
    /// let txt = format!("{:!>102o}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!!!11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 14
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>0102o}", a_biguint);
    /// let txt = format!("{:@>0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 15
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^102o}", a_biguint);
    /// let txt = format!("{:#^102o}", a_biguint);
    /// assert_eq!(txt, "########11506117236125542215231050004463337025330420704216361264762044667420601552357042554013########");
    /// ```
    /// 
    /// # Example 16
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^0102o}", a_biguint);
    /// let txt = format!("{:$^0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 17
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<102o}", a_biguint);
    /// let txt = format!("{:%<102o}", a_biguint);
    /// assert_eq!(txt, "11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 18
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:*<0102o}", a_biguint);
    /// let txt = format!("{:*<0102o}", a_biguint);
    /// assert_eq!(txt, "000000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 19
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#102o}", a_biguint);
    /// let txt = format!("{:>#102o}", a_biguint);
    /// assert_eq!(txt, "              0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 20
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:>#0102o}", a_biguint);
    /// let txt = format!("{:>#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 21
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#102o}", a_biguint);
    /// let txt = format!("{:^#102o}", a_biguint);
    /// assert_eq!(txt, "       0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013       ");
    /// ```
    /// 
    /// # Example 22
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^#0102o}", a_biguint);
    /// let txt = format!("{:^#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 23
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#102o}", a_biguint);
    /// let txt = format!("{:<#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013              ");
    /// ```
    /// 
    /// # Example 24
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:<#0102o}", a_biguint);
    /// let txt = format!("{:<#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 25
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:!>#102o}", a_biguint);
    /// let txt = format!("{:!>#102o}", a_biguint);
    /// assert_eq!(txt, "!!!!!!!!!!!!!!0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 26
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:@>#0102o}", a_biguint);
    /// let txt = format!("{:@>#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 27
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:#^#102o}", a_biguint);
    /// let txt = format!("{:#^#102o}", a_biguint);
    /// assert_eq!(txt, "#######0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013#######");
    /// ```
    /// 
    /// # Example 28
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:$^#0102o}", a_biguint);
    /// let txt = format!("{:$^#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    /// 
    /// # Example 29
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:%<#102o}", a_biguint);
    /// let txt = format!("{:%<#102o}", a_biguint);
    /// assert_eq!(txt, "0o11506117236125542215231050004463337025330420704216361264762044667420601552357042554013%%%%%%%%%%%%%%");
    /// ```
    /// 
    /// # Example 30
    /// ```
    /// use std::str::FromStr;
    /// use cryptocol::define_utypes_with;
    /// define_utypes_with!(u128);
    /// 
    /// let a_biguint = U256::from_str("69743176821145534028236692093846345739169743176821145534028236692093846345739").unwrap();
    /// println!("{:^<#0102o}", a_biguint);
    /// let txt = format!("{:^<#0102o}", a_biguint);
    /// assert_eq!(txt, "0o0000000000000011506117236125542215231050004463337025330420704216361264762044667420601552357042554013");
    /// ```
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error>
    {
        unimplemented!(); // Dummy code for documentation
    }
}