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
// #[allow(dead_code)]``
pub fn main()
{
    biguint_add();
    biguint_sub();
    biguint_mul();
    biguint_div();
    biguint_rem();
    biguint_next_multiple();
}

fn biguint_add()
{
    biguint_carrying_add();
    biguint_carrying_add_assign();
    biguint_wrapping_add();
    biguint_wrapping_add_assign();
    biguint_overflowing_add();
    biguint_overflowing_add_assign();
    biguint_checked_add();
    biguint_unchecked_add();
    biguint_saturating_add();
    biguint_saturating_add_assign();
    biguint_modular_add();
    biguint_modular_add_assign();
    biguint_safe_add();
    biguint_safe_add_assign();
}

fn biguint_carrying_add()
{
    println!("biguint_carrying_add");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    let a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();

    let (c_biguint_lo, carry) = a_biguint_lo.carrying_add(&b_biguint_lo, false);
    let (c_biguint_hi, overflow) = a_biguint_hi.carrying_add(&b_biguint_hi, carry);
    println!("{}:{} + {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);
    assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    assert_eq!(overflow, false);
    assert_eq!(c_biguint_hi.is_overflow(), false);
    assert_eq!(c_biguint_hi.is_underflow(), false);
    assert_eq!(c_biguint_hi.is_infinity(), false);
    assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(c_biguint_hi.is_undefined(), false);
    assert_eq!(c_biguint_hi.is_left_carry(), false);
    assert_eq!(c_biguint_hi.is_right_carry(), false);

    assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    assert_eq!(carry, true);
    assert_eq!(c_biguint_lo.is_overflow(), true);
    assert_eq!(c_biguint_lo.is_underflow(), false);
    assert_eq!(c_biguint_lo.is_infinity(), false);
    assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(c_biguint_lo.is_undefined(), false);
    assert_eq!(c_biguint_lo.is_left_carry(), false);
    assert_eq!(c_biguint_lo.is_right_carry(), false);

    let a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    let a_biguint_lo = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    let b_biguint_hi = U256::zero();
    let b_biguint_lo = U256::one();

    let (c_biguint_lo, carry) = a_biguint_lo.carrying_add(&b_biguint_lo, false);
    let (c_biguint_hi, overflow) = a_biguint_hi.carrying_add(&b_biguint_hi, carry);
    println!("{}:{} + {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);
    assert_eq!(c_biguint_hi.to_string(), "0");
    assert_eq!(overflow, true);
    assert_eq!(c_biguint_hi.is_overflow(), true);
    assert_eq!(c_biguint_hi.is_underflow(), false);
    assert_eq!(c_biguint_hi.is_infinity(), false);
    assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(c_biguint_hi.is_undefined(), false);
    assert_eq!(c_biguint_hi.is_left_carry(), false);
    assert_eq!(c_biguint_hi.is_right_carry(), false);

    assert_eq!(c_biguint_lo.to_string(), "0");
    assert_eq!(carry, true);
    assert_eq!(c_biguint_lo.is_overflow(), true);
    assert_eq!(c_biguint_lo.is_underflow(), false);
    assert_eq!(c_biguint_lo.is_infinity(), false);
    assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(c_biguint_hi.is_undefined(), false);
    assert_eq!(c_biguint_lo.is_left_carry(), false);
    assert_eq!(c_biguint_lo.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_carrying_add_assign()
{
    println!("biguint_carrying_add_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint_hi = U256::from_str_radix("1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210", 16).unwrap();
    let mut a_biguint_lo = U256::from_str_radix("1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531", 16).unwrap();
    let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();
    println!("Originally, a_biguint_hi = {}\na_biguint_lo = {}\nb_biguint_hi = {}\nb_biguint_lo = {}", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    print!("Operation is: {}:{} + {}:{} ", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);

    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_underflow(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    assert_eq!(a_biguint_lo.is_overflow(), false);
    assert_eq!(a_biguint_lo.is_underflow(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(b_biguint_hi.is_overflow(), false);
    assert_eq!(b_biguint_hi.is_underflow(), false);
    assert_eq!(b_biguint_hi.is_infinity(), false);
    assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(b_biguint_hi.is_undefined(), false);
    assert_eq!(b_biguint_hi.is_left_carry(), false);
    assert_eq!(b_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_lo.is_overflow(), false);
    assert_eq!(b_biguint_lo.is_underflow(), false);
    assert_eq!(b_biguint_lo.is_infinity(), false);
    assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(b_biguint_lo.is_undefined(), false);
    assert_eq!(b_biguint_lo.is_left_carry(), false);
    assert_eq!(b_biguint_lo.is_right_carry(), false);

    let carry = a_biguint_lo.carrying_add_assign(&b_biguint_lo, false);
    let overflow = a_biguint_hi.carrying_add_assign(&b_biguint_hi, carry);

    println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);
    println!("After a_biguint_lo.carrying_add_assign(&b_biguint_lo, false), a_biguint_lo = {}", a_biguint_lo);
    println!("After a_biguint_hi.carrying_add_assign(&b_biguint_hi, {}), a_biguint_hi = {}", carry, a_biguint_hi);

    assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110");
    assert_eq!(carry, true);
    assert_eq!(a_biguint_lo.is_overflow(), true);
    assert_eq!(a_biguint_lo.is_underflow(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);
    
    assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_underflow(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_hi.is_overflow(), false);
    assert_eq!(b_biguint_hi.is_underflow(), false);
    assert_eq!(b_biguint_hi.is_infinity(), false);
    assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(b_biguint_hi.is_undefined(), false);
    assert_eq!(b_biguint_hi.is_left_carry(), false);
    assert_eq!(b_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_lo.is_overflow(), false);
    assert_eq!(b_biguint_lo.is_underflow(), false);
    assert_eq!(b_biguint_lo.is_infinity(), false);
    assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(b_biguint_lo.is_undefined(), false);
    assert_eq!(b_biguint_lo.is_left_carry(), false);
    assert_eq!(b_biguint_lo.is_right_carry(), false);

    let mut a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    let mut a_biguint_lo = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF", 16).unwrap();
    let b_biguint_hi = U256::zero();
    let b_biguint_lo = U256::one();
    println!("Originally, a_biguint_hi = {}\na_biguint_lo = {}\nb_biguint_hi = {}\nb_biguint_lo = {}", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);
    print!("Operation is: {}:{} + {}:{} ", a_biguint_hi, a_biguint_lo, b_biguint_hi, b_biguint_lo);

    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_underflow(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    assert_eq!(a_biguint_lo.is_overflow(), false);
    assert_eq!(a_biguint_lo.is_underflow(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(b_biguint_hi.is_overflow(), false);
    assert_eq!(b_biguint_hi.is_underflow(), false);
    assert_eq!(b_biguint_hi.is_infinity(), false);
    assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(b_biguint_hi.is_undefined(), false);
    assert_eq!(b_biguint_hi.is_left_carry(), false);
    assert_eq!(b_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_lo.is_overflow(), false);
    assert_eq!(b_biguint_lo.is_underflow(), false);
    assert_eq!(b_biguint_lo.is_infinity(), false);
    assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(b_biguint_lo.is_undefined(), false);
    assert_eq!(b_biguint_lo.is_left_carry(), false);
    assert_eq!(b_biguint_lo.is_right_carry(), false);

    let carry = a_biguint_lo.carrying_add_assign(&b_biguint_lo, false);
    let overflow = a_biguint_hi.carrying_add_assign(&b_biguint_hi, carry);

    println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("carry = {}, overflow = {}", carry, overflow);
    println!("After a_biguint_lo.carrying_add_assign(&b_biguint_lo, false), a_biguint_lo = {}", a_biguint_lo);
    println!("After a_biguint_hi.carrying_add_assign(&b_biguint_hi, {}), a_biguint_hi = {}", carry, a_biguint_hi);

    assert_eq!(a_biguint_lo.to_string(), "0");
    assert_eq!(carry, true);
    assert_eq!(a_biguint_lo.is_overflow(), true);
    assert_eq!(a_biguint_lo.is_underflow(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(a_biguint_hi.to_string(), "0");
    assert_eq!(overflow, true);
    assert_eq!(a_biguint_hi.is_overflow(), true);
    assert_eq!(a_biguint_hi.is_underflow(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_hi.is_overflow(), false);
    assert_eq!(b_biguint_hi.is_underflow(), false);
    assert_eq!(b_biguint_hi.is_infinity(), false);
    assert_eq!(b_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(b_biguint_hi.is_undefined(), false);
    assert_eq!(b_biguint_hi.is_left_carry(), false);
    assert_eq!(b_biguint_hi.is_right_carry(), false);

    assert_eq!(b_biguint_lo.is_overflow(), false);
    assert_eq!(b_biguint_lo.is_underflow(), false);
    assert_eq!(b_biguint_lo.is_infinity(), false);
    assert_eq!(b_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(b_biguint_lo.is_undefined(), false);
    assert_eq!(b_biguint_lo.is_left_carry(), false);
    assert_eq!(b_biguint_lo.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_add()
{
    println!("biguint_wrapping_add");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let one_biguint = U512::one();
    let res = a_biguint.wrapping_add(&one_biguint);
    println!("{} + 1 = {}", a_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::max();
    let one_biguint = U512::one();
    let res = b_biguint.wrapping_add(&one_biguint);
    println!("{} + {} = {}", b_biguint, one_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let c_biguint = U512::zero();
    let one_biguint = U512::one();
    let res = c_biguint.wrapping_add(&one_biguint);
    println!("{} + {} = {}", c_biguint, one_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_add_assign()
{
    println!("biguint_wrapping_add_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint.wrapping_add_assign(&one_biguint);
    println!("After a_biguint.wrapping_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, U512::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint.wrapping_add_assign(&one_biguint);
    println!("After a_biguint.wrapping_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_add_assign(&one_biguint);
    println!("After a_biguint.wrapping_add_assign(&U512::one()),\ta_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_add()
{
    println!("biguint_overflowing_add");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let one_biguint = U512::one();
    let (res, overflow) = a_biguint.overflowing_add(&one_biguint);
    println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, res, overflow);
    assert_eq!(overflow, false);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::max();
    let one_biguint = U512::one();
    let (res, overflow) = b_biguint.overflowing_add(&one_biguint);
    println!("{} + {} = {}, overflow = {}", b_biguint, one_biguint, res, overflow);
    assert_eq!(overflow, true);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let c_biguint = U512::max();
    let two_biguint = U512::from_uint(2_u8);
    let (res, overflow) = c_biguint.overflowing_add(&two_biguint);
    println!("{} + {} = {}, overflow = {}", c_biguint, two_biguint, res, overflow);
    assert_eq!(overflow, true);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_add_assign()
{
    println!("biguint_overflowing_add_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    let overflow = a_biguint.overflowing_add_assign(&one_biguint);
    println!("After a_biguint.overflowing_add_assign(&U512::one()), a_biguint = {}, overflow = {}", a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint, U512::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    let overflow = a_biguint.overflowing_add_assign(&one_biguint);
    println!("After a_biguint.overflowing_add_assign(&U512::one()),\ta_biguint = {}, overflow = {}", a_biguint, overflow);
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_add_assign(&one_biguint);
    println!("After a_biguint.overflowing_add_assign(&U512::one()),\ta_biguint = {}, overflow = {}", a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_add()
{
    println!("biguint_checked_add");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let one_biguint = U512::one();
    let res = a_biguint.checked_add(&one_biguint);
    match res
    {
        Some(r) => {
                println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, r, r.is_overflow());
                assert_eq!(r, U512::max());
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error: Overflow"); },
    }

    let b_biguint = U512::max();
    let one_biguint = U512::one();
    let res = b_biguint.checked_add(&one_biguint);
    match res
    {
        Some(r) => { println!("{} + {} = {}, overflow = {}", b_biguint, one_biguint, r, r.is_overflow()); },
        None => { 
                println!("Error: Overflow");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_unchecked_add()
{
    println!("biguint_unchecked_add()");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let one_biguint = U512::one();
    let res = a_biguint.unchecked_add(&one_biguint);
    println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, res, res.is_overflow());
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _b_biguint = U512::max();
    let _one_biguint = U512::one();
    // It will panic.
    // let res = _b_biguint.unchecked_add(&_one_biguint);
    println!("---------------------------");
}

fn biguint_saturating_add()
{
    println!("biguint_saturating_add");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let one_biguint = U512::one();
    let res = a_biguint.saturating_add(&one_biguint);
    println!("{} + {} = {}", a_biguint, one_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::max();
    let one_biguint = U512::one();
    let res: cryptocol::number::BigUInt<u64, 8> = b_biguint.saturating_add(&one_biguint);
    println!("{} + {} = {}", b_biguint, one_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_saturating_add_assign()
{
    println!("biguint_saturating_add_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U512::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint.saturating_add_assign(&one_biguint);
    println!("After a_biguint.saturating_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, U512::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut b_biguint = U512::max();
    println!("Originally, b_biguint = {}", b_biguint);
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    b_biguint.saturating_add_assign(&one_biguint);
    println!("After a_biguint.saturating_add_assign(&U512::one()), b_biguint = {}", b_biguint);
    assert_eq!(b_biguint, U512::max());
    assert_eq!(b_biguint.is_overflow(), false);
    assert_eq!(b_biguint.is_underflow(), false);
    assert_eq!(b_biguint.is_infinity(), false);
    assert_eq!(b_biguint.is_divided_by_zero(), false);
    assert_eq!(b_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_add()
{
    println!("biguint_modular_add");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    let one = U256::one();
    let res = a_biguint.modular_add(&one, &m);
    println!("{} + {} = {}", a_biguint, one, res);
    assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649007");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    let two = U256::from_uint(2_u8);
    let res = a_biguint.modular_add(&two, &m);
    println!("{} + {} = {}", a_biguint, two, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 3
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8); // == 76801874298166903427690031858186486050853753882811946569946433649008
    let three = U256::from_uint(3_u8);
    let res = a_biguint.modular_add(&three, &m);
    println!("{} + {} = {}", a_biguint, three, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == Self::max()
    let a_biguint = U256::max().wrapping_sub_uint(2_u8);
    let m = U256::max();
    let three = U256::from_uint(3_u8);
    let res = a_biguint.modular_add(&three, &m);
    println!("{} + {} = {}", a_biguint, three, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    //  op1 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let three = U256::from_uint(3_u8);
    let res = a_biguint.modular_add(&three, &m);
    println!("{} + {} = {}(mod {})", a_biguint, three, res, m);
    assert_eq!(res.to_string(), "3");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == multiple of modulo
    let a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    let three = U256::from_uint(3_u8);
    let res = a_biguint.modular_add(&three, &m);
    println!("{} + {} = {}(mod {})", a_biguint, three, res, m);
    assert_eq!(res.to_string(), "3");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op2 == 0
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    let res = a_biguint.modular_add(&zero, &m);
    println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op2 == multiple of modulo
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0 and op2 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    let res = a_biguint.modular_add(&zero, &m);
    println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == multiple of modulo and op2 == 0
    let a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    let res = a_biguint.modular_add(&zero, &m);
    println!("{} + {} = {}(mod {})", a_biguint, zero, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0 and op2 == multiple of modulo
    let a_biguint = U256::zero();
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let a_biguint = U256::from_uint(150_u8);
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    let res = a_biguint.modular_add(&multiple_of_modulo, &m);
    println!("{} + {} = {}(mod {})", a_biguint, multiple_of_modulo, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Panic Examples
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::zero();
    let _rhs = U256::from_uint(3_u8);
    // It will panic.
    // let res = _a_biguint.modular_add(&_rhs, &_m);
    
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::one();
    let _rhs = U256::from_uint(3_u8);
    // It will panic.
    // let res = _a_biguint.modular_add(&_rhs, &_m);
    println!("---------------------------");
}

fn biguint_modular_add_assign()
{
    println!("biguint_modular_add_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case 1
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    let one = U256::one();
    a_biguint.modular_add_assign(&one, &m);
    println!("After a_biguint.modular_add_assign_uint(&U256::one(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490061");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    println!("Originally, b_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    let two = U256::from_uint(2_u8);
    a_biguint.modular_add_assign(&two, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(2_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 3
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();    
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8); // == 768018742981669034276900318581864860508537538828119465699464336490062
    let three = U256::from_uint(3_u8);
    a_biguint.modular_add_assign(&three, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.modular_add_assign(&three, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "4");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == Self::max()
    let mut a_biguint = U256::max().wrapping_sub_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::max();
    let three = U256::from_uint(3_u8);
    a_biguint.modular_add_assign(&three, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::from_uint(250_u8);
    let three = U256::from_uint(3_u8);
    a_biguint.modular_add_assign(&three, &m);
    println!("After a_biguint.modular_add_assign(U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "3");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo
    let mut a_biguint = U256::from_uint(750_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::from_uint(250_u8);
    let three = U256::from_uint(3_u8);
    a_biguint.modular_add_assign(&three, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(3_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "3");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    a_biguint.modular_add_assign(&zero, &m);
    println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == multiple of modulo
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    a_biguint.modular_add_assign(&multiple_of_modulo, &m);
    println!("After a_biguint.modular_add_assign(& U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0 and op2 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    a_biguint.modular_add_assign(&zero, &m);
    println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == 0
    let mut a_biguint = U256::from_uint(750_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let zero = U256::zero();
    a_biguint.modular_add_assign(&zero, &m);
    println!("After a_biguint.modular_add_assign(&U256::zero(), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == multiple of modulo
    let mut a_biguint = U256::from_uint(150_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(50_u8);
    let multiple_of_modulo = U256::from_uint(250_u8);
    a_biguint.modular_add_assign(&multiple_of_modulo, &m);
    println!("After a_biguint.modular_add_assign(&U256::from_uint(250_u8), &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Panic Examples
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::zero();
    let _rhs = U256::one();
    // It will panic.
    // _a_biguint.modular_add_assign(&_rhs, &_m);
    
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::one();
    let _rhs = U256::one();
    // It will panic.
    // _a_biguint.modular_add_assign_uint(&_rhs, &_m);
    println!("---------------------------");
}

fn biguint_safe_add()
{
    println!("biguint_safe_add");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::max().safe_sub_uint(1_u8);
    let one_biguint = U512::one();
    let res = a_biguint.safe_add(&one_biguint);
    println!("{} + {} = {}", a_biguint, one_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let b_biguint = U512::max();
        let one_biguint = U512::one();
        let res = b_biguint.safe_add(&one_biguint);
        println!("{} + {} = {}", b_biguint, one_biguint, res);
        assert_eq!(res.to_string(), "0");
        assert_eq!(res.is_overflow(), true);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
    }

    let c_biguint = U512::zero();
    let one_biguint = U512::one();
    let res = c_biguint.safe_add(&one_biguint);
    println!("{} + {} = {}", c_biguint, one_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_add();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_add()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u128);

        let _b_biguint = U512::max();
        let _one_biguint = U512::one();
        let _res = _b_biguint.safe_add(&_one_biguint);
    }
}

fn biguint_safe_add_assign()
{
    println!("biguint_safe_add_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U512::max().safe_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one_biguint = U512::one();
    a_biguint.safe_add_assign(&one_biguint);
    println!("After a_biguint.safe_add_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, U512::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let mut a_biguint = U512::max();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let one_biguint = U512::one();
        a_biguint.safe_add_assign(&one_biguint);
        println!("After a_biguint.safe_add_assign(&U512::one()), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "0");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_add_assign(&one_biguint);
        println!("After a_biguint.safe_add_assign(&U512::one()),\ta_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "1");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_add_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_add_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u8);
    
        let mut _a_biguint = U512::max();
        let _one_biguint = U512::one();
        _a_biguint.safe_add_assign(&_one_biguint);
        _a_biguint.safe_add_assign(&_one_biguint);
    }
}

fn biguint_sub()
{
    biguint_borrowing_sub();
    biguint_borrowing_sub_assign();
    biguint_wrapping_sub();
    biguint_wrapping_sub_assign();
    biguint_overflowing_sub();
    biguint_overflowing_sub_assign();
    biguint_checked_sub();
    biguint_unchecked_sub();
    biguint_saturating_sub();
    biguint_saturating_sub_assign();
    biguint_modular_sub();
    biguint_modular_sub_assign();
    biguint_abs_diff();
    biguint_safe_sub();
    biguint_safe_sub_assign();
}

fn biguint_borrowing_sub()
{
    println!("biguint_borrowing_sub");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    let a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();

    let (c_biguint_lo, borrow) = a_biguint_lo.borrowing_sub(&b_biguint_lo, false);
    let (c_biguint_hi, unerflow) = a_biguint_hi.borrowing_sub(&b_biguint_hi, borrow);

    println!("{}:{} - {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("borrow = {}, overflow = {}", borrow, unerflow);
    assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    assert_eq!(borrow, true);
    assert_eq!(c_biguint_lo.is_underflow(), true);
    assert_eq!(c_biguint_lo.is_overflow(), false);
    assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(c_biguint_lo.is_infinity(), false);
    assert_eq!(c_biguint_lo.is_undefined(), false);
    assert_eq!(c_biguint_lo.is_left_carry(), false);
    assert_eq!(c_biguint_lo.is_right_carry(), false);

    assert_eq!(unerflow, false);
    assert_eq!(c_biguint_hi.is_underflow(), false);
    assert_eq!(c_biguint_hi.is_overflow(), false);
    assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(c_biguint_hi.is_infinity(), false);
    assert_eq!(c_biguint_hi.is_undefined(), false);
    assert_eq!(c_biguint_hi.is_left_carry(), false);
    assert_eq!(c_biguint_hi.is_right_carry(), false);

    let a_biguint_hi = U256::zero();
    let a_biguint_lo = U256::zero();
    let b_biguint_hi = U256::zero();
    let b_biguint_lo = U256::one();

    let (c_biguint_lo, borrow) = a_biguint_lo.borrowing_sub(&b_biguint_lo, false);
    let (c_biguint_hi, underflow) = a_biguint_hi.borrowing_sub(&b_biguint_hi, borrow);

    println!("{}:{} - {}:{} = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("borrow = {}, underflow = {}", borrow, underflow);

    assert_eq!(c_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(c_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(borrow, true);
    assert_eq!(c_biguint_lo.is_underflow(), true);
    assert_eq!(c_biguint_lo.is_overflow(), false);
    assert_eq!(c_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(c_biguint_lo.is_infinity(), false);
    assert_eq!(c_biguint_lo.is_undefined(), false);
    assert_eq!(c_biguint_lo.is_left_carry(), false);
    assert_eq!(c_biguint_lo.is_right_carry(), false);

    assert_eq!(underflow, true);
    assert_eq!(c_biguint_hi.is_underflow(), true);
    assert_eq!(c_biguint_hi.is_overflow(), false);
    assert_eq!(c_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(c_biguint_hi.is_infinity(), false);
    assert_eq!(c_biguint_hi.is_undefined(), false);
    assert_eq!(c_biguint_hi.is_left_carry(), false);
    assert_eq!(c_biguint_hi.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_borrowing_sub_assign()
{
    println!("biguint_borrowing_sub_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint_hi = U256::from_str_radix("FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFF2", 16).unwrap();
    let mut a_biguint_lo = U256::from_str_radix("1111_1110_1111_1111_1111_1101_1111_1111_1111_1110_1111_1111_1111_1101_1111_1110", 16).unwrap();
    let b_biguint_hi = U256::from_str_radix("EDCB_A987_6543_210F_0123_4567_89AB_CDEF_EDCB_A987_6543_210F_0123_4567_89AB_CDE1", 16).unwrap();
    let b_biguint_lo = U256::from_str_radix("FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF_FDB9_7531_0ECA_8642_2468_ACE0_1357_9BDF", 16).unwrap();

    print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("borrow = {}, underflow = {}", borrow, underflow);

    assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210_1234_5678_9ABC_DEF0_FEDC_BA98_7654_3210");
    assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531_1357_9BDF_0246_8ACE_ECA8_6420_FDB9_7531");
    assert_eq!(borrow, true);
    assert_eq!(a_biguint_lo.is_underflow(), true);
    assert_eq!(a_biguint_lo.is_overflow(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(underflow, false);
    assert_eq!(a_biguint_hi.is_underflow(), false);
    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    let mut a_biguint_hi = U256::zero();
    let mut a_biguint_lo = U256::zero();
    let b_biguint_hi = U256::zero();
    let b_biguint_lo = U256::one();

    print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("borrow = {}, underflow = {}", borrow, underflow);

    assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(borrow, true);
    assert_eq!(a_biguint_lo.is_underflow(), true);
    assert_eq!(a_biguint_lo.is_overflow(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(underflow, true);
    assert_eq!(a_biguint_hi.is_underflow(), true);
    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);

    print!("{}:{} - {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), b_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    let borrow = a_biguint_lo.borrowing_sub_assign(&b_biguint_lo, false);
    let underflow = a_biguint_hi.borrowing_sub_assign(&b_biguint_hi, borrow);
    println!(" = {}:{}", a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap());
    println!("borrow = {}, underflow = {}", borrow, underflow);

    assert_eq!(a_biguint_hi.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF");
    assert_eq!(a_biguint_lo.to_string_with_radix_and_stride(16, 4).unwrap(), "FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFE");
    assert_eq!(borrow, false);
    assert_eq!(a_biguint_lo.is_underflow(), true);
    assert_eq!(a_biguint_lo.is_overflow(), false);
    assert_eq!(a_biguint_lo.is_divided_by_zero(), false);
    assert_eq!(a_biguint_lo.is_infinity(), false);
    assert_eq!(a_biguint_lo.is_undefined(), false);
    assert_eq!(a_biguint_lo.is_left_carry(), false);
    assert_eq!(a_biguint_lo.is_right_carry(), false);

    assert_eq!(underflow, false);
    assert_eq!(a_biguint_hi.is_underflow(), true);
    assert_eq!(a_biguint_hi.is_overflow(), false);
    assert_eq!(a_biguint_hi.is_divided_by_zero(), false);
    assert_eq!(a_biguint_hi.is_infinity(), false);
    assert_eq!(a_biguint_hi.is_undefined(), false);
    assert_eq!(a_biguint_hi.is_left_carry(), false);
    assert_eq!(a_biguint_hi.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_sub()
{
    println!("biguint_wrapping_sub");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::one();
    let res = a_biguint.wrapping_sub(&U512::one());
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res, U512::zero());
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::zero();
    let res = b_biguint.wrapping_sub(&U512::one());
    println!("{} - 1 = {}", b_biguint, res);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let c_biguint = U512::max();
    let res = c_biguint.wrapping_sub(&U512::one());
    println!("{} - 1 = {}", c_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_sub_assign()
{
    println!("biguint_wrapping_sub_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign(&U512::one());
    println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign(&U512::one());
    println!("After a_biguint.wrapping_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign(&U512::one());
    println!("After a_biguint.wrapping_sub_assign(&U512::one()),\ta_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_sub()
{
    println!("biguint_overflowing_sub");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::one();
    let (res, underflow) = a_biguint.overflowing_sub(&U512::one());
    println!("{} - 1 = {}, underflow = {}", a_biguint, res, underflow);
    assert_eq!(underflow, false);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::zero();
    let (res, underflow) = b_biguint.overflowing_sub(&U512::one());
    println!("{} - 1 = {}, underflow = {}", b_biguint, res, underflow);
    assert_eq!(underflow, true);
    assert_eq!(res, U512::max());
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_sub_assign()
{
    println!("biguint_overflowing_sub_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = a_biguint.overflowing_sub_assign(&U512::one());
    println!("After a_biguint.overflowing_sub_assign(&U512::one()), a_biguint = {}, underflow = {}", a_biguint, underflow);
    assert_eq!(underflow, false);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut b_biguint = U512::zero();
    println!("Originally, b_biguint = {}", b_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = b_biguint.overflowing_sub_assign(&U512::one());
    println!("After b_biguint.overflowing_sub_assign(&U512::one()),\tb_biguint = {}, underflow = {}", b_biguint, underflow);
    assert_eq!(underflow, true);
    assert_eq!(b_biguint, U512::max());
    assert_eq!(b_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_sub()
{
    println!("biguint_checked_sub");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::one();
    let res = a_biguint.checked_sub(&U512::one());
    match res
    {
        Some(r) => {
                println!("{} - 1 = {}, underflow = {}", a_biguint, r, r.is_underflow());
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error: Underflow"); },
    }

    let b_biguint = U512::max();
    let res = b_biguint.checked_add(&U512::one());
    match res
    {
        Some(r) => { println!("{} - 1 = {}, underflow = {}", b_biguint, r, r.is_underflow()); },
        None => { 
                println!("Error: Underflow");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_unchecked_sub()
{
    println!("biguint_unchecked_sub");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::one();
    let res = a_biguint.unchecked_sub(&U512::one());
    println!("{} - 1 = {}, underflow = {}", a_biguint, res, res.is_underflow());
    assert_eq!(res, U512::zero());
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _b_biguint = U512::zero();
    // It will panic.
    // let res = _b_biguint.unchecked_sub(&U512::one());
    println!("---------------------------");
}

fn biguint_saturating_sub()
{
    println!("biguint_saturating_sub");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U512::one();
    let one = U512::one();
    let res = a_biguint.saturating_sub(&one);
    println!("{} - {} = {}", a_biguint, one, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U512::zero();
    let one = U512::one();
    let res = b_biguint.saturating_sub(&one);
    println!("{} - {} = {}", b_biguint, one, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let c_biguint = U512::from_uint(5_u8);
    let ten = U512::from_uint(10_u8);
    let res = c_biguint.saturating_sub(&ten);
    println!("{} - {} = {}", c_biguint, ten, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_saturating_sub_assign()
{
    println!("biguint_saturating_sub_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one = U512::one();
    a_biguint.saturating_sub_assign(&one);
    println!("After a_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one = U512::one();
    a_biguint.saturating_sub_assign(&one);
    println!("After a_biguint.saturating_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U512::from_uint(5_u8);
    println!("Originally, b_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let ten = U512::from_uint(10_u8);
    a_biguint.saturating_sub_assign(&ten);
    println!("After b_biguint.saturating_sub_assign(&U512::from_uint(10_u8)), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_sub()
{
    println!("biguint_modular_sub");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal Case 1
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let one = U256::one();
    let res = a_biguint.modular_sub(&one, &m);
    println!("{} - {} = {} (mod {})", a_biguint, one, res, m);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal Case 2
    let a_biguint = U256::one();
    let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let one = U256::one();
    let res = a_biguint.modular_sub(&one, &m);
    println!("{} - {} = {} (mod {})", a_biguint, one, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal Case 3
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("10000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let three = U256::from_uint(4_u8);
    let res = a_biguint.modular_sub(&three, &m);
    println!("{} - {} = {} (mod {})", a_biguint, one, res, m);
    assert_eq!(res.to_string(), "9999999999999999999999999999999999999999999999999999999999999999998");
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == Self::max()
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::max();
    let rhs = U256::from_uint(3_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res, U256::max().wrapping_sub_uint(1_u8));
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let rhs = U256::from_uint(3_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "247");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == multiple of modulo
    let a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    let rhs = U256::from_uint(3_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "247");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op2 == 0
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op2 == multiple of modulo
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(50_u8);
    let rhs = U256::from_uint(250_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "6");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0 and op2 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == 0
    let a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0 and op2 == multiple of modulo
    let a_biguint = U256::zero();
    let m = U256::from_uint(50_u8);
    let rhs = U256::from_uint(250_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let a_biguint = U256::from_uint(150_u8);
    let m = U256::from_uint(50_u8);
    let rhs = U256::from_uint(250_u8);
    let res = a_biguint.modular_sub(&rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // Pacnic Examples
    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::zero();
    let _rhs = U256::from_uint(3_u8);
    // It will panic.
    // let res = _a_biguint.modular_sub(&_rhs, &_m);
    
    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::one();
    let _rhs = U256::from_uint(3_u8);
    // It will panic.
    // let res = _a_biguint.modular_sub(&_rhs, &_m);
    println!("---------------------------");
}

fn biguint_modular_sub_assign()
{
    println!("biguint_modular_sub_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case 1
    let mut a_biguint = UU32::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = UU32::one();
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Normal case 2
    let mut a_biguint = UU32::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
       
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = UU32::from_uint(2_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 3
    let mut a_biguint = UU32::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
       
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = UU32::from_uint(3_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084090");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // modulo == Self::max()
    let mut a_biguint = U256::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::max();
    let rhs = U256::from_uint(3_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint, U256::max().wrapping_sub_uint(1_u8));
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let rhs = U256::from_uint(3_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "247");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == multiple of modulo
    let mut a_biguint = U256::from_uint(750_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let rhs = U256::from_uint(3_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "247");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == multiple of modulo
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(50_u8);
    let rhs = U256::from_uint(250_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "6");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == 0 and op2 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == 0
    let mut a_biguint = U256::from_uint(750_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(250_u8);
    let rhs = U256::zero();
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == multiple of modulo
    let mut a_biguint = U256::from_uint(150_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let m = U256::from_uint(50_u8);
    let rhs = U256::from_uint(250_u8);
    a_biguint.modular_sub_assign(&rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Panic Examples
    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::zero();
    let _rhs = U256::one();
    // It will panic.
    // _a_biguint.modular_sub_assign(&_rhs, &m);
    
    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::one();
    let _rhs = U256::one();
    // It will panic.
    // _a_biguint.modular_sub_assign(&_rhs, &m);
    println!("---------------------------");
}

fn biguint_abs_diff()
{
    println!("biguint_abs_diff");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let a_biguint = U256::from_str("500000000000000000500000000500000000500000000500000000").unwrap();
    let b_biguint = U256::from_str("500000000000000000000000000000000000000000000000000000").unwrap();
    let c_biguint = a_biguint.abs_diff(&b_biguint);
    let d_biguint = b_biguint.abs_diff(&a_biguint);
    println!("500000000000000000500000000500000000500000000500000000 <-> 500000000000000000000000000000000000000000000000000000 = {}", c_biguint);
    println!("500000000000000000000000000000000000000000000000000000 <-> 500000000000000000500000000500000000500000000500000000 = {}", d_biguint);
    assert_eq!(c_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    assert_eq!(d_biguint, U256::from_str("500000000500000000500000000500000000").unwrap());
    assert_eq!(d_biguint.is_overflow(), false);
    assert_eq!(d_biguint.is_underflow(), false);
    assert_eq!(d_biguint.is_divided_by_zero(), false);
    assert_eq!(d_biguint.is_infinity(), false);
    assert_eq!(d_biguint.is_undefined(), false);
    assert_eq!(d_biguint.is_left_carry(), false);
    assert_eq!(d_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_safe_sub()
{
    println!("biguint_safe_sub");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::one();
    let one = U512::one();
    let res = a_biguint.safe_sub(&one);
    println!("{} - {} = {}", a_biguint, one, res);
    assert_eq!(res, U512::zero());
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let b_biguint = U512::zero();
        let one = U512::one();
        let res = b_biguint.safe_sub(&one);
        println!("{} - {} = {}", b_biguint, one, res);
        assert_eq!(res, U512::max());
        assert_eq!(res.is_underflow(), true);
        assert_eq!(res.is_overflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
    }

    let c_biguint = U512::max();
    let one = U512::one();
    let res = c_biguint.safe_sub(&one);
    println!("{} - {} = {}", c_biguint, one, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_sub();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_sub()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u128);
    
        let _b_biguint = U512::zero();
        let _one = U512::one();
        let _res = _b_biguint.safe_sub(&_one);
    }
}

fn biguint_safe_sub_assign()
{
    println!("biguint_safe_sub_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = U512::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one = U512::one();
    a_biguint.safe_sub_assign(&one);
    println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let mut a_biguint = U512::zero();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let one = U512::one();
        a_biguint.safe_sub_assign(&one);
        println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        assert_eq!(a_biguint.is_underflow(), true);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }

    let mut a_biguint = U512::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let one = U512::one();
    a_biguint.safe_sub_assign(&one);
    println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_sub_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_sub_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u8);

        let mut a_biguint = U512::zero();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let one = U512::one();
        // It will panic.
        a_biguint.safe_sub_assign(&one);
        println!("After a_biguint.safe_sub_assign(&U512::one()), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        assert_eq!(a_biguint.is_underflow(), true);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }
}

fn biguint_mul()
{
    biguint_carrying_mul();
    biguint_carrying_mul_assign();
    biguint_widening_mul();
    biguint_widening_mul_assign();
    biguint_wrapping_mul();
    biguint_wrapping_mul_assign();
    biguint_overflowing_mul();
    biguint_overflowing_mul_assign();
    biguint_checked_mul();
    biguint_unchecked_mul();
    biguint_saturating_mul();
    biguint_saturating_mul_assign();
    biguint_modular_mul();
    biguint_modular_mul_assign();
    biguint_safe_mul();
    biguint_safe_mul_assign();
}

fn biguint_carrying_mul()
{
    println!("biguint_carrying_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint_low = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint_high = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_biguint = UU32::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    let (res_biguint_low, res_biguint_high) = a_biguint_low.carrying_mul(&b_biguint, UU32::zero());
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);
    let (res_biguint_high, res_biguint_higher) = a_biguint_high.carrying_mul(&b_biguint, res_biguint_high);

    println!("{}:{} X {} = {}:{}:{}", a_biguint_high, a_biguint_low, b_biguint, res_biguint_higher, res_biguint_high, res_biguint_low);
    assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    assert_eq!(res_biguint_higher.is_overflow(), false);
    assert_eq!(res_biguint_higher.is_underflow(), false);
    assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    assert_eq!(res_biguint_higher.is_infinity(), false);
    assert_eq!(res_biguint_higher.is_undefined(), false);
    assert_eq!(res_biguint_higher.is_left_carry(), false);
    assert_eq!(res_biguint_higher.is_right_carry(), false);

    assert_eq!(res_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    assert_eq!(res_biguint_high.is_overflow(), true);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    assert_eq!(res_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    assert_eq!(res_biguint_low.is_overflow(), true);
    assert_eq!(res_biguint_low.is_underflow(), false);
    assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    assert_eq!(res_biguint_low.is_infinity(), false);
    assert_eq!(res_biguint_low.is_undefined(), false);
    assert_eq!(res_biguint_low.is_left_carry(), false);
    assert_eq!(res_biguint_low.is_right_carry(), false);

    // Maximum case
    let a_biguint_low = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    let a_biguint_high = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    let b_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    let (res_biguint_low, res_biguint_high) = a_biguint_low.carrying_mul(&b_biguint, UU32::zero());
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);
    let (res_biguint_high, res_biguint_higher) = a_biguint_high.carrying_mul(&b_biguint, res_biguint_high);

    println!("{}:{} X {} = {}:{}:{}", a_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), a_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    assert_eq!(res_biguint_higher.is_overflow(), false);
    assert_eq!(res_biguint_higher.is_underflow(), false);
    assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    assert_eq!(res_biguint_higher.is_infinity(), false);
    assert_eq!(res_biguint_higher.is_undefined(), false);
    assert_eq!(res_biguint_higher.is_left_carry(), false);
    assert_eq!(res_biguint_higher.is_right_carry(), false);

    assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    assert_eq!(res_biguint_high.is_overflow(), true);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    assert_eq!(res_biguint_low.to_string(), "1");
    assert_eq!(res_biguint_low.is_overflow(), true);
    assert_eq!(res_biguint_low.is_underflow(), false);
    assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    assert_eq!(res_biguint_low.is_infinity(), false);
    assert_eq!(res_biguint_low.is_undefined(), false);
    assert_eq!(res_biguint_low.is_left_carry(), false);
    assert_eq!(res_biguint_low.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_carrying_mul_assign()
{
    println!("biguint_carrying_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case
    let mut a_biguint_low = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint_high = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_biguint = U256::from_string("16962363268797823794757102636892132280421717087553271230257168091959361441925").unwrap();
    println!("Originally, a_biguint_low = {}\na_biguint_high = {}", a_biguint_low, a_biguint_high);
    assert_eq!(a_biguint_low.is_overflow(), false);
    assert_eq!(a_biguint_low.is_underflow(), false);
    assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    assert_eq!(a_biguint_low.is_infinity(), false);
    assert_eq!(a_biguint_low.is_undefined(), false);
    assert_eq!(a_biguint_low.is_left_carry(), false);
    assert_eq!(a_biguint_low.is_right_carry(), false);

    assert_eq!(a_biguint_high.is_overflow(), false);
    assert_eq!(a_biguint_high.is_underflow(), false);
    assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    assert_eq!(a_biguint_high.is_infinity(), false);
    assert_eq!(a_biguint_high.is_undefined(), false);
    assert_eq!(a_biguint_high.is_left_carry(), false);
    assert_eq!(a_biguint_high.is_right_carry(), false);
    
    let res_biguint_high = a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero());
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    let res_biguint_higher = a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high);
    println!("After a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero()),\na_biguint_low = {}", a_biguint_low);
    println!("After a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high),\na_biguint_high = {}", a_biguint_high);
    println!("res_biguint_higher = {}", res_biguint_higher);
    assert_eq!(res_biguint_higher.to_string(), "11043616366686523019040587905143508095308019572635527295298701528708842829");
    assert_eq!(res_biguint_higher.is_overflow(), false);
    assert_eq!(res_biguint_higher.is_underflow(), false);
    assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    assert_eq!(res_biguint_higher.is_infinity(), false);
    assert_eq!(res_biguint_higher.is_undefined(), false);
    assert_eq!(res_biguint_higher.is_left_carry(), false);
    assert_eq!(res_biguint_higher.is_right_carry(), false);

    assert_eq!(a_biguint_high.to_string(), "47612192950075281462365720785702517256274202447286280420710978194126658529299");
    assert_eq!(a_biguint_high.is_overflow(), true);
    assert_eq!(a_biguint_high.is_underflow(), false);
    assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    assert_eq!(a_biguint_high.is_infinity(), false);
    assert_eq!(a_biguint_high.is_undefined(), false);
    assert_eq!(a_biguint_high.is_left_carry(), false);
    assert_eq!(a_biguint_high.is_right_carry(), false);

    assert_eq!(a_biguint_low.to_string(), "99569105317044689054574557712853522297141576321520100863242044268764373638902");
    assert_eq!(a_biguint_low.is_overflow(), true);
    assert_eq!(a_biguint_low.is_underflow(), false);
    assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    assert_eq!(a_biguint_low.is_infinity(), false);
    assert_eq!(a_biguint_low.is_undefined(), false);
    assert_eq!(a_biguint_low.is_left_carry(), false);
    assert_eq!(a_biguint_low.is_right_carry(), false);

    // Maximum case
    let mut a_biguint_low = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    let mut a_biguint_high = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    let b_biguint = U256::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    println!("Originally, a_biguint_low = {}\na_biguint_high = {}", a_biguint_low, a_biguint_high);
    assert_eq!(a_biguint_low.is_overflow(), false);
    assert_eq!(a_biguint_low.is_underflow(), false);
    assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    assert_eq!(a_biguint_low.is_infinity(), false);
    assert_eq!(a_biguint_low.is_undefined(), false);
    assert_eq!(a_biguint_low.is_left_carry(), false);
    assert_eq!(a_biguint_low.is_right_carry(), false);

    assert_eq!(a_biguint_high.is_overflow(), false);
    assert_eq!(a_biguint_high.is_underflow(), false);
    assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    assert_eq!(a_biguint_high.is_infinity(), false);
    assert_eq!(a_biguint_high.is_undefined(), false);
    assert_eq!(a_biguint_high.is_left_carry(), false);
    assert_eq!(a_biguint_high.is_right_carry(), false);
    
    let res_biguint_high = a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero());
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    let res_biguint_higher = a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high);
    println!("After a_biguint_low.carrying_mul_assign(&b_biguint, UU32::zero()),\na_biguint_low = {}", a_biguint_low);
    println!("After a_biguint_high.carrying_mul_assign(&b_biguint, res_biguint_high),\na_biguint_high = {}", a_biguint_high);
    println!("res_biguint_higher = {}", res_biguint_higher);
    assert_eq!(res_biguint_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    assert_eq!(res_biguint_higher.is_overflow(), false);
    assert_eq!(res_biguint_higher.is_underflow(), false);
    assert_eq!(res_biguint_higher.is_divided_by_zero(), false);
    assert_eq!(res_biguint_higher.is_infinity(), false);
    assert_eq!(res_biguint_higher.is_undefined(), false);
    assert_eq!(res_biguint_higher.is_left_carry(), false);
    assert_eq!(res_biguint_higher.is_right_carry(), false);

    assert_eq!(a_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF");
    assert_eq!(a_biguint_high.is_overflow(), true);
    assert_eq!(a_biguint_high.is_underflow(), false);
    assert_eq!(a_biguint_high.is_divided_by_zero(), false);
    assert_eq!(a_biguint_high.is_infinity(), false);
    assert_eq!(a_biguint_high.is_undefined(), false);
    assert_eq!(a_biguint_high.is_left_carry(), false);
    assert_eq!(a_biguint_high.is_right_carry(), false);

    assert_eq!(a_biguint_low.to_string(), "1");
    assert_eq!(a_biguint_low.is_overflow(), true);
    assert_eq!(a_biguint_low.is_underflow(), false);
    assert_eq!(a_biguint_low.is_divided_by_zero(), false);
    assert_eq!(a_biguint_low.is_infinity(), false);
    assert_eq!(a_biguint_low.is_undefined(), false);
    assert_eq!(a_biguint_low.is_left_carry(), false);
    assert_eq!(a_biguint_low.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_widening_mul()
{
    println!("biguint_widening_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case
    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    let (res_biguint_low, res_biguint_high) = a_biguint.widening_mul(&b_biguint);

    println!("{} X {} = {}:{}", a_biguint, b_biguint, res_biguint_high, res_biguint_low);
    assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    assert_eq!(res_biguint_low.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    assert_eq!(res_biguint_low.is_overflow(), true);
    assert_eq!(res_biguint_low.is_underflow(), false);
    assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    assert_eq!(res_biguint_low.is_infinity(), false);
    assert_eq!(res_biguint_low.is_undefined(), false);
    assert_eq!(res_biguint_low.is_left_carry(), false);
    assert_eq!(res_biguint_low.is_right_carry(), false);

    // Maximum case
    let a_biguint = U256::max();
    let b_biguint = U256::max();
    let (res_biguint_low, res_biguint_high) = a_biguint.widening_mul(&b_biguint);

    println!("{} X {} = {}:{}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    assert_eq!(res_biguint_low.to_string_with_radix_and_stride(16, 8).unwrap(), "1");
    assert_eq!(res_biguint_low.is_overflow(), true);
    assert_eq!(res_biguint_low.is_underflow(), false);
    assert_eq!(res_biguint_low.is_divided_by_zero(), false);
    assert_eq!(res_biguint_low.is_infinity(), false);
    assert_eq!(res_biguint_low.is_undefined(), false);
    assert_eq!(res_biguint_low.is_left_carry(), false);
    assert_eq!(res_biguint_low.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_widening_mul_assign()
{
    println!("biguint_widening_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case
    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_string("123456789098765432101234566789098765432101234567890987654321012345678909876").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let res_biguint_high = a_biguint.widening_mul_assign(&b_biguint);
    println!("After a_biguint.widening_mul_assign(&b_biguint),\na_biguint = {}\nres_biguint_high = {}", a_biguint, res_biguint_high);
    assert_eq!(a_biguint.to_string(), "99383456710232708163688724311017197312314189592099594761784803361525674171544");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    assert_eq!(res_biguint_high.to_string(), "934840581853378776614741519244947987886551255599166686673415072970125925");
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);

    // Maximum case
    let mut a_biguint = U256::max();
    let b_biguint = U256::max();
    println!("Originally, a_biguint = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let res_biguint_high = a_biguint.widening_mul_assign(&b_biguint);
    println!("After a_biguint.widening_mul_assign(&b_biguint),\na_biguint = {}\nres_biguint_high = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    assert_eq!(res_biguint_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE");
    assert_eq!(res_biguint_high.is_overflow(), false);
    assert_eq!(res_biguint_high.is_underflow(), false);
    assert_eq!(res_biguint_high.is_divided_by_zero(), false);
    assert_eq!(res_biguint_high.is_infinity(), false);
    assert_eq!(res_biguint_high.is_undefined(), false);
    assert_eq!(res_biguint_high.is_left_carry(), false);
    assert_eq!(res_biguint_high.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_mul()
{
    println!("biguint_wrapping_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.wrapping_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.wrapping_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_mul_assign()
{
    println!("biguint_wrapping_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_mul_assign(&b_biguint);
    println!("After a_biguint.wrapping_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_mul_assign(&b_biguint);
    println!("After c_biguint.wrapping_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_mul()
{
    println!("biguint_overflowing_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let (res, overflow) = a_biguint.overflowing_mul(&b_biguint);
    println!("{} X {} = {}, {}", a_biguint, b_biguint, res, overflow);
    assert_eq!(overflow, false);
    assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let (res, overflow) = a_biguint.overflowing_mul(&b_biguint);
    println!("{} X {} = {}, {}", a_biguint, b_biguint, res, overflow);
    assert_eq!(overflow, true);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_mul_assign()
{
    println!("biguint_overflowing_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);

    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_mul_assign(&b_biguint);
    println!("After a_biguint.overflowing_mul_assign(&b_biguint), a_biguint = {}, {}", a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_mul_assign(&b_biguint);
    println!("After c_biguint.overflowing_mul_assign(&b_biguint), a_biguint = {}, {}", a_biguint, overflow);
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_mul()
{
    println!("biguint_checked_mul");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.checked_mul(&b_biguint);
    match &res
    {
        Some(r) =>
            {
                println!("{} X {} = {}", a_biguint, b_biguint, r);
                assert_eq!(r.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Overflow happend!"); },
    }

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.checked_mul(&b_biguint);
    match &res
    {
        Some(r) => { println!("{} X {} = {}", a_biguint, b_biguint, r); },
        None =>
            {
                println!("Overflow happend!");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_unchecked_mul()
{
    println!("biguint_unchecked_mul");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = UU32::from_uint(248_u8);
    let res = a_biguint.unchecked_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let _b_biguint = UU32::from_uint(248_u8);
    // It will panic.
    // let res = a_biguint.unchecked_mul(&_b_biguint);
    println!("---------------------------");
}

fn biguint_saturating_mul()
{
    println!("biguint_saturating_mul");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = UU32::from_uint(5_u8);
    let res = a_biguint.saturating_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = UU32::from_uint(248_u8);
    let res = a_biguint.saturating_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res, UU32::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_saturating_mul_assign()
{
    println!("biguint_saturating_mul_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = UU32::from_uint(5_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_mul_assign(&b_biguint);
    println!("After a_biguint.saturating_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_biguint = UU32::from_uint(248_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    a_biguint.saturating_mul_assign(&b_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    println!("After a_biguint.saturating_mul_assign_uint(&b_biguint), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU32::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_mul()
{
    println!("biguint_modular_mul");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mul_biguint = UU32::from_uint(5_u8);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mul_biguint = UU32::from_uint(123456789_u32);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "8622247606403727534023749230384750061554739874487486410968923457264899031");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == maximum
    let m = UU32::max();
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mul_biguint = UU32::from_uint(123456789_u32);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "111970462099597246185125739952117562742423650866418469977837510261574559319010");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::zero();
    let mul_biguint = UU32::from_uint(5_u8);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_uint(4321000_u32);
    let mul_biguint = UU32::from_uint(5_u8);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op2 == 0
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_biguint = UU32::zero();
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op2 == multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let mul_biguint = UU32::from_uint(4321000_u32);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == 0 and op2 == 0
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::zero();
    let mul_biguint = UU32::zero();
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == 0 and op2 == multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::zero();
    let mul_biguint = UU32::from_uint(4321000_u32);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == 0
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_uint(4321000_u32);
    let mul_biguint = UU32::zero();
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_uint(4321000_u32);
    let mul_biguint = UU32::from_uint(4321000_u32);
    let res = a_biguint.modular_mul(&mul_biguint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_biguint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    
    // Panic Examples
    let _m = UU32::zero();
    let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _mul_biguint = UU32::from_uint(248_u8);
    // It will panic!
    // let res = _a_biguint.modular_mul(&_mul_biguint, &_m);
    
    let _m = UU32::one();
    let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _mul_biguint = UU32::from_uint(248_u8);
    // It will panic!
    // let res = _a_biguint.modular_mul(&_mul_biguint, &_m);
    println!("---------------------------");
}

fn biguint_modular_mul_assign()
{
    println!("biguint_modular_mul_assign");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_biguint = UU32::from_uint(5_u8);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After a_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_biguint = UU32::from_uint(123456789_u32);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "8622247606403727534023749230384750061554739874487486410968923457264899031");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == maximum
    let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::max();
    let mul_biguint = UU32::from_uint(123456789_u32);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "111970462099597246185125739952117562742423650866418469977837510261574559319010");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::from_uint(5_u8);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo
    let mut a_biguint = U256::from_uint(4321000_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::from_uint(5_u8);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::zero();
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op2 == multiple of modulo
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::from_uint(4321000_u32);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == 0 and op2 == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::zero();
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == 0 and op2 == multiple of modulo
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::from_uint(4321000_u32);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == 0
    let mut a_biguint = U256::from_uint(4321000_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::zero();
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // op1 == multiple of modulo and op2 == multiple of modulo
    let mut a_biguint = U256::from_uint(4321000_u32);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = UU32::from_uint(1000_u16);
    let mul_biguint = UU32::from_uint(4321000_u32);
    a_biguint.modular_mul_assign(&mul_biguint, &m);
    println!("After b_biguint.modular_mul_assign(&mul_biguint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    // Panic Examples
    let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    assert_eq!(_a_biguint.is_overflow(), false);
    assert_eq!(_a_biguint.is_underflow(), false);
    assert_eq!(_a_biguint.is_divided_by_zero(), false);
    assert_eq!(_a_biguint.is_infinity(), false);
    assert_eq!(_a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _m = UU32::zero();
    let _mul_biguint = UU32::from_uint(248_u8);
    // It will panic!
    // _a_biguint.modular_mul_assign(&_mul_biguint, &_m);
    
    let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    assert_eq!(_a_biguint.is_overflow(), false);
    assert_eq!(_a_biguint.is_underflow(), false);
    assert_eq!(_a_biguint.is_divided_by_zero(), false);
    assert_eq!(_a_biguint.is_infinity(), false);
    assert_eq!(_a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _m = UU32::one();
    let _mul_biguint = UU32::from_uint(248_u8);
    // It will panic!
    // _a_biguint.modular_mul_assign(&_mul_biguint, &_m);
    println!("---------------------------");
}

fn biguint_safe_mul()
{
    println!("biguint_safe_mul");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_biguint = U256::from_uint(248_u8);
    let res = a_biguint.safe_mul(&b_biguint);
    println!("{} X {} = {}", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let b_biguint = U256::from_uint(248_u8);
        let res = a_biguint.safe_mul(&b_biguint);
        println!("{} X {} = {}", a_biguint, b_biguint, res);
        assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
        assert_eq!(res.is_overflow(), true);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_mul();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_mul()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u16);
        
        let _a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_biguint = U256::from_uint(248_u8);
        let _res = _a_biguint.safe_mul(&_b_biguint);
    }
}

fn biguint_safe_mul_assign()
{
    println!("biguint_safe_mul_assign");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_uint(248_u8);
    a_biguint.safe_mul_assign(&b_biguint);
    println!("After a_biguint.safe_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let b_biguint = U256::from_uint(248_u8);
        a_biguint.safe_mul_assign(&b_biguint);
        println!("After c_biguint.safe_mul_assign(&b_biguint), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_mul_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_mul_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u32);

        let mut _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_biguint = U256::from_uint(248_u8);
        _a_biguint.safe_mul_assign(&_b_biguint);
    }
}


fn biguint_div()
{
    biguint_divide_fully();
    biguint_wrapping_div();
    biguint_wrapping_div_assign();
    biguint_overflowing_div();
    biguint_overflowing_div_assign();
    biguint_checked_div();
    biguint_unchecked_div();
    biguint_saturating_div();
    biguint_saturating_div_assign();
    biguint_modular_div();
    biguint_modular_div_assign();
}

fn biguint_divide_fully()
{
    println!("biguint_divide_fully");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let (quotient, remainder) = dividend.divide_fully(&divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let (quotient, remainder) = dividend.divide_fully(&divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    // It will panic!
    // let (quotient, remainder) = dividend.divide_fully(&_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    // It will panic!
    // let (quotient, remainder) = dividend.divide_fully(&_divisor);
    println!("---------------------------");
}

fn biguint_wrapping_div()
{
    println!("biguint_wrapping_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.wrapping_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let dividend = U256::zero();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.wrapping_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    // It will panic!
    // let quotient = _dividend.wrapping_div(&_divisor);

    let _dividend = U256::zero();
    let _divisor = U256::zero();
    // It will panic!
    // let quotient = _dividend.wrapping_div(&_divisor);
    println!("---------------------------");
}

fn biguint_wrapping_div_assign()
{
    println!("biguint_wrapping_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_div_assign(&divisor);
    println!("After a_biguint.wrapping_div_assign(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_div_assign(&divisor);
    println!("After a_biguint.wrapping_div_assign(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.wrapping_div_assign(&_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = UU32::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.wrapping_div_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_overflowing_div()
{
    println!("biguint_overflowing_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let (quotient, overflow) = dividend.overflowing_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(overflow, false);
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let dividend = U256::zero();
    let divisor = U256::from_uint(87_u8);
    let (quotient, overflow) = dividend.overflowing_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    // It will panic!
    // let (quotient, overflow) = _dividend.overflowing_div(&_divisor);

    let _dividend = U256::zero();
    let _divisor = U256::zero();
    // It will panic!
    // let (quotient, overflow) = _dividend.overflowing_div(&_divisor);
    println!("---------------------------");
}

fn biguint_overflowing_div_assign()
{
    println!("biguint_overflowing_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let overflow = a_biguint.overflowing_div_assign(&divisor);
    println!("After a_biguint.overflowing_div_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let overflow = a_biguint.overflowing_div_assign(&divisor);
    println!("After a_biguint.overflowing_div_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    // It will panic!
    // let overflow = _a_biguint.overflowing_div_assign(&_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic!
    // let overflow = _a_biguint.overflowing_div_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_checked_div()
{
    println!("biguint_checked_div");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.checked_div(&divisor);
    match quotient
    {
        Some(q) =>
            {
                println!("{} / {} = {}", dividend, divisor, q);
                assert_eq!(q.to_string(), "1419043551905275201680884938348044216837079832");
                assert_eq!(q.is_overflow(), false);
                assert_eq!(q.is_underflow(), false);
                assert_eq!(q.is_infinity(), false);
                assert_eq!(q.is_undefined(), false);
                assert_eq!(q.is_divided_by_zero(), false);
                assert_eq!(q.is_left_carry(), false);
                assert_eq!(q.is_right_carry(), false);
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = U256::zero();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.checked_div(&divisor);
    match quotient
    {
        Some(q) =>
            {
                println!("{} / {} = {}", dividend, divisor, q);
                assert_eq!(q.to_string(), "0");
                assert_eq!(q.is_overflow(), false);
                assert_eq!(q.is_underflow(), false);
                assert_eq!(q.is_infinity(), false);
                assert_eq!(q.is_undefined(), false);
                assert_eq!(q.is_divided_by_zero(), false);
                assert_eq!(q.is_left_carry(), false);
                assert_eq!(q.is_right_carry(), false);
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::zero();
    let quotient = dividend.checked_div(&divisor);
    match quotient
    {
        Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(quotient, None);
            },
    }

    let dividend = U256::zero();
    let divisor = U256::zero();
    let quotient = dividend.checked_div(&divisor);
    match quotient
    {
        Some(q) => { println!("{} / {} = {}", dividend, divisor, q); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(quotient, None);
            },
    }
    println!("---------------------------");
}

fn biguint_unchecked_div()
{
    println!("biguint_unchecked_div");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let quotient = dividend.unchecked_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let quotient = dividend.unchecked_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    // It will panic.
    // let quotient = _dividend.uchecked_div_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic.
    // let quotient = _dividend.uchecked_div_uint(_divisor);
    println!("---------------------------");
}

fn biguint_saturating_div()
{
    println!("biguint_saturating_div");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let quotient = dividend.saturating_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let quotient = dividend.saturating_div(&divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    // It will panic!
    // let quotient = _dividend.saturating_div(&divisor);

    let _dividend = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic!
    // let quotient = _dividend.saturating_div(&divisor);
    println!("---------------------------");
}

fn biguint_saturating_div_assign()
{
    println!("biguint_saturating_div_assign");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_div_assign(&divisor);
    println!("After a_biguint.saturating_div_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_div_assign(&divisor);
    println!("After a_biguint.saturating_div_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.saturating_div_assign(&_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = UU32::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.saturating_div_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_modular_div()
{
    println!("biguint_modular_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.modular_div(&divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "3");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // dividend == 0
    let dividend = U256::zero();
    let modulo = U256::from_uint(250_u8);
    let divisor = U256::from_uint(3_u8);
    let res = dividend.modular_div(&divisor, &modulo);
    println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // dividend == multiple of modulo
    let dividend = U256::from_uint(750_u16);
    let modulo = U256::from_uint(250_u8);
    let divisor = U256::from_uint(3_u8);
    let res = dividend.modular_div(&divisor, &modulo);
    println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op2 == 0
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // op2 == multiple of modulo
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // op1 == 0 and op2 == 0
    let _a_biguint = U256::zero();
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // op1 == multiple of modulo and op2 == 0
    let _a_biguint = U256::from_uint(750_u16);
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // op1 == 0 and op2 == multiple of modulo
    let _a_biguint = U256::zero();
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let _a_biguint = U256::from_uint(150_u8);
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // let res = _a_biguint.modular_div(&_rhs, &_m);

    // modulo == 0
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::zero();
    // It will panic!
    // let quotient = _dividend.modular_div(&_divisor, &_modulo);

    // modulo == 1
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::one();
    // It will panic!
    // let quotient = _dividend.modular_div(&_divisor, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_div_assign()
{
    println!("biguint_modular_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case
    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "3");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // dividend == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(3_u8);
    let modulo = U256::from_uint(250_u8);
    a_biguint.modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // dividend == multiple of modulo
    let mut a_biguint = U256::from_uint(750_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let modulo = U256::from_uint(250_u8);
    let divisor = U256::from_uint(3_u8);
    a_biguint.modular_div_assign(&divisor, &modulo);
    println!("After a_biguint.modular_div_assign({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op2 == 0
    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // op2 == multiple of modulo
    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // op1 == 0 and op2 == 0
    let mut _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // op1 == multiple of modulo and op2 == 0
    let mut _a_biguint = U256::from_uint(750_u16);
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = U256::zero();
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // op1 == 0 and op2 == multiple of modulo
    let mut _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let mut _a_biguint = U256::from_uint(150_u8);
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = U256::from_uint(250_u8);
    // It will panic.
    // _a_biguint.modular_div_assign(&_rhs, &_m);

    // modulo == 0
    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::zero();
    // It will panic!
    // _a_biguint.modular_div_assign(&_divisor, &_modulo);

    // modulo == 1
    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::one();
    // It will panic!
    // _a_biguint.modular_div_assign(&_divisor, &_modulo);
    println!("---------------------------");
}


fn biguint_rem()
{
    biguint_wrapping_rem();
    biguint_wrapping_rem_assign();
    biguint_overflowing_rem();
    biguint_overflowing_rem_assign();
    biguint_checked_rem();
    biguint_unchecked_rem();
    biguint_saturating_rem();
    biguint_saturating_rem_assign();
    biguint_modular_rem();
    biguint_modular_rem_assign();
}

fn biguint_wrapping_rem()
{
    println!("biguint_wrapping_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.wrapping_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.wrapping_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    // It will panic!
    // let remainder = _dividend.wrapping_rem(&_divisor);

    let _dividend = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic!
    // let remainder = _dividend.wrapping_rem(&_divisor);
    println!("---------------------------");
}

fn biguint_wrapping_rem_assign()
{
    println!("biguint_wrapping_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::from_uint(87_u8);
    a_biguint.wrapping_rem_assign(&divisor);
    println!("After a_biguint.wrapping_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
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

    let divisor = U256::from_uint(87_u8);
    a_biguint.wrapping_rem_assign(&divisor);
    println!("After a_biguint.wrapping_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    // It will panic!
    // _a_biguint.wrapping_rem_assign(&_divisor);

    let mut _a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    // It will panic!
    // _a_biguint.wrapping_rem_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_overflowing_rem()
{
    println!("biguint_overflowing_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let (remainder, overflow) = dividend.overflowing_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(overflow, false);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let (remainder, overflow) = dividend.overflowing_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(overflow, false);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    // It will panic!
    // let (remainder, overflow) = _dividend.overflowing_rem(&_divisor);

    let _dividend = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic!
    // let (remainder, overflow) = _dividend.overflowing_rem(&_divisor);
    println!("---------------------------");
}

fn biguint_overflowing_rem_assign()
{
    println!("biguint_overflowing_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(87_u8);
    let overflow = a_biguint.overflowing_rem_assign(&divisor);
    println!("After a_biguint.overflowing_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
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

    let divisor = UU32::from_uint(87_u8);
    let overflow = a_biguint.overflowing_rem_assign(&divisor);
    println!("After a_biguint.overflowing_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic!
    // let overflow = _a_biguint.overflowing_rem_assign(&_divisor);

    let mut _a_biguint = U256::zero();
    let _divisor = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic!
    // let overflow = _a_biguint.overflowing_rem_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_checked_rem()
{
    println!("biguint_checked_rem");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.checked_rem(&divisor);
    match remainder
    {
        Some(r) =>
            {
                println!("{} % {} = {}", dividend, divisor, r);
                assert_eq!(r.to_string(), "8");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.checked_rem(&divisor);
    match remainder
    {
        Some(r) =>
            {
                println!("{} % {} = {}", dividend, divisor, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::zero();
    let remainder = dividend.checked_rem(&divisor);
    match remainder
    {
        Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(remainder, None);
            },
    }

    let dividend = UU32::zero();
    let divisor = UU32::zero();
    let remainder = dividend.checked_rem(&divisor);
    match remainder
    {
        Some(r) => { println!("{} % {} = {}", dividend, divisor, r); },
        None =>
            {
                println!("Divided By Zero");
                assert_eq!(remainder, None);
            },
    }
    println!("---------------------------");
}

fn biguint_unchecked_rem()
{
    println!("biguint_unchecked_rem");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(87_u8);
    let remainder = dividend.unchecked_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let dividend = U256::zero();
    let divisor = U256::from_uint(87_u8);
    let remainder = dividend.unchecked_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    // It will panic.
    // let remainder = _dividend.unchecked_rem(&_divisor);

    let _dividend = U256::zero();
    let _divisor = U256::zero();
    // It will panic.
    // let remainder = _dividend.unchecked_rem(&_divisor);
    println!("---------------------------");
}

fn biguint_saturating_rem()
{
    println!("biguint_saturating_rem");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.saturating_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = UU32::from_uint(87_u8);
    let remainder = dividend.saturating_rem(&divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = UU32::zero();
    // It will panic!
    // let remainder = _dividend.saturating_rem(&_divisor);

    let _dividend = UU32::zero();
    let _divisor = UU32::zero();
    // It will panic!
    // let remainder = _dividend.saturating_rem(&_divisor);
    println!("---------------------------");
}

fn biguint_saturating_rem_assign()
{
    println!("biguint_saturating_rem_assign");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = U256::from_uint(87_u8);
    a_biguint.saturating_rem_assign(&divisor);
    println!("After a_biguint.saturating_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
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

    let divisor = U256::from_uint(87_u8);
    a_biguint.saturating_rem_assign(&divisor);
    println!("After a_biguint.saturating_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.saturating_rem_assign(&_divisor);

    let mut _a_biguint = U256::zero();
    let _divisor = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.saturating_rem_assign(&_divisor);
    println!("---------------------------");
}

fn biguint_modular_rem()
{
    println!("biguint_modular_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem(&divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == 0 and divisor != 0
    let dividend = U256::zero();
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem(&divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == multiple of modulo and divisor != 0
    let dividend = U256::from_uint(200_u8);
    let divisor = U256::from_uint(128_u8);
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem(&divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend != 0 and divisor == 0    
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == 0
    let _dividend = U256::zero();
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    let _dividend = U256::zero();
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    let _dividend = U256::from_uint(200_u8);
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    let _dividend = U256::from_uint(200_u8);
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo == 0
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::zero();
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);

    // modulo == 1
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::one();
    // It will panic!
    // let quotient = _dividend.modular_rem(&_divisor, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_rem_assign()
{
    println!("biguint_modular_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case
    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and dividend == 0 and divisor != 0
    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and dividend == multiple of modulo and divisor != 0
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = UU32::from_uint(128_u8);
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign(&divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and dividend != 0 and divisor == 0
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == 0
    let _a_biguint = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    let _a_biguint = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    let _a_biguint = U256::from_uint(200_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::zero();
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    let _a_biguint = U256::from_uint(200_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(200_u8);
    let _modulo = U256::from_uint(100_u8);
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo == 0
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::zero();
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);

    // modulo == 1
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    let _divisor = U256::from_uint(128_u8);
    let _modulo = U256::one();
    // It will panic!
    // _a_biguint.modular_rem_assign(&_divisor, &_modulo);
    println!("---------------------------");
}


fn biguint_next_multiple()
{
    biguint_next_multiple_of();
    biguint_next_multiple_of_assign();
    biguint_modular_next_multiple_of();
    biguint_modular_next_multiple_of_assign();
    biguint_is_multiple_of();
}

fn biguint_next_multiple_of()
{
    println!("biguint_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = U256::from(586478_u32);
    let multiple = a_biguint.next_multiple_of(&num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "123456789012345678901234567890123697594");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    let a_biguint = U256::max();
    let num = U256::from(586478_u32);
    let multiple = a_biguint.next_multiple_of(&num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "448670");
    assert_eq!(multiple.is_overflow(), true);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = U256::zero();
    // It will panic.
    // let _multiple = _a_biguint.next_multiple_of(&_num);
    println!("---------------------------");
}

fn biguint_next_multiple_of_assign()
{
    println!("biguint_next_multiple_of_assign");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = UU32::from(586478_u32);
    a_biguint.next_multiple_of_assign(&num);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = UU32::from(586478_u32);
    a_biguint.next_multiple_of_assign(&num);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "448670");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = UU32::zero();
    // It will panic.
    // _a_biguint.next_multiple_of_assign(&_num);
    println!("---------------------------");
}

fn biguint_modular_next_multiple_of()
{
    println!("biguint_modular_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = U256::from(100_u8);
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.modular_next_multiple_of(&num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "123456789012345678901234567890123456800");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::max();
    let num = U256::from(100_u8);
    let modulo = a_biguint.wrapping_add_uint(200_u8); println!("modulo = {}", modulo);
    let multiple = a_biguint.modular_next_multiple_of(&num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "1");
    assert_eq!(multiple.is_overflow(), true);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // rhs == 0
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = U256::zero();
    let _modulo = a_biguint.wrapping_add_uint(200_u8);
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);

    // rhs == multiple of modulo
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = U256::from(200_u8);
    let _modulo = U256::from(100_u8);
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);

    // modulo == 0
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = U256::from(100_u8);
    let _modulo = U256::zero();
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);

    // modulo == 1
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = U256::from(100_u8);
    let _modulo = U256::one();
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of(&_num, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_next_multiple_of_assign()
{
    println!("biguint_modular_next_multiple_of_assign");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = UU32::from(100_u8);
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.modular_next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123456800");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = UU32::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = UU32::from(100_u8);
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.modular_next_multiple_of_assign(&num, &modulo);
    println!("After a_biguint.next_multiple_of_assign({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == 0
    let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _num = UU32::zero();
    let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    // _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);

    // rhs == multiple of modulo
    let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _num = UU32::from(200_u8);
    let _modulo = UU32::from(100_u8);
    // _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);

    // modulo == 0
    let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _num = UU32::from(100_u8);
    let _modulo = UU32::zero();
    // _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);

    // modulo == 1
    let mut _a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", _a_biguint);
    let _num = UU32::from(100_u8);
    let _modulo = UU32::one();
    // _a_biguint.modular_next_multiple_of_assign(&_num, &_modulo);
    println!("---------------------------");
}

fn biguint_is_multiple_of()
{
    println!("biguint_is_multiple_of()");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = U256::from(100_u8);
    let ans = a_biguint.is_multiple_of(&rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, true);

    // Normal case 2
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = U256::from(99_u8);
    let ans = a_biguint.is_multiple_of(&rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, false);

    // rhs == 0 and self != 0
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = U256::zero();
    let ans = a_biguint.is_multiple_of(&rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, false);

    // rhs == 0 and self == 0
    let a_biguint = U256::zero();
    let rhs = U256::zero();
    let ans = a_biguint.is_multiple_of(&rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, true);
    println!("---------------------------");
}
