// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]
pub fn main()
{
    biguint_exponentiation_logarithm_uint_main();
    biguint_exponentiation_logarithm_biguint_main();
}


fn biguint_exponentiation_logarithm_uint_main()
{
    biguint_pow_uint();
    biguint_pow_assign_uint();
    biguint_wrapping_pow_uint();
    biguint_wrapping_pow_assign_uint();
    biguint_overflowing_pow_uint();
    biguint_overflowing_pow_assign_uint();

    biguint_iroot_uint();
    biguint_iroot_assign_uint();

    biguint_ilog_uint();
    biguint_ilog_assign_uint();
}

fn biguint_pow_uint()
{
    println!("biguint_pow_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u8;
    let res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = 100_u8;
    let res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = 0_u8;
    let res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 0 ** 30
    let a_biguint = UU32::zero();
    let exp = 30_u8;
    let res = a_biguint.pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = UU32::zero();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.pow_uint(_exp);
    println!("---------------------------");
}

fn biguint_pow_assign_uint()
{
    println!("biguint_pow_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 100_u8;
    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 10_u8;
    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    a_biguint.pow_assign_uint(exp);
    println!("After a_biguint.pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // _a_biguint.pow_assign_uint(_exp);
    println!("---------------------------");
}

fn biguint_wrapping_pow_uint()
{
    println!("biguint_wrapping_pow_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u32;
    let res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 100_u32;
    let res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::zero();
    let exp = 30_u8;
    let res = a_biguint.wrapping_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = UU32::zero();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.wrapping_pow_uint(_exp);
    println!("---------------------------");
}

fn biguint_wrapping_pow_assign_uint()
{
    println!("biguint_wrapping_pow_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 100_u8;
    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    a_biguint.wrapping_pow_assign_uint(exp);
    println!("After a_biguint.wrapping_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    println!("Originally, _a_biguint = {}", _a_biguint);
    // It will panic.
    // _a_biguint.wrapping_pow_assign_uint(_exp);
    println!("---------------------------");
}

fn biguint_overflowing_pow_uint()
{
    println!("biguint_overflowing_pow_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u32;
    let (res, overflow) = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res, overflow);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(overflow, false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 100_u32;
    let (res, overflow) = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res, overflow);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(overflow, true);
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let (res, overflow) = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res, overflow);
    assert_eq!(res.to_string(), "1");
    assert_eq!(overflow, false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::zero();
    let exp = 30_u8;
    let (res, overflow) = a_biguint.overflowing_pow_uint(exp);
    println!("{} ** {} = {}\noverflow = {}", a_biguint, exp, res, overflow);
    assert_eq!(res.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = UU32::zero();
    let _exp = 0_u8;
    // It will panic.
    // let (res, overflow) = _a_biguint.overflowing_pow_uint(_exp);

    println!("---------------------------");
}

fn biguint_overflowing_pow_assign_uint()
{
    println!("biguint_overflowing_pow_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    let overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}), a_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_string("1000000000000000000000000000000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 3_u8;
    let overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}), a_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "51484102413631087777415798035541167055393351402420714880745735202410401366016");
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    let overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}), a_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 10_u8;
    let overflow = a_biguint.overflowing_pow_assign_uint(exp);
    println!("After a_biguint.overflowing_pow_assign_uint({}), a_biguint = {}\noverflow = {}", exp, a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // let overflow = _a_biguint.overflowing_pow_assign_uint(_exp);
    println!("---------------------------");
}

fn biguint_iroot_uint()
{
    println!("biguint_iroot_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 8_u8;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "100000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 65_u8;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "9");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 212_u8;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "2");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 213_u8;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = u128::MAX;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let exp = 6_u8;
    let res = a_biguint.iroot_uint(exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.iroot_uint(_exp);

    let _a_biguint = U256::one();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.iroot_uint(_exp);

    let _a_biguint = U256::zero();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.iroot_uint(_exp);
    println!("---------------------------");
}

fn biguint_iroot_assign_uint()
{
    println!("biguint_iroot_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 8_u8;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "100000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 65_u8;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "9");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 212_u8;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 213_u8;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = u128::MAX;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 6_u8;
    a_biguint.iroot_assign_uint(exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _exp = 0_u8;
    // It will panic.
    // a_biguint.iroot_assign_uint(exp);

    let mut _a_biguint = U256::one();
    let _exp = 0_u8;
    // It will panic.
    // a_biguint.iroot_assign_uint(exp);

    let mut _a_biguint = U256::zero();
    let _exp = 0_u8;
    // It will panic.
    // a_biguint.iroot_assign_uint(exp);
    println!("---------------------------");
}

fn biguint_ilog_uint()
{
    println!("biguint_ilog_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    let res = a_biguint.ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "2");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 10_u8;
    let res = a_biguint.ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "64");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = 6_u8;
    let res = a_biguint.ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 6_u8;
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::one();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::one();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog(&_base);
    println!("---------------------------");
}

fn biguint_ilog_assign_uint()
{
    println!("biguint_ilog_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    a_biguint.ilog_assign_uint(base);
    println!("After a_biguint.ilog_assign_uint({}), a_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = 10_u8;
    a_biguint.ilog_assign_uint(base);
    println!("After a_biguint.ilog_assign_uint({}), a_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "64");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = 6_u8;
    a_biguint.ilog_assign_uint(base);
    println!("After a_biguint.ilog_assign_uint({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 6_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let _a_biguint = U256::one();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);

    let _a_biguint = U256::one();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.ilog_assign_uint(_base);
    println!("---------------------------");
}


fn biguint_exponentiation_logarithm_biguint_main()
{
    biguint_next_power_of_two();
    biguint_next_power_of_two_assign();
    biguint_is_power_of_two();

    biguint_pow();
    biguint_pow_assign();
    biguint_wrapping_pow();
    biguint_wrapping_pow_assign();
    biguint_overflowing_pow();
    biguint_overflowing_pow_assign();

    biguint_iroot();
    biguint_iroot_assign();
    biguint_isqrt();
    biguint_isqrt_assign();

    biguint_ilog();
    biguint_ilog_assign();

    biguint_ilog2();
    biguint_ilog2_assign();

    biguint_ilog10();
    biguint_ilog10_assign();
}

fn biguint_next_power_of_two()
{
    println!("biguint_next_power_of_two");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.next_power_of_two();
    println!("The next power of two is {}.", res);
    assert_eq!(res.to_string(), "170141183460469231731687303715884105728");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Maximum
    let a_biguint = U256::max();
    let res = a_biguint.next_power_of_two();
    println!("The next power of two is {}.", res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Minimum
    let a_biguint = U256::zero();
    let res = a_biguint.next_power_of_two();
    println!("The next power of two is {}.", res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_next_power_of_two_assign()
{
    println!("biguint_next_power_of_two_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.next_power_of_two_assign();
    println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "170141183460469231731687303715884105728");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Maximum
    let mut a_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.next_power_of_two_assign();
    println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Minimum
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.next_power_of_two_assign();
    println!("After a_biguint.next_power_of_two_assign(), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_power_of_two()
{
    println!("biguint_is_power_of_two()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.is_power_of_two();
    println!("Is {} the power of two? - {}.", a_biguint, res);
    assert_eq!(res, false);

    // Normal case 2
    let a_biguint = U256::from_str("170141183460469231731687303715884105728").unwrap();
    let res = a_biguint.is_power_of_two();
    println!("Is {} the power of two? - {}.", a_biguint, res);
    assert_eq!(res, true);

    // Maximum
    let a_biguint = U256::max();
    let res = a_biguint.is_power_of_two();
    println!("Is {} the power of two? - {}.", a_biguint, res);
    assert_eq!(res, false);

    // Minimum
    let a_biguint = U256::zero();
    let res = a_biguint.is_power_of_two();
    println!("Is {} the power of two? - {}.", a_biguint, res);
    assert_eq!(res, true);
    println!("---------------------------");
}

fn biguint_pow()
{
    println!("biguint_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let res = a_biguint.pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let res = a_biguint.pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = UU32::zero();
    let res = a_biguint.pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let a_biguint = UU32::zero();
    let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Panic example
    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.pow(&_exp);
    println!("---------------------------");
}

fn biguint_pow_assign()
{
    println!("biguint_pow_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // normal exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    a_biguint.pow_assign(&exp);
    println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    a_biguint.pow_assign(&exp);
    println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    a_biguint.pow_assign(&exp);
    println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    a_biguint.pow_assign(&exp);
    println!("After a_biguint.pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic example
    let mut _a_biguint = U256::zero();
    let _exp = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // _a_biguint.pow_assign(&_exp);
    println!("---------------------------");
}

fn biguint_wrapping_pow()
{
    println!("biguint_wrapping_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let res = a_biguint.wrapping_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let res = a_biguint.wrapping_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = UU32::zero();
    let res = a_biguint.wrapping_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let a_biguint = UU32::zero();
    let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.wrapping_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Panic example
    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.wrapping_pow(&_exp);
    println!("---------------------------");
}

fn biguint_wrapping_pow_assign()
{
    println!("biguint_wrapping_pow_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // normal exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    a_biguint.wrapping_pow_assign(&exp);
    println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    a_biguint.wrapping_pow_assign(&exp);
    println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    a_biguint.wrapping_pow_assign(&exp);
    println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    a_biguint.wrapping_pow_assign(&exp);
    println!("After a_biguint.wrapping_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic example
    let mut _a_biguint = U256::zero();
    let _exp = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // _a_biguint.wrapping_pow_assign(&_exp);
    println!("---------------------------");
}

fn biguint_overflowing_pow()
{
    println!("biguint_overflowing_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let (res, overflow) = a_biguint.overflowing_pow(&exp);
    println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    assert_eq!(overflow, false);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let (res, overflow) = a_biguint.overflowing_pow(&exp);
    println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    assert_eq!(overflow, true);
    assert_eq!(res.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = UU32::zero();
    let (res, overflow) = a_biguint.overflowing_pow(&exp);
    println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    assert_eq!(overflow, false);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let a_biguint = UU32::zero();
    let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let (res, overflow) = a_biguint.overflowing_pow(&exp);
    println!("{} ** {} = {}, {}", a_biguint, exp, res, overflow);
    assert_eq!(overflow, false);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Panic example
    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let (res, overflow) = _a_biguint.overflowing_pow(&_exp);
    println!("---------------------------");
}

fn biguint_overflowing_pow_assign()
{
    println!("biguint_overflowing_pow_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // normal exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(30_u8);
    let overflow = a_biguint.overflowing_pow_assign(&exp);
    println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // wrapping (modular) exponentiation
    let mut a_biguint = U256::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(100_u8);
    let overflow = a_biguint.overflowing_pow_assign(&exp);
    println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.to_string(), "60053020119642567005817971699943807522652027577520184704273238430174760927232");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::zero();
    let overflow = a_biguint.overflowing_pow_assign(&exp);
    println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // 0 ** 123456789012345678901234567890123456789
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let overflow = a_biguint.overflowing_pow_assign(&exp);
    println!("After a_biguint.overflowing_pow_assign({}), a_biguint = {}, {}", exp, a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Panic example
    let mut _a_biguint = U256::zero();
    let _exp = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // let overflow = _a_biguint.overflowing_pow_assign(&_exp);
    println!("---------------------------");
}

fn biguint_iroot()
{
    println!("biguint_iroot");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(8_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "100000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(65_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "9");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(212_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "2");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(213_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let exp = U256::from_uint(6_u8);
    let res = a_biguint.iroot(&exp);
    println!("The {}-th root of {} is {}.", exp, a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.iroot(&_exp);

    let _a_biguint = U256::one();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.iroot(&_exp);

    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.iroot(&_exp);
    println!("---------------------------");
}

fn biguint_iroot_assign()
{
    println!("biguint_iroot_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(8_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "100000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(65_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "9");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(212_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(213_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = U256::from_uint(6_u8);
    a_biguint.iroot_assign(&exp);
    println!("After a_biguint.iroot_assign_uint({}), a_biguint = {}.", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _exp = U256::zero();
    // It will panic.
    // _a_biguint.iroot_assign(&_exp);

    let mut _a_biguint = U256::one();
    let _exp = U256::zero();
    // It will panic.
    // _a_biguint.iroot_assign(&_exp);

    let mut _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // _a_biguint.iroot_assign(&_exp);
    println!("---------------------------");
}


fn biguint_isqrt()
{
    println!("biguint_isqrt");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let res = a_biguint.isqrt();
    println!("The square root of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::zero();
    let res = a_biguint.isqrt();
    println!("The second root of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_isqrt_assign()
{
    println!("biguint_isqrt_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.isqrt_assign();
    println!("After a_biguint.isqrt_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string_with_radix_and_stride(10, 4).unwrap(), "1_0000_0000_0000_0000_0000_0000_0000_0000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.isqrt_assign();
    println!("After a_biguint.isqrt_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_ilog()
{
    println!("biguint_ilog");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    let res = a_biguint.ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "2");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(10_u8);
    let res = a_biguint.ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "64");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let base = U256::from_uint(6_u8);
    let res = a_biguint.ilog(&base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog_uint(_base);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::from_uint(6_u8);
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::one();
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog(&_base);

    let _a_biguint = U256::one();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog(&_base);
    println!("---------------------------");
}

fn biguint_ilog_assign()
{
    println!("biguint_ilog_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    a_biguint.ilog_assign(&base);
    println!("After a_biguint.ilog_assign({}), a_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = U256::from_uint(10_u8);
    a_biguint.ilog_assign(&base);
    println!("After a_biguint.ilog_assign({}), a_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "64");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let base = U256::from_uint(6_u8);
    a_biguint.ilog_assign(&base);
    println!("After a_biguint.ilog_assign({}),\na_biguint = {}.", base, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();

    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::from_uint(6_u8);
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::one();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);

    let _a_biguint = U256::one();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.ilog_assign(&_base);
    println!("---------------------------");
}

fn biguint_ilog2()
{
    println!("biguint_ilog2");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_uint(64_u8);
    let res = a_biguint.ilog2();
    println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_uint(70_u8);
    let res = a_biguint.ilog2();
    println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let res = a_biguint.ilog2();
    println!("The base 2 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog2();
    println!("---------------------------");
}

fn biguint_ilog2_assign()
{
    println!("biguint_ilog2_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_uint(64_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog2_assign();
    println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(70_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog2_assign();
    println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog2_assign();
    println!("After a_biguint.ilog2_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _a_biguint = U256::zero();
    // It will panic.
    // _a_biguint.ilog2_assign();
    println!("---------------------------");
}

fn biguint_ilog10()
{
    println!("biguint_ilog10");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_uint(10000_u32);
    let res = a_biguint.ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "4");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);

    let a_biguint = U256::from_uint(12345_u32);
    let res = a_biguint.ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "4");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::one();
    let res = a_biguint.ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::zero();
    // It will panic.
    // let res = _a_biguint.ilog10();
    println!("---------------------------");
}

fn biguint_ilog10_assign()
{
    println!("biguint_ilog10_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_uint(10000_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog10_assign();
    println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "4");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_uint(12345_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog10_assign();
    println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "4");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.ilog10_assign();
    println!("After a_biguint.ilog10_assign(),\na_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _a_biguint = U256::zero();
    // It will panic.
    // _a_biguint.ilog10_assign();
    println!("---------------------------");
}