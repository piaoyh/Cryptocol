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
    biguint_add_uint();
    biguint_sub_uint();
    biguint_mul_uint();
    biguint_div_uint();
    biguint_rem_uint();
    biguint_next_multiple_uint();
}

fn biguint_add_uint()
{
    biguint_carrying_add_uint();
    biguint_carrying_add_assign_uint();
    biguint_wrapping_add_uint();
    biguint_wrapping_add_assign_uint();
    biguint_overflowing_add_uint();
    biguint_overflowing_add_assign_uint();
    biguint_checked_add_uint();
    biguint_unchecked_add_uint();
    biguint_saturating_add_uint();
    biguint_saturating_add_assign_uint();
    biguint_modular_add_uint();
    biguint_modular_add_assign_uint();
    biguint_panic_free_modular_add_uint();
    biguint_panic_free_modular_add_assign_uint();
    biguint_safe_add_uint();
    biguint_safe_add_assign_uint();
}

fn biguint_carrying_add_uint()
{
    println!("biguint_carrying_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;

    let (sum, carry) = a_biguint.carrying_add_uint(num_uint, false);
    println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722605");
    assert_eq!(carry, false);
    assert_eq!(sum.is_overflow(), false);
    assert_eq!(sum.is_underflow(), false);
    assert_eq!(sum.is_divided_by_zero(), false);
    assert_eq!(sum.is_infinity(), false);
    assert_eq!(sum.is_undefined(), false);
    assert_eq!(sum.is_left_carry(), false);
    assert_eq!(sum.is_right_carry(), false);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;

    let (sum, carry) = a_biguint.carrying_add_uint(num_uint, true);
    println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "115761816335569101403435733562708448393664880666628652711615198738168793722606");
    assert_eq!(carry, false);
    assert_eq!(sum.is_overflow(), false);
    assert_eq!(sum.is_underflow(), false);
    assert_eq!(sum.is_divided_by_zero(), false);
    assert_eq!(sum.is_infinity(), false);
    assert_eq!(sum.is_undefined(), false);
    assert_eq!(sum.is_left_carry(), false);
    assert_eq!(sum.is_right_carry(), false);

    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;

    let (sum, carry) = a_biguint.carrying_add_uint(num_uint, false);
    println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "22774453838368691933710012711845097214");
    assert_eq!(carry, true);
    assert_eq!(sum.is_overflow(), true);
    assert_eq!(sum.is_underflow(), false);
    assert_eq!(sum.is_divided_by_zero(), false);
    assert_eq!(sum.is_infinity(), false);
    assert_eq!(sum.is_undefined(), false);
    assert_eq!(sum.is_left_carry(), false);
    assert_eq!(sum.is_right_carry(), false);

    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFF_u128;

    let (sum, carry) = a_biguint.carrying_add_uint(num_uint, true);
    println!("{} + {} = {}\ncarry = {}", a_biguint, num_uint, sum, carry);
    assert_eq!(sum.to_string(), "22774453838368691933710012711845097215");
    assert_eq!(carry, true);
    assert_eq!(sum.is_overflow(), true);
    assert_eq!(sum.is_underflow(), false);
    assert_eq!(sum.is_divided_by_zero(), false);
    assert_eq!(sum.is_infinity(), false);
    assert_eq!(sum.is_undefined(), false);
    assert_eq!(sum.is_left_carry(), false);
    assert_eq!(sum.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_carrying_add_assign_uint()
{
    println!("biguint_carrying_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num_uint = 0x9900AABB_CCDDEEFF_u64;
    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let carry = a_biguint.carrying_add_assign_uint(num_uint, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302573");
    assert_eq!(carry, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_uint = 0x9900AABB_CCDDEEFF_u64;
    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let carry = a_biguint.carrying_add_assign_uint(num_uint, true);
    println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019692513725068324302574");
    assert_eq!(carry, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_uint = 0x9900AABB_CCDDEEFF_u64;
    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let carry = a_biguint.carrying_add_assign_uint(num_uint, false);
    println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    assert_eq!(a_biguint.to_string(), "11024999611375677182");
    assert_eq!(carry, true);
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_uint = 0x9900AABB_CCDDEEFF_u64;
    let num_str2 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let carry = a_biguint.carrying_add_assign_uint(num_uint, true);
    println!("After a_biguint += {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, carry);
    assert_eq!(a_biguint.to_string(), "11024999611375677183");
    assert_eq!(carry, true);
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_add_uint()
{
    println!("biguint_wrapping_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.wrapping_add_uint(1_u8);
    println!("{} + 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.wrapping_add_uint(2_u8);
    println!("{} + 2 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.wrapping_add_uint(3_u8);
    println!("{} + 3 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_add_assign_uint()
{
    println!("biguint_wrapping_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_add_assign_uint(1_u8);
    println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::max();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_add_assign_uint(1_u8);
    println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_add_assign_uint(1_u8);
    println!("After a_biguint.wrapping_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_add_uint()
{
    println!("biguint_overflowing_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let (res, overflow) = a_biguint.overflowing_add_uint(1_u8);
    println!("{} + 1 = {}\noverflow = {}", a_biguint, res, overflow);
    assert_eq!(res, U512::max());
    assert_eq!(overflow, false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let (res, overflow) = a_biguint.overflowing_add_uint(2_u8);
    println!("{} + 2 = {}\noverflow = {}", a_biguint, res, overflow);
    assert_eq!(res.to_string(), "0");
    assert_eq!(overflow, true);
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let (res, overflow) = a_biguint.overflowing_add_uint(3_u8);
    println!("{} + 3 = {}\noverflow = {}", a_biguint, res, overflow);
    assert_eq!(res.to_string(), "1");
    assert_eq!(overflow, true);
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_add_assign_uint()
{
    println!("biguint_overflowing_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_add_assign_uint(1_u8);
    println!("After a_biguint.overflowing_add_assign_uint(1_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut overflow = a_biguint.overflowing_add_assign_uint(2_u8);
    println!("After a_biguint.overflowing_add_assign_uint(2_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    overflow = a_biguint.overflowing_add_assign_uint(2_u8);
    println!("After a_biguint.overflowing_add_assign_uint(2_u8), a_biguint = {}\noverflow = {}", a_biguint, overflow);
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_add_uint()
{
    println!("biguint_checked_add_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.checked_add_uint(1_u8);
    match res
    {
        Some(num) => {
            println!("{} + 1 = {}", a_biguint, num);
            assert_eq!(num, U512::max());
            assert_eq!(num.is_overflow(), false);
            assert_eq!(num.is_underflow(), false);
            assert_eq!(num.is_divided_by_zero(), false);
            assert_eq!(num.is_infinity(), false);
            assert_eq!(num.is_undefined(), false);
            assert_eq!(num.is_left_carry(), false);
            assert_eq!(num.is_right_carry(), false);
        },
        None => {
            println!("{} + 1 = overflow", a_biguint);
        }
    }

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.checked_add_uint(2_u8);
    match res
    {
        Some(num) => {
            println!("{} + 2 = {}", a_biguint, num);
        },
        None => {
            println!("{} + 2 = overflow", a_biguint);
            assert_eq!(res, None);
        }
    }

    let a_biguint = U512::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.checked_add_uint(3_u8);
    match res
    {
        Some(num) => {
            println!("{} + 3 = {}", a_biguint, num);
        },
        None => {
            println!("{} + 3 = overflow", a_biguint);
            assert_eq!(res, None);
        }
    }
    println!("---------------------------");
}

fn biguint_unchecked_add_uint()
{
    println!("biguint_unchecked_add_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    let res = a_biguint.unchecked_add_uint(1_u8);
    println!("{} + 1 = {}", a_biguint, res);
    assert_eq!(res, UU64::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_unchecked_add_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_unchecked_add_uint()
{
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    let _res = _a_biguint.unchecked_add_uint(2_u8);
}

fn biguint_saturating_add_uint()
{
    println!("biguint_saturating_add_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    let res = a_biguint.saturating_add_uint(1_u8);
    println!("{} + 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    let res = a_biguint.saturating_add_uint(2_u8);
    println!("{} + 2 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::max().wrapping_sub_uint(2_u8);
    let res = a_biguint.saturating_add_uint(3_u8);
    println!("{} + 3 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_saturating_add_assign_uint()
{
    println!("biguint_saturating_add_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_add_assign_uint(1_u8);
    println!("After a_biguint.saturating_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_add_assign_uint(2_u8);
    println!("After a_biguint.saturating_add_assign_uint(2_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU64::max().wrapping_sub_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_add_assign_uint(3_u8);
    println!("After a_biguint.saturating_add_assign_uint(3_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_add_uint()
{
    println!("biguint_modular_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 1_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 2_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 3_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}", a_biguint, rhs, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let rhs = 3_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_add_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_add_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::zero();
    let _rhs = 3_u8;
    let _res = _a_biguint.modular_add_uint(_rhs, &_m);

    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::one();
    let _rhs = 3_u8;
    let _res = _a_biguint.modular_add_uint(_rhs, &_m);
}

fn biguint_modular_add_assign_uint()
{
    println!("biguint_modular_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case 1
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 1_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 2_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    println!("Originally,a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 3_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.modular_add_assign_uint(1_u8, &m);
    println!("After a_biguint.modular_add_assign_uint(1_u8, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "150");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_add_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_add_assign_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::zero();
    let _rhs = 1_u8;
   
    _a_biguint.modular_add_assign_uint(_rhs, &_m);

    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::one();
    let _rhs = 1_u8;
    _a_biguint.modular_add_assign_uint(_rhs, &_m);
}

fn biguint_panic_free_modular_add_uint()
{
    println!("biguint_panic_free_modular_add_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 1_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 2_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 == 0
    let a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 0
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::zero();
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 1
    let a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::one();
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
    println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // collectively
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_add_uint(rhs, &m);
                println!("{} + {} = {} (mod {})", a_biguint, rhs, res, m);
                assert_eq!(res.to_string(), "0");
                assert_eq!(res.is_overflow(), false);
                assert_eq!(res.is_underflow(), false);
                assert_eq!(res.is_divided_by_zero(), false);
                assert_eq!(res.is_infinity(), false);
                assert_eq!(res.is_undefined(), true);
                assert_eq!(res.is_left_carry(), false);
                assert_eq!(res.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_modular_add_assign_uint()
{
    println!("biguint_panic_free_modular_add_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    let m = a_biguint.wrapping_add_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 1_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 2_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = a_biguint.wrapping_add_uint(2_u8);
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_add_assign_uint(1_u8, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint(1_u8, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "150");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    let m = U256::zero();
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1
    let mut a_biguint = U256::from_string("768018742981669034276900318581864860508537538828119465699464336490060").unwrap();
    let m = U256::one();
    println!("Originally, a = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "768018742981669034276900318581864860508537538828119465699464336490060");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_add_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // collectively
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let mut a_biguint = a.clone();
                println!("Originally, a = {}", a_biguint);
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), false);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            
                a_biguint.panic_free_modular_add_assign_uint(rhs, &m);
                println!("After a_biguint.panic_free_modular_add_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
                assert_eq!(a_biguint.to_string(), "0");
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), true);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_safe_add_uint()
{
    println!("biguint_safe_add_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U512::max().safe_sub_uint(1_u8);
    let res = a_biguint.safe_add_uint(1_u8);
    println!("{} + 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let a_biguint = U512::max().safe_sub_uint(1_u8);
        let res = a_biguint.safe_add_uint(2_u8);
        println!("{} + 2 = {}", a_biguint, res);
        assert_eq!(res.to_string(), "0");
        assert_eq!(res.is_overflow(), true);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);

        let a_biguint = U512::max().safe_sub_uint(1_u8);
        let res = a_biguint.safe_add_uint(3_u8);
        println!("{} + 3 = {}", a_biguint, res);
        assert_eq!(res.to_string(), "1");
        assert_eq!(res.is_overflow(), true);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_add_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_add_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u128);
    
        let _a_biguint = U512::max().wrapping_sub_uint(1_u8);
        let _res = _a_biguint.safe_add_uint(2_u8);
        let _res = _a_biguint.safe_add_uint(3_u8);
    }
}

fn biguint_safe_add_assign_uint()
{
    println!("biguint_safe_add_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU64::max().safe_sub_uint(1_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.safe_add_assign_uint(1_u8);
    println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU64::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    #[cfg(not(debug_assertions))]
    {
        let mut a_biguint = UU64::max();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_add_assign_uint(1_u8);
        println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "0");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_add_assign_uint(1_u8);
        println!("After a_biguint.safe_add_assign_uint(1_u8), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "1");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_add_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_add_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u8);
    
        let mut _a_biguint = U512::max();
        _a_biguint.safe_add_assign_uint(1_u8);
        _a_biguint.safe_add_assign_uint(1_u8);
    }
}


fn biguint_sub_uint()
{
    biguint_borrowing_sub_uint();
    biguint_borrowing_sub_assign_uint();
    biguint_wrapping_sub_uint();
    biguint_wrapping_sub_assign_uint();
    biguint_overflowing_sub_uint();
    biguint_overflowing_sub_assign_uint();
    biguint_checked_sub_uint();
    biguint_unchecked_sub_uint();
    biguint_saturating_sub_uint();
    biguint_saturating_sub_assign_uint();
    biguint_modular_sub_uint();
    biguint_modular_sub_assign_uint();
    biguint_panic_free_modular_sub_uint();
    biguint_panic_free_modular_sub_assign_uint();
    biguint_abs_diff_uint();
    biguint_safe_sub_uint();
    biguint_safe_sub_assign_uint();
}

fn biguint_borrowing_sub_uint()
{
    println!("biguint_borrowing_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;

    let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, false);
    println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528175");
    assert_eq!(borrow, false);
    assert_eq!(dif.is_underflow(), false);
    assert_eq!(dif.is_overflow(), false);
    assert_eq!(dif.is_divided_by_zero(), false);
    assert_eq!(dif.is_infinity(), false);
    assert_eq!(dif.is_undefined(), false);
    assert_eq!(dif.is_left_carry(), false);
    assert_eq!(dif.is_right_carry(), false);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let a_biguint = UU32::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;

    let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, true);
    println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    assert_eq!(dif.to_string(), "115761816335569101403435733562708448393619331758951915327747778712745103528174");
    assert_eq!(borrow, false);
    assert_eq!(dif.is_underflow(), false);
    assert_eq!(dif.is_overflow(), false);
    assert_eq!(dif.is_divided_by_zero(), false);
    assert_eq!(dif.is_infinity(), false);
    assert_eq!(dif.is_undefined(), false);
    assert_eq!(dif.is_left_carry(), false);
    assert_eq!(dif.is_right_carry(), false);

    let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;

    let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, false);
    println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639919");
    assert_eq!(borrow, true);
    assert_eq!(dif.is_underflow(), true);
    assert_eq!(dif.is_overflow(), false);
    assert_eq!(dif.is_divided_by_zero(), false);
    assert_eq!(dif.is_infinity(), false);
    assert_eq!(dif.is_undefined(), false);
    assert_eq!(dif.is_left_carry(), false);
    assert_eq!(dif.is_right_carry(), false);

    let num_str2 = "11223344_55667788_9900AABB_CCDDEEEe";
    let a_biguint = UU32::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x11223344_55667788_9900AABB_CCDDEEFf_u128;

    let (dif, borrow) = a_biguint.borrowing_sub_uint(num_uint, true);
    println!("{} - {} = {}\nborrow = {}", a_biguint, num_uint, dif, borrow);
    assert_eq!(dif.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639918");
    assert_eq!(borrow, true);
    assert_eq!(dif.is_underflow(), true);
    assert_eq!(dif.is_overflow(), false);
    assert_eq!(dif.is_divided_by_zero(), false);
    assert_eq!(dif.is_infinity(), false);
    assert_eq!(dif.is_undefined(), false);
    assert_eq!(dif.is_left_carry(), false);
    assert_eq!(dif.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_borrowing_sub_assign_uint()
{
    println!("biguint_borrowing_sub_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFf_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, false);
    println!("After a_biguint -= {},\ta_biguint = {}\tborrow = {}", num_uint, a_biguint, borrow);
    assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948207");
    assert_eq!(borrow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_str1 = "FFEEDDBB_AA998877_66554433_221100FF_EEDDBBAA_99887766_55443322_1100FFEE";
    let mut a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFf_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, true);
    println!("After a_biguint -= {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, borrow);
    assert_eq!(a_biguint.to_string(), "115761816335569101403435733562708448393642106212790284019670463725845572948206");
    assert_eq!(borrow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_str2 = "9900AABB_CCDDEEFe";
    let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFf_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, false);
    println!("After a_biguint -= {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, borrow);
    assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(borrow, true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num_str2 = "9900AABB_CCDDEEFe";
    let mut a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFf_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let borrow = a_biguint.borrowing_sub_assign_uint(num_uint, true);
    println!("After a_biguint -= {},\ta_biguint = {}\tcarry = {}", num_uint, a_biguint, borrow);
    assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639934");
    assert_eq!(borrow, true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_sub_uint()
{
    println!("biguint_wrapping_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U512::one();
    let res = a_biguint.wrapping_sub_uint(1_u8);
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let res = a_biguint.wrapping_sub_uint(2_u8);
    println!("{} - 2 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let res = a_biguint.wrapping_sub_uint(3_u8);
    println!("{} - 3 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_sub_assign_uint()
{
    println!("biguint_wrapping_sub_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    a_biguint.wrapping_sub_assign_uint(1_u8);
    println!("After a_biguint.wrapping_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign_uint(2_u8);
    println!("After a_biguint.wrapping_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign_uint(3_u8);
    println!("After a_biguint.wrapping_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_sub_assign_uint(1_u8);
    println!("After a_biguint.wrapping_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_sub_uint()
{
    println!("biguint_overflowing_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U512::one();
    let (res, underflow) = a_biguint.overflowing_sub_uint(1_u8);
    println!("{} - 1 = {}\nunderflow = {}", a_biguint, res, underflow);
    assert_eq!(res.to_string(), "0");
    assert_eq!(underflow, false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let (res, underflow) = a_biguint.overflowing_sub_uint(2_u8);
    println!("{} - 2 = {}\nunderflow = {}", a_biguint, res, underflow);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(underflow, true);
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::one();
    let (res, underflow) = a_biguint.overflowing_sub_uint(3_u8);
    println!("{} - 3 = {}\nunderflow = {}", a_biguint, res, underflow);
    assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(underflow, true);
    assert_eq!(res.is_underflow(), true);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_sub_assign_uint()
{
    println!("biguint_overflowing_sub_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = a_biguint.overflowing_sub_assign_uint(1_u8);
    println!("After a_biguint.overflowing_sub_assign_uint(1_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(underflow, false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = a_biguint.overflowing_sub_assign_uint(2_u8);
    println!("After a_biguint.overflowing_sub_assign_uint(2_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
    assert_eq!(underflow, true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = a_biguint.overflowing_sub_assign_uint(3_u8);
    println!("After a_biguint.overflowing_sub_assign_uint(3_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(underflow, true);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let underflow = a_biguint.overflowing_sub_assign_uint(1_u8);
    println!("After a_biguint.overflowing_sub_assign_uint(1_u8), a_biguint = {}\nunderflow = {}", a_biguint, underflow);
    assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(underflow, false);
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_sub_uint()
{
    println!("biguint_checked_sub_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U512::one();
    let res = a_biguint.checked_sub_uint(1_u8);
    match res
    {
        Some(num) => {
            println!("{} - 1 = {}", a_biguint, num);
            assert_eq!(num.to_string(), "0");
            assert_eq!(num.is_underflow(), false);
            assert_eq!(num.is_overflow(), false);
            assert_eq!(num.is_divided_by_zero(), false);
            assert_eq!(num.is_infinity(), false);
            assert_eq!(num.is_undefined(), false);
            assert_eq!(num.is_left_carry(), false);
            assert_eq!(num.is_right_carry(), false);
        },
        None => {
            println!("{} - 1 = overflow", a_biguint);
        }
    }

    let a_biguint = U512::one();
    let res = a_biguint.checked_sub_uint(2_u8);
    match res
    {
        Some(num) => {
            println!("{} - 2 = {}", a_biguint, num);
        },
        None => {
            println!("{} - 2 = overflow", a_biguint);
            assert_eq!(res, None);
        }
    }

    let a_biguint = U512::one();
    let res = a_biguint.checked_sub_uint(3_u8);
    match res
    {
        Some(num) => {
            println!("{} - 3 = {}", a_biguint, num);
        },
        None => {
            println!("{} - 3 = overflow", a_biguint);
            assert_eq!(res, None);
        }
    }
    println!("---------------------------");
}

fn biguint_unchecked_sub_uint()
{
    println!("biguint_unchecked_sub_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = UU64::one();
    let res = a_biguint.unchecked_sub_uint(1_u8);
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_should_panic_unchecked_sub_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_unchecked_sub_uint()
{
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _a_biguint = UU64::one();
    let _res = _a_biguint.unchecked_sub_uint(2_u8);
}

fn biguint_saturating_sub_uint()
{
    println!("biguint_saturating_sub_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);
    
    let a_biguint = U512::from_uint(2_u8);
    let res = a_biguint.saturating_sub_uint(1_u8);
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::from_uint(2_u8);
    let res = a_biguint.saturating_sub_uint(2_u8);
    println!("{} - 2 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U512::from_uint(2_u8);
    let res = a_biguint.saturating_sub_uint(3_u8);
    println!("{} - 3 = {}", a_biguint, res);
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

fn biguint_saturating_sub_assign_uint()
{
    println!("biguint_saturating_sub_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = UU64::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_sub_assign_uint(1_u8);
    println!("After a_biguint.saturating_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_sub_assign_uint(2_u8);
    println!("After a_biguint.saturating_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let mut a_biguint = UU64::from_uint(2_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_sub_assign_uint(3_u8);
    println!("After a_biguint.saturating_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
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

fn biguint_modular_sub_uint()
{
    println!("biguint_modular_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 1_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 2_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 3
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 3_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
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
    let rhs = 3_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
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
    let rhs = 0_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_sub_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_sub_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::zero();
    let _rhs = 3_u8;
    let _res = _a_biguint.modular_sub_uint(_rhs, &_m);

    let _a_biguint = U256::from_uint(2_u8);
    let _m = U256::one();
    let _rhs = 3_u8;
    let _res = _a_biguint.modular_sub_uint(_rhs, &_m);
}

fn biguint_modular_sub_assign_uint()
{
    println!("biguint_modular_sub_assign_uint");
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
    let rhs = 1_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 2_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 3_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 3_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 3_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 3_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 0_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 250_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 0_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 0_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
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
    let rhs = 250_u8;
    a_biguint.modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_sub_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_sub_assign_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut _a_biguint = U256::from_uint(2_u8);
    let _m = U256::zero();
    let _rhs = 1_u8;
    _a_biguint.modular_sub_assign_uint(_rhs, &_m);

    let mut _a_biguint = U256::from_uint(2_u8);
    let _m = U256::one();
    let _rhs = 1_u8;
    _a_biguint.modular_sub_assign_uint(_rhs, &_m);
}

fn biguint_panic_free_modular_sub_uint()
{
    println!("biguint_panic_free_modular_sub_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 1_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "1");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 2_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 3
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
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
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 3_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
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
    let rhs = 0_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
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
    let rhs = 250_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {}(mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 0
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::zero();
    let rhs = 1_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 1
    let a_biguint = U256::from_uint(2_u8);
    let m = U256::one();
    let rhs = 1_u8;
    let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
    println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // collectively
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_sub_uint(rhs, &m);
                println!("{} - {} = {} (mod {})", a_biguint, rhs, res, m);
                assert_eq!(res.to_string(), "0");
                assert_eq!(res.is_overflow(), false);
                assert_eq!(res.is_underflow(), false);
                assert_eq!(res.is_divided_by_zero(), false);
                assert_eq!(res.is_infinity(), false);
                assert_eq!(res.is_undefined(), true);
                assert_eq!(res.is_left_carry(), false);
                assert_eq!(res.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_modular_sub_assign_uint()
{
    println!("biguint_panic_free_modular_sub_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let mut a_biguint = UU32::from_uint(2_u8);
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 1_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
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
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 2_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
 
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
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
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let rhs = 3_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
 
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006084093");
    assert_eq!(a_biguint.is_underflow(), true);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "150");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0
    let mut a_biguint = U256::from_uint(2_u8);
    let m = U256::zero();
    let rhs = 3_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
 
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1
    let mut a_biguint = U256::from_uint(2_u8);
    let m = U256::one();
    let rhs = 3_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "2");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
 
    a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
    println!("After a_biguint.modular_sub_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // collectively
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let mut a_biguint = a.clone();
                println!("Originally, a = {}", a_biguint);
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), false);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            
                a_biguint.panic_free_modular_sub_assign_uint(rhs, &m);
                println!("After a_biguint.panic_free_modular_sub_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
                assert_eq!(a_biguint.to_string(), "0");
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), true);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_abs_diff_uint()
{
    println!("biguint_abs_diff_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let num_str1 = "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF";
    let a_biguint = U256::from_str_radix(num_str1, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    let res = a_biguint.abs_diff_uint(num_uint);
    println!("| {} - {} | = {}", a_biguint, num_uint, res);
    assert_eq!(res.to_string(), "115792089237316195423570985008687907853066609319396769656704041438214461985024");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let num_str2 = "12345678_9ABCDEF0_12345678_9ABCDEF0";
    let a_biguint = U256::from_str_radix(num_str2, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    let res = a_biguint.abs_diff_uint(num_uint);
    println!("| {} - {} | = {}", a_biguint, num_uint, res);
    assert_eq!(res.to_string(), "179177489040527647888749252028162707471");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let num_str3 = "9900AABB_CCDDEEFF_9900AABB_CCDDEEFF";
    let a_biguint = U256::from_str_radix(num_str3, 16).unwrap();
    let num_uint = 0x9900AABB_CCDDEEFF_9900AABB_CCDDEEFF_u128;
    let res = a_biguint.abs_diff_uint(num_uint);
    println!("| {} - {} | = {}", a_biguint, num_uint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_safe_sub_uint()
{
    println!("biguint_wrapping_sub_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U512::one();
    let res = a_biguint.safe_sub_uint(1_u8);
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(not(debug_assertions))]
    {
        let a_biguint = U512::one();
        let res = a_biguint.safe_sub_uint(2_u8);
        println!("{} - 2 = {}", a_biguint, res);
        assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        assert_eq!(res.is_underflow(), true);
        assert_eq!(res.is_overflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);

        let a_biguint = U512::one();
        let res = a_biguint.safe_sub_uint(3_u8);
        println!("{} - 3 = {}", a_biguint, res);
        assert_eq!(res.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
        assert_eq!(res.is_underflow(), true);
        assert_eq!(res.is_overflow(), false);
        assert_eq!(res.is_divided_by_zero(), false);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), false);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_sub_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_sub_uint()
{
    #[cfg(not(debug_assertions))]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u8);

        let _a_biguint = U512::one();
        // It will panic.
        let _res = _a_biguint.safe_sub_uint(2_u8);

        let _a_biguint = U512::one();
        // It will panic.
        let _res = _a_biguint.safe_sub_uint(3_u8);
    }
}

fn biguint_safe_sub_assign_uint()
{
    println!("biguint_safe_sub_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU64::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    a_biguint.safe_sub_assign_uint(1_u8);
    println!("After a_biguint.safe_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
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
        let mut a_biguint = UU64::one();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "1");
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_sub_assign_uint(2_u8);
        println!("After a_biguint.safe_sub_assign_uint(2_u8), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");
        assert_eq!(a_biguint.is_underflow(), true);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let mut a_biguint = UU64::one();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "1");
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_sub_assign_uint(3_u8);
        println!("After a_biguint.safe_sub_assign_uint(3_u8), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084094");
        assert_eq!(a_biguint.is_underflow(), true);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        a_biguint.safe_sub_assign_uint(1_u8);
        println!("After a_biguint.safe_sub_assign_uint(1_u8), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084093");
        assert_eq!(a_biguint.is_underflow(), true);
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }

    #[cfg(test)] // It will panic.
    biguint_should_panic_safe_sub_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_sub_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u16);

        let mut _a_biguint = UU64::one();
        // It will panic.
        _a_biguint.safe_sub_assign_uint(2_u8);

        let mut _a_biguint = UU64::one();
        // It will panic.
        _a_biguint.safe_sub_assign_uint(3_u8);
    }
}

fn biguint_mul_uint()
{
    biguint_carrying_mul_uint();
    biguint_carrying_mul_assign_uint();
    biguint_widening_mul_uint();
    biguint_widening_mul_assign_uint();
    biguint_wrapping_mul_uint();
    biguint_wrapping_mul_assign_uint();
    biguint_overflowing_mul_uint();
    biguint_overflowing_mul_assign_uint();
    biguint_checked_mul_uint();
    biguint_unchecked_mul_uint();
    biguint_saturating_mul_uint();
    biguint_saturating_mul_assign_uint();
    biguint_modular_mul_uint();
    biguint_modular_mul_assign_uint();
    biguint_panic_free_modular_mul_uint();
    biguint_panic_free_modular_mul_assign_uint();
    biguint_safe_mul_uint();
    biguint_safe_mul_assign_uint();
}

fn biguint_carrying_mul_uint()
{
    println!("biguint_carrying_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case
    let a_low_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_high_biguint = UU32::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_uint = 225_u8;
    let (res_low, res_high) = a_low_biguint.carrying_mul_uint(b_uint, UU32::zero());
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    let (res_high, res_higher) = a_high_biguint.carrying_mul_uint(b_uint, res_high);
    println!("{}:{} X {} = {}:{}:{}", a_high_biguint, a_low_biguint, b_uint, res_higher, res_high, res_low);
    assert_eq!(res_higher.to_string(), "0");
    assert_eq!(res_higher.is_overflow(), false);
    assert_eq!(res_higher.is_underflow(), false);
    assert_eq!(res_higher.is_divided_by_zero(), false);
    assert_eq!(res_higher.is_infinity(), false);
    assert_eq!(res_higher.is_undefined(), false);
    assert_eq!(res_higher.is_left_carry(), false);
    assert_eq!(res_higher.is_right_carry(), false);

    assert_eq!(res_high.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    assert_eq!(res_low.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    assert_eq!(res_low.is_overflow(), false);
    assert_eq!(res_low.is_underflow(), false);
    assert_eq!(res_low.is_divided_by_zero(), false);
    assert_eq!(res_low.is_infinity(), false);
    assert_eq!(res_low.is_undefined(), false);
    assert_eq!(res_low.is_left_carry(), false);
    assert_eq!(res_low.is_right_carry(), false);

    // Maximum case
    let a_low_biguint = U256::max();
    let a_high_biguint = UU32::max();
    let b_uint = u64::MAX;
    let (res_low, res_high) = a_low_biguint.carrying_mul_uint(b_uint, UU32::zero());
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    let (res_high, res_higher) = a_high_biguint.carrying_mul_uint(b_uint, res_high);
    println!("{}:{} X {:X} = {}:{}:{}", a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_uint, res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), res_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_low.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    assert_eq!(res_higher.is_overflow(), false);
    assert_eq!(res_higher.is_underflow(), false);
    assert_eq!(res_higher.is_divided_by_zero(), false);
    assert_eq!(res_higher.is_infinity(), false);
    assert_eq!(res_higher.is_undefined(), false);
    assert_eq!(res_higher.is_left_carry(), false);
    assert_eq!(res_higher.is_right_carry(), false);

    assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000040");
    assert_eq!(res_high.is_overflow(), true);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    assert_eq!(res_low.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    assert_eq!(res_low.is_overflow(), true);
    assert_eq!(res_low.is_underflow(), false);
    assert_eq!(res_low.is_divided_by_zero(), false);
    assert_eq!(res_low.is_infinity(), false);
    assert_eq!(res_low.is_undefined(), false);
    assert_eq!(res_low.is_left_carry(), false);
    assert_eq!(res_low.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_carrying_mul_assign_uint()
{
    println!("biguint_carrying_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case
    let mut a_low_biguint = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_high_biguint = U256::from_string("75388281194656994643364900608409476801874298166903427690031858186486050853").unwrap();
    let b_uint = 225_u8;

    println!("Originally, a_low_biguint = {}", a_low_biguint);
    assert_eq!(a_low_biguint.is_overflow(), false);
    assert_eq!(a_low_biguint.is_underflow(), false);
    assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    assert_eq!(a_low_biguint.is_infinity(), false);
    assert_eq!(a_low_biguint.is_undefined(), false);
    assert_eq!(a_low_biguint.is_left_carry(), false);
    assert_eq!(a_low_biguint.is_right_carry(), false);

    println!("Originally, a_high_biguint = {}", a_high_biguint);
    assert_eq!(a_high_biguint.is_overflow(), false);
    assert_eq!(a_high_biguint.is_underflow(), false);
    assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    assert_eq!(a_high_biguint.is_infinity(), false);
    assert_eq!(a_high_biguint.is_undefined(), false);
    assert_eq!(a_high_biguint.is_left_carry(), false);
    assert_eq!(a_high_biguint.is_right_carry(), false);

    let res_high = a_low_biguint.carrying_mul_assign_uint(b_uint, UU32::zero());
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    println!("After a_low_biguint.carrying_mul_assign_uint(225_u8, 0),\na_low_biguint = {}", a_low_biguint);
    assert_eq!(a_low_biguint.to_string(), "17280421717087553271230257168091959361442094623632687978237947571026368921150");
    assert_eq!(a_low_biguint.is_overflow(), false);
    assert_eq!(a_low_biguint.is_underflow(), false);
    assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    assert_eq!(a_low_biguint.is_infinity(), false);
    assert_eq!(a_low_biguint.is_undefined(), false);
    assert_eq!(a_low_biguint.is_left_carry(), false);
    assert_eq!(a_low_biguint.is_right_carry(), false);

    let res_higher = a_high_biguint.carrying_mul_assign_uint(b_uint, res_high);
    println!("After a_high_biguint.carrying_mul_assign_uint(225_u8, res_higher),\na_high_biguint = {}\nres_higher = {}", a_high_biguint, res_higher);
    assert_eq!(a_high_biguint.to_string(), "16962363268797823794757102636892132280421717087553271230257168091959361441925");
    assert_eq!(res_higher.to_string(), "0");
    assert_eq!(res_higher.is_overflow(), false);
    assert_eq!(res_higher.is_underflow(), false);
    assert_eq!(res_higher.is_divided_by_zero(), false);
    assert_eq!(res_higher.is_infinity(), false);
    assert_eq!(res_higher.is_undefined(), false);
    assert_eq!(res_higher.is_left_carry(), false);
    assert_eq!(res_higher.is_right_carry(), false);

    assert_eq!(a_high_biguint.is_overflow(), false);
    assert_eq!(a_high_biguint.is_underflow(), false);
    assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    assert_eq!(a_high_biguint.is_infinity(), false);
    assert_eq!(a_high_biguint.is_undefined(), false);
    assert_eq!(a_high_biguint.is_left_carry(), false);
    assert_eq!(a_high_biguint.is_right_carry(), false);

    // Maximum case
    let mut a_low_biguint = U256::max();
    let mut a_high_biguint = UU32::max();
    let b_uint = u64::MAX;

    println!("Originally, a_low_biguint = {}", a_low_biguint);
    assert_eq!(a_low_biguint.is_overflow(), false);
    assert_eq!(a_low_biguint.is_underflow(), false);
    assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    assert_eq!(a_low_biguint.is_infinity(), false);
    assert_eq!(a_low_biguint.is_undefined(), false);
    assert_eq!(a_low_biguint.is_left_carry(), false);
    assert_eq!(a_low_biguint.is_right_carry(), false);

    println!("Originally, a_high_biguint = {}", a_high_biguint);
    assert_eq!(a_high_biguint.is_overflow(), false);
    assert_eq!(a_high_biguint.is_underflow(), false);
    assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    assert_eq!(a_high_biguint.is_infinity(), false);
    assert_eq!(a_high_biguint.is_undefined(), false);
    assert_eq!(a_high_biguint.is_left_carry(), false);
    assert_eq!(a_high_biguint.is_right_carry(), false);

    let res_high = a_low_biguint.carrying_mul_assign_uint(b_uint, UU32::zero());
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    println!("After a_low_biguint.carrying_mul_assign_uint(u64:MAX, 0),\na_low_biguint = {}", a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_low_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    assert_eq!(a_low_biguint.is_overflow(), true);
    assert_eq!(a_low_biguint.is_underflow(), false);
    assert_eq!(a_low_biguint.is_divided_by_zero(), false);
    assert_eq!(a_low_biguint.is_infinity(), false);
    assert_eq!(a_low_biguint.is_undefined(), false);
    assert_eq!(a_low_biguint.is_left_carry(), false);
    assert_eq!(a_low_biguint.is_right_carry(), false);

    let res_higher = a_high_biguint.carrying_mul_assign_uint(b_uint, res_high);
    println!("After a_high_biguint.carrying_mul_assign_uint(u64:MAX, res_higher),\na_high_biguint = {}\nres_higher = {}", a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_higher.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(a_high_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000040");
    assert_eq!(a_high_biguint.is_overflow(), true);
    assert_eq!(a_high_biguint.is_underflow(), false);
    assert_eq!(a_high_biguint.is_divided_by_zero(), false);
    assert_eq!(a_high_biguint.is_infinity(), false);
    assert_eq!(a_high_biguint.is_left_carry(), false);
    assert_eq!(a_high_biguint.is_right_carry(), false);
    
    assert_eq!(res_higher.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    assert_eq!(res_higher.is_overflow(), false);
    assert_eq!(res_higher.is_underflow(), false);
    assert_eq!(res_higher.is_divided_by_zero(), false);
    assert_eq!(res_higher.is_infinity(), false);
    assert_eq!(res_higher.is_left_carry(), false);
    assert_eq!(res_higher.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_widening_mul_uint()
{
    println!("biguint_widening_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case
    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u128;
    let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    println!("{} X {} = {}:{}", a_biguint, b_uint, res_high, res_low);
    assert_eq!(res_high.to_string(), "1");
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    assert_eq!(res_low.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res_low.is_overflow(), true);
    assert_eq!(res_low.is_underflow(), false);
    assert_eq!(res_low.is_divided_by_zero(), false);
    assert_eq!(res_low.is_infinity(), false);
    assert_eq!(res_low.is_undefined(), false);
    assert_eq!(res_low.is_left_carry(), false);
    assert_eq!(res_low.is_right_carry(), false);

    // Maximum case
    let a_biguint = U256::max();
    let b_uint = u128::MAX;
    let (res_low, res_high) = a_biguint.widening_mul_uint(b_uint);
    println!("{} X {:X} = {}:{}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), b_uint, res_high.to_string_with_radix_and_stride(16, 8).unwrap(), res_low.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "7F");
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    assert_eq!(res_low.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000000_00000000_00000001");
    assert_eq!(res_low.is_overflow(), true);
    assert_eq!(res_low.is_underflow(), false);
    assert_eq!(res_low.is_divided_by_zero(), false);
    assert_eq!(res_low.is_infinity(), false);
    assert_eq!(res_low.is_undefined(), false);
    assert_eq!(res_low.is_left_carry(), false);
    assert_eq!(res_low.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_widening_mul_assign_uint()
{
    println!("biguint_widening_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case
    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u64;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let res_high = a_biguint.widening_mul_assign_uint(b_uint);
    println!("After a_biguint.widening_mul_assign_uint(248_u8),\na_biguint = {}\nres_high = {}", a_biguint, res_high);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(res_high.to_string(), "1");
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);
    
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Maximum case
    let mut a_biguint = UU32::max();
    let b_uint = u64::MAX;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let res_high = a_biguint.widening_mul_assign_uint(b_uint);
    println!("After a_biguint.widening_mul_assign_uint(u64::MAX),\na_biguint = {}\nres_high = {}", a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), res_high.to_string_with_radix_and_stride(16, 8).unwrap());
    assert_eq!(res_high.to_string_with_radix_and_stride(16, 8).unwrap(), "3F");
    assert_eq!(res_high.is_overflow(), false);
    assert_eq!(res_high.is_underflow(), false);
    assert_eq!(res_high.is_divided_by_zero(), false);
    assert_eq!(res_high.is_infinity(), false);
    assert_eq!(res_high.is_undefined(), false);
    assert_eq!(res_high.is_left_carry(), false);
    assert_eq!(res_high.is_right_carry(), false);

    assert_eq!(a_biguint.to_string_with_radix_and_stride(16, 8).unwrap(), "FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_00000000_00000001");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_mul_uint()
{
    println!("biguint_wrapping_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_uint = 248_u16;
    let res = a_biguint.wrapping_mul_uint(b_uint);
    println!("{} X {} = {}", a_biguint, b_uint, res);
    assert_eq!(res.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let res = b_biguint.wrapping_mul_uint(b_uint);
    println!("{} X {} = {}", b_biguint, b_uint, res);
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

fn biguint_wrapping_mul_assign_uint()
{
    println!("biguint_wrapping_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_uint = 248_u16;

    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "12380187429816690342769003185818648605085375388281194656994643364900608");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_mul_assign_uint(b_uint);
    println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "3070286482594539205006712790083024854061173096293736274934671554495350784");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_mul_assign_uint(b_uint);
    println!("After a_biguint.wrapping_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
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

fn biguint_overflowing_mul_uint()
{
    println!("biguint_overflowing_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u8;
    let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    assert_eq!(res.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    assert_eq!(overflow, false);
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let (res, overflow) = a_biguint.overflowing_mul_uint(b_uint);
    println!("{} X {} = {}, {}", a_biguint, b_uint, res, overflow);
    assert_eq!(res.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(overflow, true);
    assert_eq!(res.is_overflow(), true);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_overflowing_mul_assign_uint()
{
    println!("biguint_overflowing_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU32::from_string("1874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let b_uint = 248_u128;

    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "464825945392050067127900830248540611730962937362749346715544953508855312");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_mul_assign_uint(b_uint);
    println!("After a_biguint.overflowing_mul_assign_uint(248_u16), a_biguint = {}, {}", a_biguint, overflow);
    assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
    assert_eq!(overflow, true);
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_checked_mul_uint()
{
    println!("biguint_checked_mul_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut b_uint = 248_u16;
    let res = a_biguint.checked_mul_uint(b_uint);
    match &res
    {
        Some(r) => { println!("{} X {} = {}", a_biguint, b_uint, r); },
        None => {
            println!("Overflow happend!");
            assert_eq!(res, None);
        },
    }

    b_uint = 5_u16;
    let res = a_biguint.checked_mul_uint(b_uint);
    match &res
    {
        Some(r) => {
            println!("{} X {} = {}", a_biguint, b_uint, r);
            assert_eq!(r.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
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
    println!("---------------------------");
}

fn biguint_unchecked_mul_uint()
{
    println!("biguint_unchecked_mul_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let res = a_biguint.unchecked_mul_uint(5_u8);
    println!("{} X {} = {}", a_biguint, 5_u8, res);
    assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_unchecked_mul_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_unchecked_mul_uint()
{
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let _res = _a_biguint.unchecked_mul_uint(248_u8);
}

fn biguint_saturating_mul_uint()
{
    println!("biguint_saturating_mul_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let res = a_biguint.saturating_mul_uint(5_u8);
    println!("{} X {} = {}", a_biguint, 5_u8, res);
    assert_eq!(res.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let res = a_biguint.saturating_mul_uint(248_u8);
    println!("{} X {} = {}", a_biguint, 248_u8, res);
    assert_eq!(res.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
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

fn biguint_saturating_mul_assign_uint()
{
    println!("biguint_saturating_mul_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_mul_assign_uint(5_u8);
    println!("After a_biguint.saturating_mul_assign_uint(5_u8), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "4384009371490834517138450159290932430254268769414059732849732168245030420470");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_mul_assign_uint(248_u8);
    println!("After a_biguint.saturating_mul_assign_uint(248_u8), a_biguint = {}", a_biguint);
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

fn biguint_modular_mul_uint()
{
    println!("biguint_modular_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let mul_uint = 5_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let mul_uint = 248_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
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
    let mul_uint = 5_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 = multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_uint(4321000_u32);
    let mul_uint = 5_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_mul_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_mul_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // modulo == 0
    let _m = UU32::zero();
    let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _mul_uint = 248_u8;
    let _res = _a_biguint.modular_mul_uint(_mul_uint, &_m);

    // modulo == 1
    let _m = UU32::one();
    let _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _mul_uint = 248_u8;
    let _res = _a_biguint.modular_mul_uint(_mul_uint, &_m);
}

fn biguint_modular_mul_assign_uint()
{
    println!("biguint_modular_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 5_u16;
    a_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 248_u16;
    a_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 3
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 248_u16;
    a_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 2_u16;
    a_biguint.modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "35149539482914268500351723679771158582906673069252814597151206317181518258");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == multiple of modulo
    let mut a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op2 == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op2 == multiple of modulo
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "150");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_mul_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_mul_assign_uint()
{
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _m = UU32::zero();
    let _mul_uint = 248_u8;
    _a_biguint.modular_mul_assign_uint(_mul_uint, &_m);

    let mut _a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let _m = UU32::one();
    let _mul_uint = 248_u8;
    _a_biguint.modular_mul_assign_uint(_mul_uint, &_m);
}

fn biguint_panic_free_modular_mul_uint()
{
    println!("biguint_panic_free_modular_mul_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case 1
    let a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_uint = 5_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Normal case 2
    let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_uint = 248_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
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
    let mul_uint = 5_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // op1 = multiple of modulo
    let m = UU32::from_uint(1000_u16);
    let a_biguint = U256::from_uint(4321000_u32);
    let mul_uint = 5_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 0_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
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
    let mul_uint = 4321000_u32;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 0
    let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::zero();
    let mul_uint = 248_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // modulo == 1
    let a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::one();
    let mul_uint = 248_u8;
    let res = a_biguint.panic_free_modular_mul_uint(mul_uint, &m);
    println!("{} * {} = {} (mod {})", a_biguint, mul_uint, res, m);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), true);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // collectively
    for a_biguint in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let res = a_biguint.panic_free_modular_mul_uint(rhs, &m);
                println!("{} * {} = {} (mod {})", a_biguint, rhs, res, m);
                assert_eq!(res.to_string(), "0");
                assert_eq!(res.is_overflow(), false);
                assert_eq!(res.is_underflow(), false);
                assert_eq!(res.is_divided_by_zero(), false);
                assert_eq!(res.is_infinity(), false);
                assert_eq!(res.is_undefined(), true);
                assert_eq!(res.is_left_carry(), false);
                assert_eq!(res.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_modular_mul_assign_uint()
{
    println!("biguint_panic_free_modular_mul_assign_uint");
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let mut a_biguint = U256::from_string("31858186486050853753882811946768018742981669034276900586487291375468285").unwrap();
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_uint = 5_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "159290932430254268769414059733840093714908345171384502932436456877341425");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mul_uint = 248_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 3
    let m = UU32::from_string("76801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 248_u16;
    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "55975706890540585964020877768978822316880213476032380583548819983093801176");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mul_uint = 2_u16;
    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "35149539482914268500351723679771158582906673069252814597151206317181518258");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == 0
    let mut a_biguint = U256::zero();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op1 == multiple of modulo
    let mut a_biguint = U256::from_uint(750_u16);
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 3_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op2 == 0
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // op2 == multiple of modulo
    let mut a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "76801874298166903427690031858186486050853753882811946569946433649006");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(250_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "750");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 0_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
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
    let m = U256::from_uint(50_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "150");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let rhs = 250_u8;
    a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &m), a_biguint = {}", rhs, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::zero();
    let mul_uint = 248_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1
    let mut a_biguint = U256::from_string("318581864860508537538828119467680187429816690342769005864872913754682855846").unwrap();
    let m = UU32::one();
    let mul_uint = 248_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m);
    println!("After a_biguint.panic_free_modular_mul_assign_uint(mul_uint, &m), a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // collectively
    for a in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::zero(), U256::from_uint(50_u8)]
    {
        for rhs in [0_u8, 3_u8, 50_u8]
        {
            for m in [U256::zero(), U256::one()]
            {
                let mut a_biguint = a.clone();
                println!("Originally, a = {}", a_biguint);
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), false);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            
                a_biguint.panic_free_modular_mul_assign_uint(rhs, &m);
                println!("After a_biguint.panic_free_modular_mul_assign_uint({}, &{}), a_biguint = {}", rhs, a_biguint, m);
                assert_eq!(a_biguint.to_string(), "0");
                assert_eq!(a_biguint.is_overflow(), false);
                assert_eq!(a_biguint.is_underflow(), false);
                assert_eq!(a_biguint.is_divided_by_zero(), false);
                assert_eq!(a_biguint.is_infinity(), false);
                assert_eq!(a_biguint.is_undefined(), true);
                assert_eq!(a_biguint.is_left_carry(), false);
                assert_eq!(a_biguint.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_safe_mul_uint()
{
    println!("biguint_safe_mul_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let a_biguint = U256::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    let b_uint = 248_u16;
    let res = a_biguint.safe_mul_uint(b_uint);
    println!("{} X {} = {}", a_biguint, b_uint, res);
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
        let b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let b_uint = 248_u16;
        let res = b_biguint.safe_mul_uint(b_uint);
        println!("{} X {} = {}", b_biguint, b_uint, res);
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
    biguint_should_panic_safe_mul_3uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_mul_3uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u16);
    
        let _b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_uint = 248_u16;
        let _res = _b_biguint.safe_mul_uint(_b_uint);
    }
}

fn biguint_safe_mul_assign_uint()
{
    println!("biguint_safe_mul_assign_uint");
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = UU32::from_string("12380187429816690342769003185818648605085375388281194656994643364900608").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "12380187429816690342769003185818648605085375388281194656994643364900608");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_uint = 248_u16;
    a_biguint.safe_mul_assign_uint(b_uint);
    println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
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
        assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let b_uint = 248_u16;
        a_biguint.safe_mul_assign_uint(b_uint);
        println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
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
    biguint_should_panic_safe_mul_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_safe_mul_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
        define_utypes_with!(u32);

        let mut a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        println!("Originally, a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "876801874298166903427690031858186486050853753882811946569946433649006084094");
        assert_eq!(a_biguint.is_overflow(), false);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);

        let b_uint = 248_u16;
        // It will panic.
        a_biguint.safe_mul_assign_uint(b_uint);
        println!("After a_biguint.safe_mul_assign_uint(248_u16), a_biguint = {}", a_biguint);
        assert_eq!(a_biguint.to_string(), "101654775588629196626496142892142340687341746297296798709889131537040379215376");
        assert_eq!(a_biguint.is_overflow(), true);
        assert_eq!(a_biguint.is_underflow(), false);
        assert_eq!(a_biguint.is_divided_by_zero(), false);
        assert_eq!(a_biguint.is_infinity(), false);
        assert_eq!(a_biguint.is_undefined(), false);
        assert_eq!(a_biguint.is_left_carry(), false);
        assert_eq!(a_biguint.is_right_carry(), false);
    }
}


fn biguint_div_uint()
{
    biguint_divide_fully_uint();
    biguint_panic_free_divide_fully_uint();
    biguint_wrapping_div_uint();
    biguint_wrapping_div_assign_uint();
    biguint_overflowing_div_uint();
    biguint_overflowing_div_assign_uint();
    biguint_checked_div_uint();
    biguint_unchecked_div_uint();
    biguint_saturating_div_uint();
    biguint_saturating_div_assign_uint();
    biguint_panic_free_div_uint();
    biguint_panic_free_div_assign_uint();
    biguint_modular_div_uint();
    biguint_modular_div_assign_uint();
    biguint_panic_free_modular_div_uint();
    biguint_panic_free_modular_div_assign_uint();
}

fn biguint_divide_fully_uint()
{
    println!("biguint_divide_fully_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let (quotient, remainder) = dividend.divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    let dividend = UU32::zero();
    let divisor = 87_u8;
    let (quotient, remainder) = dividend.divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_divide_fully_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_divide_fully_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let (_quotient, _remainder) = _dividend.divide_fully_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let (_quotient, _remainder) = _dividend.divide_fully_uint(_divisor);
}

fn biguint_panic_free_divide_fully_uint()
{
    println!("biguint_panic_free_divide_fully_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case 1
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // Normal case 2
    let dividend = UU32::zero();
    let divisor = 87_u8;
    let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // dividend != 0 and divisor == 0
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient, UU32::max());
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), true);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // dividend == 0 and divisor == 0
    let dividend = UU32::zero();
    let divisor = 0_u8;
    let (quotient, remainder) = dividend.panic_free_divide_fully_uint(divisor);
    println!("{} / {} => quotient = {} , remainder = {}", dividend, divisor, quotient, remainder);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_wrapping_div_uint()
{
    println!("biguint_wrapping_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let quotient = dividend.wrapping_div_uint(divisor);
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
    let divisor = 87_u8;
    let quotient = dividend.wrapping_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_wrapping_div_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_wrapping_div_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _quotient = _dividend.wrapping_div_uint(_divisor);

    let _dividend = U256::zero();
    let _divisor = 0_u8;
    let _quotient = _dividend.wrapping_div_uint(_divisor);
}

fn biguint_wrapping_div_assign_uint()
{
    println!("biguint_wrapping_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_div_assign_uint(divisor);
    println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.wrapping_div_assign_uint(divisor);
    println!("After a_biguint.wrapping_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_wrapping_div_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_wrapping_div_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    _a_biguint.wrapping_div_assign_uint(_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = 0_u8;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    _a_biguint.wrapping_div_assign_uint(_divisor);
}

fn biguint_overflowing_div_uint()
{
    println!("biguint_overflowing_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let (quotient, overflow) = dividend.overflowing_div_uint(divisor);
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
    let divisor = 87_u8;
    let (quotient, overflow) = dividend.overflowing_div_uint(divisor);
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

    #[cfg(test)] // It will panic.
    biguint_should_panic_overflowing_div_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_overflowing_div_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let (_quotient, _overflow) = _dividend.overflowing_div_uint(_divisor);

    let _dividend = U256::zero();
    let _divisor = 0_u8;
    let (_quotient, _overflow) = _dividend.overflowing_div_uint(_divisor);
}

fn biguint_overflowing_div_assign_uint()
{
    println!("biguint_overflowing_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let overflow = a_biguint.overflowing_div_assign_uint(divisor);
    println!("After a_biguint.overflowing_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let overflow = a_biguint.overflowing_div_assign_uint(divisor);
    println!("After a_biguint.overflowing_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_overflowing_div_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_overflowing_div_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _overflow = _a_biguint.overflowing_div_assign_uint(_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = 0_u8;
    let _overflow = _a_biguint.overflowing_div_assign_uint(_divisor);
}

fn biguint_checked_div_uint()
{
    println!("biguint_checked_div_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let quotient = dividend.checked_div_uint(divisor);
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
    let divisor = 87_u8;
    let quotient = dividend.checked_div_uint(divisor);
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
    let divisor = 0_u8;
    let quotient = dividend.checked_div_uint(divisor);
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
    let divisor = 0_u8;
    let quotient = dividend.checked_div_uint(divisor);
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

fn biguint_unchecked_div_uint()
{
    println!("biguint_unchecked_div_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let quotient = dividend.unchecked_div_uint(divisor);
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
    let divisor = 87_u8;
    let quotient = dividend.unchecked_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_unchecked_div_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_unchecked_div_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _quotient = _dividend.unchecked_div_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _quotient = _dividend.unchecked_div_uint(_divisor);
}

fn biguint_saturating_div_uint()
{
    println!("biguint_saturating_div_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let quotient = dividend.saturating_div_uint(divisor);
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
    let divisor = 87_u8;
    let quotient = dividend.saturating_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_saturating_div_uint();
    println!("---------------------------")
}

#[test]
#[should_panic]
fn biguint_should_panic_saturating_div_uint()
{
    println!("biguint_saturating_div_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _quotient = _dividend.saturating_div_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _quotient = _dividend.saturating_div_uint(_divisor);
    println!("---------------------------")
}

fn biguint_saturating_div_assign_uint()
{
    println!("biguint_saturating_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_div_assign_uint(divisor);
    println!("After a_biguint.saturating_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_div_assign_uint(divisor);
    println!("After a_biguint.saturating_div_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_saturating_div_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_saturating_div_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let mut _a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    println!("Originally, _a_biguint = {}", _a_biguint);
    _a_biguint.saturating_div_assign_uint(_divisor);

    let mut _a_biguint = UU32::zero();
    let _divisor = 0_u8;
    println!("Originally, _a_biguint = {}", _a_biguint);
    _a_biguint.saturating_div_assign_uint(_divisor);
    println!("---------------------------");
}


fn biguint_panic_free_div_uint()
{
    println!("biguint_panic_free_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let quotient = dividend.panic_free_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // Normal case 2
    let dividend = U256::zero();
    let divisor = 87_u8;
    let quotient = dividend.panic_free_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // dividend != 0 and divisor = 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let quotient = dividend.panic_free_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient, U256::max());
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), true);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // dividend == 0 and divisor = 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let quotient = dividend.panic_free_div_uint(divisor);
    println!("{} / {} = {}", dividend, divisor, quotient);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_div_assign_uint()
{
    println!("biguint_panic_free_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 87_u8;
    a_biguint.panic_free_div_assign_uint(divisor);
    println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "1419043551905275201680884938348044216837079832");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 87_u8;
    a_biguint.panic_free_div_assign_uint(divisor);
    println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    a_biguint.panic_free_div_assign_uint(divisor);
    println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint, UU32::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    a_biguint.panic_free_div_assign_uint(divisor);
    println!("After a_biguint.panic_free_div_assign_uint(&divisor),\na_biguint = {}", a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_div_uint()
{
    println!("biguint_modular_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.modular_div_uint(divisor, &modulo);
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
    let divisor = 3_u8;
    let res = dividend.modular_div_uint(divisor, &modulo);
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
    let divisor = 3_u8;
    let res = dividend.modular_div_uint(divisor, &modulo);
    println!("{} / {} = {}(mod {})", dividend, divisor, res, modulo);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_div_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_div_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // op2 == 0
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // op2 == multiple of modulo
    let _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // op1 == 0 and op2 == 0
    let _a_biguint = U256::zero();
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // op1 == multiple of modulo and op2 == 0
    let _a_biguint = U256::from_uint(750_u16);
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // op1 == 0 and op2 == multiple of modulo
    let _a_biguint = U256::zero();
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let _a_biguint = U256::from_uint(150_u8);
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // modulo == 0
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::zero();
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);

    // modulo == 1
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::one();
    let _res = _a_biguint.modular_div_uint(_rhs, &_m);
}

fn biguint_modular_div_assign_uint()
{
    println!("biguint_modular_div_assign_uint");
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

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_div_assign_uint({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
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

    let divisor = 3_u8;
    let modulo = U256::from_uint(250_u8);
    a_biguint.modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_div_assign_uint({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
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
    let divisor = 3_u8;
    a_biguint.modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_div_assign_uint({}, {}),\na_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_div_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_div_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // op2 == 0
    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // op2 == multiple of modulo
    let mut _a_biguint = U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // op1 == 0 and op2 == 0
    let mut _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // op1 == multiple of modulo and op2 == 0
    let mut _a_biguint = U256::from_uint(750_u16);
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(250_u8);
    let _rhs = 0_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // op1 == 0 and op2 == multiple of modulo
    let mut _a_biguint = U256::zero();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // op1 == multiple of modulo and op2 == multiple of modulo
    let mut _a_biguint = U256::from_uint(150_u8);
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _m = U256::from_uint(50_u8);
    let _rhs = 250_u8;
    _a_biguint.modular_div_assign_uint(_rhs, &_m);

    // modulo == 0
    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _divisor = 128_u8;
    let _modulo = U256::zero();
    _a_biguint.modular_div_assign_uint(_divisor, &_modulo);

    // modulo == 1
    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _divisor = 128_u8;
    let _modulo = U256::one();
    _a_biguint.modular_div_assign_uint(_divisor, &_modulo);
}

fn biguint_panic_free_modular_div_uint()
{
    println!("biguint_panic_free_modular_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "3");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // Normal case 2 for modulo >= 2 and dividend == 0 and divisor != 0
    let dividend = U256::zero();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // Normal case 3 for modulo >= 2 and dividend == multiple of modulo and divisor != 0
    let dividend = U256::from_uint(10000_u16);
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == 0 and dividend != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient, U256::max());
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), true);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == multiple of modulo and dividend != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient, U256::max());
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), true);
    assert_eq!(quotient.is_undefined(), false);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == 0 and dividend == multiple of modulo
    let dividend = U256::from_uint(30000_u16);
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == multiple of modulo and dividend == 0
    let dividend = U256::zero();
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo >= 2 and divisor == multiple of modulo and dividend == multiple of modulo
    let dividend = U256::from_uint(30000_u16);
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo == 0 and divisor != 0 and dividend != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::zero();
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo == 1 and divisor != 0 and dividend != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::one();
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), false);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo == 0 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::zero();
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // modulo == 1 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::one();
    let quotient = dividend.panic_free_modular_div_uint(divisor, &modulo);
    println!("{} / {} = {} (mod {})", dividend, divisor, quotient, modulo);
    assert_eq!(quotient.to_string(), "0");
    assert_eq!(quotient.is_overflow(), false);
    assert_eq!(quotient.is_underflow(), false);
    assert_eq!(quotient.is_infinity(), false);
    assert_eq!(quotient.is_undefined(), true);
    assert_eq!(quotient.is_divided_by_zero(), true);
    assert_eq!(quotient.is_left_carry(), false);
    assert_eq!(quotient.is_right_carry(), false);

    // collectively
    for modulo in [U256::zero(), U256::one()]
    {
        let op1 = U256::zero();
        let op2 = 0_u8;
        let res = op1.panic_free_modular_div_uint(op2, &modulo);
        println!("{} / {} = {} (mod {})", op1, op2, res, modulo);
        assert_eq!(res.to_string(), "0");
        assert_eq!(res.is_overflow(), false);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_divided_by_zero(), true);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), true);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
        
        for dividend in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
        {
            let rhs = 0_u8;
            let res = dividend.panic_free_modular_div_uint(rhs, &modulo);
            println!("{} / {} = {} (mod {})", dividend, rhs, res, modulo);
            assert_eq!(res.to_string(), "0");
            assert_eq!(res.is_overflow(), false);
            assert_eq!(res.is_underflow(), false);
            assert_eq!(res.is_divided_by_zero(), true);
            assert_eq!(res.is_infinity(), true);
            assert_eq!(res.is_undefined(), true);
            assert_eq!(res.is_left_carry(), false);
            assert_eq!(res.is_right_carry(), false);

            for divisor in [3_u8, 50_u8]
            {
                let res = dividend.panic_free_modular_div_uint(divisor, &modulo);
                println!("{} / {} = {} (mod {})", dividend, divisor, res, modulo);
                assert_eq!(res.to_string(), "0");
                assert_eq!(res.is_overflow(), false);
                assert_eq!(res.is_underflow(), false);
                assert_eq!(res.is_divided_by_zero(), false);
                assert_eq!(res.is_infinity(), false);
                assert_eq!(res.is_undefined(), true);
                assert_eq!(res.is_left_carry(), false);
                assert_eq!(res.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_modular_div_assign_uint()
{
    println!("biguint_panic_free_modular_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // Normal case 1
    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "3");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2 for modulo >= 2 and self == 0 and divisor != 0
    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 3 for modulo >= 2 and self == multiple of modulo and divisor != 0
    let mut a_biguint = U256::from_uint(10000_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self != 0 and divisor == 0
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self != 0 and divisor == multiple of modulo
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), true);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == 0 and divisor == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == 0 and divisor == multiple of modulo
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == multiple of modulo and divisor == 0
    let mut a_biguint = U256::from_uint(30000_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == multiple of modulo and divisor == multiple of modulo
    let mut a_biguint = U256::from_uint(30000_u16);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0 and divisor != 0 and dividend != 0
    let mut a_biguint = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = U256::zero();
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1 and divisor != 0 and dividend != 0
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = U256::one();
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0 and divisor == 0 and dividend == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::zero();
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1 and divisor == 0 and dividend == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::one();
    a_biguint.panic_free_modular_div_assign_uint(divisor, &modulo);
    println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Collectively
    for modulo in [U256::zero(), U256::one()]
    {
        let mut dividend = U256::zero();
        println!("Originally, op1 = {}", dividend);
        assert_eq!(dividend.is_overflow(), false);
        assert_eq!(dividend.is_underflow(), false);
        assert_eq!(dividend.is_infinity(), false);
        assert_eq!(dividend.is_undefined(), false);
        assert_eq!(dividend.is_divided_by_zero(), false);
        assert_eq!(dividend.is_left_carry(), false);
        assert_eq!(dividend.is_right_carry(), false);

        let divisor = 0_u8;
        dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
        println!("After a_biguint.panic_free_modular_div_assign_uint({}, {}), op1 = {}", divisor, modulo, dividend);
        assert_eq!(dividend.to_string(), "0");
        assert_eq!(dividend.is_overflow(), false);
        assert_eq!(dividend.is_underflow(), false);
        assert_eq!(dividend.is_divided_by_zero(), true);
        assert_eq!(dividend.is_infinity(), false);
        assert_eq!(dividend.is_undefined(), true);
        assert_eq!(dividend.is_left_carry(), false);
        assert_eq!(dividend.is_right_carry(), false);
        
        for op in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
        {
            let mut dividend = op.clone();
            println!("Originally, dividend = {}", dividend);
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_infinity(), false);
            assert_eq!(dividend.is_undefined(), false);
            assert_eq!(dividend.is_divided_by_zero(), false);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            let divisor = 0_u8;
            dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
            println!("After op1.panic_free_modular_div_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
            assert_eq!(dividend.to_string(), "0");
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_divided_by_zero(), true);
            assert_eq!(dividend.is_infinity(), true);
            assert_eq!(dividend.is_undefined(), true);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            for divisor in [3_u8, 50_u8]
            {
                let mut dividend = op.clone();
                println!("Originally, dividend = {}", dividend);
                assert_eq!(dividend.is_overflow(), false);
                assert_eq!(dividend.is_underflow(), false);
                assert_eq!(dividend.is_infinity(), false);
                assert_eq!(dividend.is_undefined(), false);
                assert_eq!(dividend.is_divided_by_zero(), false);
                assert_eq!(dividend.is_left_carry(), false);
                assert_eq!(dividend.is_right_carry(), false);
    
                dividend.panic_free_modular_div_assign_uint(divisor, &modulo);
                println!("After dividend.panic_free_modular_div_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
                assert_eq!(dividend.to_string(), "0");
                assert_eq!(dividend.is_overflow(), false);
                assert_eq!(dividend.is_underflow(), false);
                assert_eq!(dividend.is_divided_by_zero(), false);
                assert_eq!(dividend.is_infinity(), false);
                assert_eq!(dividend.is_undefined(), true);
                assert_eq!(dividend.is_left_carry(), false);
                assert_eq!(dividend.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_rem_uint()
{
    biguint_wrapping_rem_uint();
    biguint_wrapping_rem_assign_uint();
    biguint_overflowing_rem_uint();
    biguint_overflowing_rem_assign_uint();
    biguint_checked_rem_uint();
    biguint_unchecked_rem_uint();
    biguint_saturating_rem_uint();
    biguint_saturating_rem_assign_uint();
    biguint_panic_free_rem_uint();
    biguint_panic_free_rem_assign_uint();
    biguint_modular_rem_uint();
    biguint_modular_rem_assign_uint();
    biguint_panic_free_modular_rem_uint();
    biguint_panic_free_modular_rem_assign_uint();
}

fn biguint_wrapping_rem_uint()
{
    println!("biguint_wrapping_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let remainder = dividend.wrapping_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    let dividend = UU32::zero();
    let divisor = 87_u8;
    let remainder = dividend.wrapping_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_should_panic_wrapping_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_wrapping_rem_uint()
{
    println!("biguint_wrapping_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _remainder = _dividend.wrapping_rem_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _remainder = _dividend.wrapping_rem_uint(_divisor);
}

fn biguint_wrapping_rem_assign_uint()
{
    println!("biguint_wrapping_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 87_u8;
    a_biguint.wrapping_rem_assign_uint(divisor);
    println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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

    let divisor = 87_u8;
    a_biguint.wrapping_rem_assign_uint(divisor);
    println!("After a_biguint.wrapping_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_wrapping_rem_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_wrapping_rem_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    println!("Originally, a_biguint = {}", _a_biguint);
    _a_biguint.wrapping_rem_assign_uint(_divisor);

    let mut _a_biguint = U256::zero();
    let _divisor = 0_u8;
    println!("Originally, a_biguint = {}", _a_biguint);
    _a_biguint.wrapping_rem_assign_uint(_divisor);
}

fn biguint_overflowing_rem_uint()
{
    println!("biguint_overflowing_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 8);
    assert_eq!(overflow, false);

    let dividend = UU32::zero();
    let divisor = 87_u8;
    let (remainder, overflow) = dividend.overflowing_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder, 0);
    assert_eq!(overflow, false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_overflowing_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_overflowing_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let (_remainder, _overflow) = _dividend.overflowing_rem_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let (_remainder, _overflow) = _dividend.overflowing_rem_uint(_divisor);
}

fn biguint_overflowing_rem_assign_uint()
{
    println!("biguint_overflowing_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u16;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    println!("After a_biguint.overflowing_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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
    let divisor = 87_u16;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let overflow = a_biguint.overflowing_rem_assign_uint(divisor);
    println!("After a_biguint.overflowing_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(overflow, false);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_overflowing_rem_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_overflowing_rem_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u16;
    println!("Originally, a_biguint = {}", _a_biguint);
    let _overflow = _a_biguint.overflowing_rem_assign_uint(_divisor);

    let mut _a_biguint = U256::zero();
    let _divisor = 0_u16;
    println!("Originally, a_biguint = {}", _a_biguint);
    let _overflow = _a_biguint.overflowing_rem_assign_uint(_divisor);
}

fn biguint_checked_rem_uint()
{
    println!("biguint_checked_rem_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let remainder = dividend.checked_rem_uint(divisor);
    match remainder
    {
        Some(r) =>
            {
                println!("{} % {} = {}", dividend, divisor, r);
                assert_eq!(r.to_string(), "8");
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = UU32::zero();
    let divisor = 87_u8;
    let remainder = dividend.checked_rem_uint(divisor);
    match remainder
    {
        Some(r) =>
            {
                println!("{} % {} = {}", dividend, divisor, r);
                assert_eq!(r.to_string(), "0");
            },
        None => { println!("Divided By Zero"); },
    }

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let remainder = dividend.checked_rem_uint(divisor);
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
    let divisor = 0_u8;
    let remainder = dividend.checked_rem_uint(divisor);
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

fn biguint_unchecked_rem_uint()
{
    println!("biguint_unchecked_rem_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let remainder = dividend.unchecked_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    let dividend = U256::zero();
    let divisor = 87_u8;
    let remainder = dividend.unchecked_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_should_panic_unchecked_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_unchecked_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _remainder = _dividend.unchecked_rem_uint(_divisor);

    let _dividend = U256::zero();
    let _divisor = 0_u8;
    let _remainder = _dividend.unchecked_rem_uint(_divisor);
}

fn biguint_saturating_rem_uint()
{
    println!("biguint_saturating_rem_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let remainder = dividend.saturating_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");

    let dividend = UU32::zero();
    let divisor = 87_u8;
    let remainder = dividend.saturating_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_should_panic_saturating_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_saturating_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);
    
    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _remainder = _dividend.saturating_rem_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _remainder = _dividend.saturating_rem_uint(_divisor);
}

fn biguint_saturating_rem_assign_uint()
{
    println!("biguint_saturating_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u16;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_rem_assign_uint(divisor);
    println!("After a_biguint.saturating_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = U256::zero();
    let divisor = 87_u16;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.saturating_rem_assign_uint(divisor);
    println!("After a_biguint.saturating_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_saturating_rem_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_saturating_rem_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u16;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    _a_biguint.saturating_rem_assign_uint(_divisor);

    let mut _a_biguint = U256::zero();
    let _divisor = 0_u16;
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    _a_biguint.saturating_rem_assign_uint(_divisor);
}

fn biguint_panic_free_rem_uint()
{
    println!("biguint_panic_free_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    // Normal case 1
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    let remainder = dividend.panic_free_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // Normal case 2
    let dividend = UU32::zero();
    let divisor = 87_u8;
    let remainder = dividend.panic_free_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // dividend != 0 and divisor == 0
    let dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let remainder = dividend.panic_free_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);
    
    // dividend == 0 and divisor == 0
    let dividend = UU32::zero();
    let divisor = 0_u8;
    let remainder = dividend.panic_free_rem_uint(divisor);
    println!("{} % {} = {}", dividend, divisor, remainder);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_rem_assign_uint()
{
    println!("biguint_panic_free_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    // Normal case 1
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_rem_assign_uint(divisor);
    println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = U256::zero();
    let divisor = 87_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_rem_assign_uint(divisor);
    println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // dividend != 0 and divisor == 0
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let divisor = 0_u8;
    a_biguint.panic_free_rem_assign_uint(divisor);
    println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // dividend == 0 and divisor == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    
    let divisor = 0_u8;
    a_biguint.panic_free_rem_assign_uint(divisor);
    println!("After a_biguint.panic_free_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_rem_uint()
{
    println!("biguint_modular_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "8");

    // modulo >= 2 and dividend == 0 and divisor != 0
    let dividend = U256::zero();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");

    // modulo >= 2 and dividend == multiple of modulo and divisor != 0
    let dividend = U256::from_uint(200_u8);
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_should_panic_modular_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_modular_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // modulo >= 2 and dividend != 0 and divisor == 0    
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == 0
    let _dividend = U256::zero();
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    let _dividend = U256::zero();
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    let _dividend = U256::from_uint(200_u8);
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    let _dividend = U256::from_uint(200_u8);
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo == 0
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::zero();
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);

    // modulo == 1
    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::one();
    let _quotient = _dividend.modular_rem_uint(_divisor, &_modulo);
}

fn biguint_modular_rem_assign_uint()
{
    println!("biguint_modular_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

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

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
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

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign_uint(divisor, &modulo);
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

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}), a_biguint = {}", divisor, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_hould_panic_modular_rem_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_hould_panic_modular_rem_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

    // modulo >= 2 and dividend != 0 and divisor == 0
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == 0
    let _a_biguint = U256::zero();
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    let _a_biguint = U256::zero();
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    let _a_biguint = U256::from_uint(200_u8);
    let _divisor = 0_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    let _a_biguint = U256::from_uint(200_u8);
    let _divisor = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo == 0
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::zero();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);

    // modulo == 1
    let _a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 128_u8;
    let _modulo = U256::one();
    println!("Originally,\n_a_biguint = {}", _a_biguint);
    // It will panic!
    // _a_biguint.modular_rem_assign_uint(_divisor, &_modulo);
    println!("---------------------------");
}

fn biguint_panic_free_modular_rem_uint()
{
    println!("biguint_panic_free_modular_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "8");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == 0 and divisor != 0
    let dividend = U256::zero();
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == multiple of modulo and divisor != 0
    let dividend = U256::from_uint(200_u8);
    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend != 0 and divisor == 0    
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);
    
    // modulo >= 2 and dividend != 0 and divisor == multiple of modulo
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == 0 and divisor == multiple of modulo
    let dividend = U256::zero();
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == multiple of modulo and divisor == 0
    let dividend = U256::from_uint(200_u8);
    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo >= 2 and dividend == multiple of modulo and divisor == multiple of modulo
    let dividend = U256::from_uint(200_u8);
    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), false);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo == 0 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::zero();
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), true);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo == 1 and divisor == 0 and dividend == 0
    let dividend = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::one();
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), true);
    assert_eq!(remainder.is_divided_by_zero(), true);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo == 0 and divisor != 0 and divisor != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::zero();
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), true);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // modulo == 1 and divisor != 0 and divisor != 0
    let dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::one();
    let remainder = dividend.panic_free_modular_rem_uint(divisor, &modulo);
    println!("{} % {} = {} (mod {})", dividend, divisor, remainder, modulo);
    assert_eq!(remainder.to_string(), "0");
    assert_eq!(remainder.is_overflow(), false);
    assert_eq!(remainder.is_underflow(), false);
    assert_eq!(remainder.is_infinity(), false);
    assert_eq!(remainder.is_undefined(), true);
    assert_eq!(remainder.is_divided_by_zero(), false);
    assert_eq!(remainder.is_left_carry(), false);
    assert_eq!(remainder.is_right_carry(), false);

    // collectively
    for modulo in [U256::zero(), U256::one()]
    {
        let op1 = U256::zero();
        let op2 = 0_u8;
        let res = op1.panic_free_modular_rem_uint(op2, &modulo);
        println!("{} % {} = {} (mod {})", op1, op2, res, modulo);
        assert_eq!(res.to_string(), "0");
        assert_eq!(res.is_overflow(), false);
        assert_eq!(res.is_underflow(), false);
        assert_eq!(res.is_divided_by_zero(), true);
        assert_eq!(res.is_infinity(), false);
        assert_eq!(res.is_undefined(), true);
        assert_eq!(res.is_left_carry(), false);
        assert_eq!(res.is_right_carry(), false);
        
        for dividend in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
        {
            let rhs = 0_u8;
            let res = dividend.panic_free_modular_rem_uint(rhs, &modulo);
            println!("{} % {} = {} (mod {})", dividend, rhs, res, modulo);
            assert_eq!(res.to_string(), "0");
            assert_eq!(res.is_overflow(), false);
            assert_eq!(res.is_underflow(), false);
            assert_eq!(res.is_divided_by_zero(), true);
            assert_eq!(res.is_infinity(), false);
            assert_eq!(res.is_undefined(), true);
            assert_eq!(res.is_left_carry(), false);
            assert_eq!(res.is_right_carry(), false);

            for divisor in [3_u8, 50_u8]
            {
                let res = dividend.panic_free_modular_rem_uint(divisor, &modulo);
                println!("{} % {} = {} (mod {})", dividend, divisor, res, modulo);
                assert_eq!(res.to_string(), "0");
                assert_eq!(res.is_overflow(), false);
                assert_eq!(res.is_underflow(), false);
                assert_eq!(res.is_divided_by_zero(), false);
                assert_eq!(res.is_infinity(), false);
                assert_eq!(res.is_undefined(), true);
                assert_eq!(res.is_left_carry(), false);
                assert_eq!(res.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_panic_free_modular_rem_assign_uint()
{
    println!("biguint_panic_free_modular_rem_assign_uint");
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

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "8");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == 0 and divisor != 0
    let mut a_biguint = UU32::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = UU32::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == multiple of modulo and divisor != 0
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 128_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self != 0 and divisor == 0  
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self != 0 and divisor == multiple of modulo
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == 0 and divisor == 0
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == 0 and divisor == multiple of modulo
    let mut a_biguint = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == multiple of modulo and divisor == 0
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 0_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo >= 2 and self == multiple of modulo and divisor == multiple of modulo
    let mut a_biguint = U256::from_uint(200_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let divisor = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0 and self != 0 and divisor != 0
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1 and self != 0 and divisor != 0
    let mut a_biguint = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let divisor = 128_u8;
    let modulo = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0 and divisor == 0 and self == 0
    let mut a_biguint = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::zero();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1 and divisor == 0 and self == 0
    let mut a_biguint = U256::zero();
    let divisor = 0_u8;
    let modulo = U256::one();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_modular_rem_assign_uint(divisor, &modulo);
    println!("After a_biguint.modular_rem_assign_uint({}, {}), a_biguint = {}", divisor, modulo, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), true);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Collectively
    for modulo in [U256::zero(), U256::one()]
    {
        let mut dividend = U256::zero();
        println!("Originally, op1 = {}", dividend);
        assert_eq!(dividend.is_overflow(), false);
        assert_eq!(dividend.is_underflow(), false);
        assert_eq!(dividend.is_infinity(), false);
        assert_eq!(dividend.is_undefined(), false);
        assert_eq!(dividend.is_divided_by_zero(), false);
        assert_eq!(dividend.is_left_carry(), false);
        assert_eq!(dividend.is_right_carry(), false);

        let divisor = 0_u8;
        dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
        println!("After a_biguint.panic_free_modular_rem_assign_uint({}, {}), op1 = {}", divisor, modulo, dividend);
        assert_eq!(dividend.to_string(), "0");
        assert_eq!(dividend.is_overflow(), false);
        assert_eq!(dividend.is_underflow(), false);
        assert_eq!(dividend.is_divided_by_zero(), true);
        assert_eq!(dividend.is_infinity(), false);
        assert_eq!(dividend.is_undefined(), true);
        assert_eq!(dividend.is_left_carry(), false);
        assert_eq!(dividend.is_right_carry(), false);
        
        for op in [U256::from_string("76801874298166903427690031858186486050853753882811946569946433649006").unwrap(), U256::from_uint(50_u8)]
        {
            let mut dividend = op.clone();
            println!("Originally, dividend = {}", dividend);
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_infinity(), false);
            assert_eq!(dividend.is_undefined(), false);
            assert_eq!(dividend.is_divided_by_zero(), false);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            let divisor = 0_u8;
            dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
            println!("After op1.panic_free_modular_rem_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
            assert_eq!(dividend.to_string(), "0");
            assert_eq!(dividend.is_overflow(), false);
            assert_eq!(dividend.is_underflow(), false);
            assert_eq!(dividend.is_divided_by_zero(), true);
            assert_eq!(dividend.is_infinity(), false);
            assert_eq!(dividend.is_undefined(), true);
            assert_eq!(dividend.is_left_carry(), false);
            assert_eq!(dividend.is_right_carry(), false);

            for divisor in [3_u8, 50_u8]
            {
                let mut dividend = op.clone();
                println!("Originally, dividend = {}", dividend);
                assert_eq!(dividend.is_overflow(), false);
                assert_eq!(dividend.is_underflow(), false);
                assert_eq!(dividend.is_infinity(), false);
                assert_eq!(dividend.is_undefined(), false);
                assert_eq!(dividend.is_divided_by_zero(), false);
                assert_eq!(dividend.is_left_carry(), false);
                assert_eq!(dividend.is_right_carry(), false);
    
                dividend.panic_free_modular_rem_assign_uint(divisor, &modulo);
                println!("After dividend.panic_free_modular_rem_assign_uint({}, {}), dividend = {}", divisor, modulo, dividend);
                assert_eq!(dividend.to_string(), "0");
                assert_eq!(dividend.is_overflow(), false);
                assert_eq!(dividend.is_underflow(), false);
                assert_eq!(dividend.is_divided_by_zero(), false);
                assert_eq!(dividend.is_infinity(), false);
                assert_eq!(dividend.is_undefined(), true);
                assert_eq!(dividend.is_left_carry(), false);
                assert_eq!(dividend.is_right_carry(), false);
            }
        }
    }
    println!("---------------------------");
}

fn biguint_next_multiple_uint()
{
    biguint_next_multiple_of_uint();
    biguint_next_multiple_of_assign_uint();
    biguint_panic_free_next_multiple_of_uint();
    biguint_panic_free_next_multiple_of_assign_uint();
    biguint_modular_next_multiple_of_uint();
    biguint_modular_next_multiple_of_assign_uint();
    biguint_panic_free_modular_next_multiple_of_uint();
    biguint_panic_free_modular_next_multiple_of_assign_uint();
    biguint_is_multiple_of_uint();
}

fn biguint_next_multiple_of_uint()
{
    println!("biguint_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 586478_u32;
    let multiple = a_biguint.next_multiple_of_uint(num);
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
    let num = 586478_u32;
    let multiple = a_biguint.next_multiple_of_uint(num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "448670");
    assert_eq!(multiple.is_overflow(), true);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_should_panic_next_multiple_of_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_should_panic_next_multiple_of_uint()
{
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 0_u32;
    let _multiple = _a_biguint.next_multiple_of_uint(_num);
}

fn biguint_next_multiple_of_assign_uint()
{
    println!("biguint_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u64);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 586478_u32;
    a_biguint.next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
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

    let num = 586478_u32;
    a_biguint.next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "448670");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 0_u8;
    // It will panic.
    // _a_biguint.next_multiple_of_assign_uint(_num);
    println!("---------------------------");
}

fn biguint_panic_free_next_multiple_of_uint()
{
    println!("biguint_panic_free_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 586478_u32;
    let multiple = a_biguint.panic_free_next_multiple_of_uint(num);
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
    let num = 586478_u32;
    let multiple = a_biguint.panic_free_next_multiple_of_uint(num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "448670");
    assert_eq!(multiple.is_overflow(), true);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), false);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 0_u8;
    let multiple = a_biguint.panic_free_next_multiple_of_uint(num);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_next_multiple_of_assign_uint()
{
    println!("biguint_panic_free_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 586478_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123697594");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::max();
    let num = 586478_u32;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "448670");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 0_u8;
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    a_biguint.panic_free_next_multiple_of_assign_uint(num);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_modular_next_multiple_of_uint()
{
    println!("biguint_modular_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.modular_next_multiple_of_uint(num, &modulo);
    println!("{}'s next multiple of {} is {}", a_biguint, num, multiple);
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
    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.modular_next_multiple_of_uint(num, &modulo);
    println!("{}'s next multiple of {} is {}", a_biguint, num, multiple);
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
    let _num = 0_u8;
    let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);

    // rhs == multiple of modulo
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);

    // modulo == 0
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 100_u8;
    let _modulo = U256::zero();
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);

    // modulo == 1
    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 100_u8;
    let _modulo = U256::one();
    // It will panic.
    // let multiple = _a_biguint.modular_next_multiple_of_uint(_num, &_modulo);
    println!("---------------------------");
}

fn biguint_modular_next_multiple_of_assign_uint()
{
    println!("biguint_modular_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

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

    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.modular_next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
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

    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == 0
    let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _num = 0_u8;
    let _modulo = _a_biguint.wrapping_add_uint(200_u8);
    // _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);

    // rhs == multiple of modulo
    let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _num = 200_u8;
    let _modulo = U256::from_uint(100_u8);
    // _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);

    // modulo == 0
    let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _num = 100_u8;
    let _modulo = U256::zero();
    // _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);

    // modulo == 1
    let mut _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, _a_biguint = {}", _a_biguint);
    let _num = 100_u8;
    let _modulo = U256::one();
    // _a_biguint.modular_next_multiple_of_assign_uint(_num, &_modulo);
    println!("---------------------------");
}

fn biguint_panic_free_modular_next_multiple_of_uint()
{
    println!("biguint_panic_free_modular_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u8);

    // Normal case 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
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
    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
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
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 0_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // rhs == multiple of modulo
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 200_u8;
    let modulo = U256::from_uint(100_u8);
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // modulo == 0
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 100_u8;
    let modulo = U256::zero();
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // modulo == 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 100_u8;
    let modulo = U256::one();
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // rhs == 0 and modulo == 0
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 0_u8;
    let modulo = U256::zero();
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);

    // rhs == 0 and modulo == 1
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let num = 0_u8;
    let modulo = U256::one();
    let multiple = a_biguint.panic_free_modular_next_multiple_of_uint(num, &modulo);
    println!("The next multiple of {} is {}", a_biguint, multiple);
    assert_eq!(multiple.to_string(), "0");
    assert_eq!(multiple.is_overflow(), false);
    assert_eq!(multiple.is_underflow(), false);
    assert_eq!(multiple.is_infinity(), false);
    assert_eq!(multiple.is_divided_by_zero(), false);
    assert_eq!(multiple.is_undefined(), true);
    assert_eq!(multiple.is_left_carry(), false);
    assert_eq!(multiple.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_panic_free_modular_next_multiple_of_assign_uint()
{
    println!("biguint_panic_free_modular_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u16);

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

    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.modular_next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "123456789012345678901234567890123456800");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // Normal case 2
    let mut a_biguint = UU32::from_str_radix("FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF", 16).unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 100_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
    assert_eq!(a_biguint.is_overflow(), true);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 0_u8;
    let modulo = a_biguint.wrapping_add_uint(200_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == multiple of modulo
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 200_u8;
    let modulo = U256::from_uint(100_u8);
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 100_u8;
    let modulo = U256::zero();
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // modulo == 1
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 100_u8;
    let modulo = U256::one();
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == 0 and modulo == 0
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 0_u8;
    let modulo = U256::zero();
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // rhs == 0 and modulo == 1
    let mut a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let num = 0_u8;
    let modulo = U256::one();
    a_biguint.panic_free_modular_next_multiple_of_assign_uint(num, &modulo);
    println!("After a_biguint.next_multiple_of_assign_uint({}), a_biguint = {}", num, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), true);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_is_multiple_of_uint()
{
    println!("biguint_is_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::number::BigUInt_More;
    use cryptocol::define_utypes_with;
    define_utypes_with!(u32);

    // Normal case 1
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = 100_u8;
    let ans = a_biguint.is_multiple_of_uint(rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, true);

    // Normal case 2
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = 99_u8;
    let ans = a_biguint.is_multiple_of_uint(rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, false);

    // rhs == 0 and self != 0
    let a_biguint = U256::from_str("12345678901234567890123456789012345678900").unwrap();
    let rhs = 0_u8;
    let ans = a_biguint.is_multiple_of_uint(rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, false);

    // rhs == 0 and self == 0
    let a_biguint = U256::zero();
    let rhs = 0_u8;
    let ans = a_biguint.is_multiple_of_uint(rhs);
    println!("Is {} the multiple of {}? -> {}", a_biguint, rhs, ans);
    assert_eq!(ans, true);
    println!("---------------------------");
}
