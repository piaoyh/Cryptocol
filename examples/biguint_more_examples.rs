// Copyright 2023, 2024, 2025 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(unused_imports)]
// #![allow(missing_docs)]
// #![allow(rustdoc::missing_doc_code_examples)]
// #[allow(non_camel_case_types)]
// #[allow(dead_code)]

pub fn main()
{
    biguint_more_quick_start();

    biguint_more_add_uint();
    biguint_more_sub_uint();
    biguint_more_mul_uint();
    biguint_more_div_uint();
    biguint_more_rem_uint();
    biguint_more_next_multiple_uint();
    biguint_more_miscellaneous_arithmatic_operation_uint();
    biguint_more_pow_uint();
    biguint_more_iroot_uint();
    biguint_more_ilog_uint();

    biguint_more_add();
    biguint_more_sub();
    biguint_more_mul();
    biguint_more_div();
    biguint_more_rem();
    biguint_more_next_multiple();
    biguint_more_miscellaneous_arithmatic_operation();
    biguint_more_pow();
    biguint_more_iroot();
    biguint_more_ilog();
    biguint_more_shift();
}

fn biguint_more_quick_start()
{
    println!("biguint_more_quick_start");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);

    let a_biguint = U512::max();
    let one_biguint = U512::one();
    let res = a_biguint.checked_add(&one_biguint);
    match res
    {
        Some(r) => {
                println!("{} + {} = {}, overflow = {}", a_biguint, one_biguint, r, r.is_overflow());
            },
        None => {
                println!("Error: Overflow");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U512::zero();
    let mut res = a_biguint.saturating_sub(&one_biguint);
    println!("{} - {} = {}", a_biguint, one_biguint, res);
    assert_eq!(res.to_string(), "0");

    let a_biguint = U512::from_str("123456789012345678901234567890123456789").unwrap();
    let num = U512::from(586478_u32);
    res = a_biguint.next_multiple_of(&num);
    println!("The next multiple of {} is {}", a_biguint, res);
    assert_eq!(res.to_string(), "123456789012345678901234567890123697594");

    let a_biguint = U512::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = U512::from_string("999998888877777666665555544444333332222211111").unwrap();
    res = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, res);
    assert_eq!(res.to_string(), "44444333332222211111555555555555555555555555555555555555555555555");
    println!("---------------------------");
}

fn biguint_more_add_uint()
{
    biguint_more_checked_add_uint();
    biguint_more_unchecked_add_uint();
    biguint_more_saturating_add_uint();
    biguint_more_saturating_add_assign_uint();
    biguint_more_safe_add_uint();
    biguint_more_safe_add_assign_uint();
}

fn biguint_more_checked_add_uint()
{
    println!("biguint_more_checked_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_add_uint()
{
    println!("biguint_more_unchecked_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_unchecked_add_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_add_uint()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _a_biguint = UU64::max().wrapping_sub_uint(1_u8);
    let _res = _a_biguint.unchecked_add_uint(2_u8);
}

fn biguint_more_saturating_add_uint()
{
    println!("biguint_more_saturating_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_add_assign_uint()
{
    println!("biguint_more_saturating_add_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_add_uint()
{
    println!("biguint_more_safe_add_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_add_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_add_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u128);
    
        let _a_biguint = U512::max().wrapping_sub_uint(1_u8);
        let _res = _a_biguint.safe_add_uint(2_u8);
        let _res = _a_biguint.safe_add_uint(3_u8);
    }
}

fn biguint_more_safe_add_assign_uint()
{
    println!("biguint_more_safe_add_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_add_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_add_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u8);
    
        let mut _a_biguint = U512::max();
        _a_biguint.safe_add_assign_uint(1_u8);
        _a_biguint.safe_add_assign_uint(1_u8);
    }
}


fn biguint_more_sub_uint()
{
    biguint_more_checked_sub_uint();
    biguint_more_unchecked_sub_uint();
    biguint_more_saturating_sub_uint();
    biguint_more_saturating_sub_assign_uint();
    biguint_more_safe_sub_uint();
    biguint_more_safe_sub_assign_uint();
}

fn biguint_more_checked_sub_uint()
{
    println!("biguint_more_checked_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_sub_uint()
{
    println!("biguint_more_unchecked_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let a_biguint = UU64::one();
    let res = a_biguint.unchecked_sub_uint(1_u8);
    println!("{} - 1 = {}", a_biguint, res);
    assert_eq!(res.to_string(), "0");

    #[cfg(test)] // It will panic.
    biguint_more_should_panic_unchecked_sub_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_sub_uint()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _a_biguint = UU64::one();
    let _res = _a_biguint.unchecked_sub_uint(2_u8);
}

fn biguint_more_saturating_sub_uint()
{
    println!("biguint_more_saturating_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_sub_assign_uint()
{
    println!("biguint_more_saturating_sub_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_sub_uint()
{
    println!("biguint_more_wrapping_sub_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_sub_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_sub_uint()
{
    #[cfg(not(debug_assertions))]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u8);

        let _a_biguint = U512::one();
        // It will panic.
        let _res = _a_biguint.safe_sub_uint(2_u8);

        let _a_biguint = U512::one();
        // It will panic.
        let _res = _a_biguint.safe_sub_uint(3_u8);
    }
}

fn biguint_more_safe_sub_assign_uint()
{
    println!("biguint_more_safe_sub_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_sub_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_sub_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u16);

        let mut _a_biguint = UU64::one();
        // It will panic.
        _a_biguint.safe_sub_assign_uint(2_u8);

        let mut _a_biguint = UU64::one();
        // It will panic.
        _a_biguint.safe_sub_assign_uint(3_u8);
    }
}

fn biguint_more_mul_uint()
{
    biguint_more_checked_mul_uint();
    biguint_more_unchecked_mul_uint();
    biguint_more_saturating_mul_uint();
    biguint_more_saturating_mul_assign_uint();
    biguint_more_safe_mul_uint();
    biguint_more_safe_mul_assign_uint();
}

fn biguint_more_checked_mul_uint()
{
    println!("biguint_more_checked_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_mul_uint()
{
    println!("biguint_more_unchecked_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_unchecked_mul_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_mul_uint()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
    let _res = _a_biguint.unchecked_mul_uint(248_u8);
}

fn biguint_more_saturating_mul_uint()
{
    println!("biguint_more_saturating_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_mul_assign_uint()
{
    println!("biguint_more_saturating_mul_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_mul_uint()
{
    println!("biguint_more_safe_mul_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_mul_3uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_mul_3uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u16);
    
        let _b_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_uint = 248_u16;
        let _res = _b_biguint.safe_mul_uint(_b_uint);
    }
}

fn biguint_more_safe_mul_assign_uint()
{
    println!("biguint_more_safe_mul_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_mul_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_mul_assign_uint()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_div_uint()
{
    biguint_more_checked_div_uint();
    biguint_more_unchecked_div_uint();
    biguint_more_saturating_div_uint();
    biguint_more_saturating_div_assign_uint();
}

fn biguint_more_checked_div_uint()
{
    println!("biguint_more_checked_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_div_uint()
{
    println!("biguint_more_unchecked_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_unchecked_div_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_div_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _quotient = _dividend.unchecked_div_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _quotient = _dividend.unchecked_div_uint(_divisor);
}

fn biguint_more_saturating_div_uint()
{
    println!("biguint_more_saturating_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_saturating_div_uint();
    println!("---------------------------")
}

#[test]
#[should_panic]
fn biguint_more_should_panic_saturating_div_uint()
{
    println!("biguint_more_saturating_div_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _quotient = _dividend.saturating_div_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _quotient = _dividend.saturating_div_uint(_divisor);
    println!("---------------------------")
}

fn biguint_more_saturating_div_assign_uint()
{
    println!("biguint_more_saturating_div_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_saturating_div_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_saturating_div_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_rem_uint()
{
    biguint_more_checked_rem_uint();
    biguint_more_unchecked_rem_uint();
    biguint_more_saturating_rem_uint();
    biguint_more_saturating_rem_assign_uint();
}

fn biguint_more_checked_rem_uint()
{
    println!("biguint_more_checked_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_rem_uint()
{
    println!("biguint_more_unchecked_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_unchecked_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);

    let _dividend = U256::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _remainder = _dividend.unchecked_rem_uint(_divisor);

    let _dividend = U256::zero();
    let _divisor = 0_u8;
    let _remainder = _dividend.unchecked_rem_uint(_divisor);
}

fn biguint_more_saturating_rem_uint()
{
    println!("biguint_more_saturating_rem_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_saturating_rem_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_saturating_rem_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);
    
    let _dividend = UU32::from_str("123456789015758942546236989636279846864825945392").unwrap();
    let _divisor = 0_u8;
    let _remainder = _dividend.saturating_rem_uint(_divisor);

    let _dividend = UU32::zero();
    let _divisor = 0_u8;
    let _remainder = _dividend.saturating_rem_uint(_divisor);
}

fn biguint_more_saturating_rem_assign_uint()
{
    println!("biguint_more_saturating_rem_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_saturating_rem_assign_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_saturating_rem_assign_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_next_multiple_uint()
{
    biguint_more_next_multiple_of_uint();
    biguint_more_next_multiple_of_assign_uint();
    biguint_more_is_multiple_of_uint();
}

fn biguint_more_next_multiple_of_uint()
{
    println!("biguint_more_next_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_next_multiple_of_uint();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_next_multiple_of_uint()
{
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let _num = 0_u32;
    let _multiple = _a_biguint.next_multiple_of_uint(_num);
}

fn biguint_more_next_multiple_of_assign_uint()
{
    println!("biguint_more_next_multiple_of_assign_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_is_multiple_of_uint()
{
    println!("biguint_more_is_multiple_of_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_miscellaneous_arithmatic_operation_uint()
{
    biguint_more_midpoint_uint();
    biguint_more_midpoint_assign_uint();
}

fn biguint_more_midpoint_uint()
{
    println!("biguint_more_midpoint_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);

    // normal case
    let a_biguint = U256::from_string("8888866666444442222233333444445555566666777778888899999").unwrap();
    let b_biguint = 77777666665555544444333332222211111_u128;
    let c_biguint = a_biguint.midpoint_uint(b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "4444433333222221111155555555555555555555555555555555555");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is even number and rhs is even number
    let a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    let b_biguint = 66666555554444433333222221111100000_u128;
    let c_biguint = a_biguint.midpoint_uint(b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is even number and rhs is odd number
    let a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    let b_biguint = 66666555554444433333222221111100001_u128;
    let c_biguint = a_biguint.midpoint_uint(b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is even number
    let a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    let b_biguint = 66666555554444433333222221111100000_u128;
    let c_biguint = a_biguint.midpoint_uint(b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is odd number
    let a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    let b_biguint = 66666555554444433333222221111100001_u128;
    let c_biguint = a_biguint.midpoint_uint(b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_more_midpoint_assign_uint()
{
    println!("biguint_more_midpoint_uint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);

    // normal case
    let mut a_biguint = U256::from_string("8888866666444442222233333444445555566666777778888899999").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 77777666665555544444333332222211111_u128;
    a_biguint.midpoint_assign_uint(b_biguint);
    println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "4444433333222221111155555555555555555555555555555555555");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is even number and rhs is even number
    let mut a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 66666555554444433333222221111100000_u128;
    a_biguint.midpoint_assign_uint(b_biguint);
    println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is even number and rhs is odd number
    let mut a_biguint = U256::from_string("66666444442222244444555556666677777888889999900000").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 66666555554444433333222221111100001_u128;
    a_biguint.midpoint_assign_uint(b_biguint);
    println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "33333222221111155555555555555555555555555555500000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is even number
    let mut a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 66666555554444433333222221111100000_u128;
    a_biguint.midpoint_assign_uint(b_biguint);
    println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is odd number
    let mut a_biguint = U256::from_string("2222288888666664444422222333334444455555666667777788888").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = 66666555554444433333222221111100001_u128;
    a_biguint.midpoint_assign_uint(b_biguint);
    println!("After a_biguint.midpoint_assign_uint(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "1111144444333332222244444444444444444444444444444444444");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_more_pow_uint()
{
    biguint_more_checked_pow_uint();
    biguint_more_unchecked_pow_uint();
    biguint_more_saturating_pow_uint();
    biguint_more_saturating_pow_assign_uint();
}

fn biguint_more_checked_pow_uint()
{
    println!("biguint_more_checked_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u8;
    let res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "1000000000000000000000000000000");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 100_u8;
    let res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
        None => {
                println!("Overflow");
                assert_eq!(res, None);
            },
    }

    let a_biguint = UU32::zero();
    let exp = 30_u8;
    let res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "0");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "1");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::zero();
    let exp = 0_u8;
    let res = a_biguint.checked_pow_uint(exp);
    match res
    {
        Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
        None => {
                println!("Undefined");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_pow_uint()
{
    println!("biguint_more_unchecked_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u128);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 30_u8;
    let res = a_biguint.unchecked_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = UU32::from_string("1000000000000000000000000000000");
    let _exp = 30_u8;
    // It will panic.
    // println!("{} ** {} = {}", _a_biguint, 100_u32, _a_biguint.unchecked_pow_uint(_exp);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let res = a_biguint.unchecked_pow_uint(exp);
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
    let res = a_biguint.unchecked_pow_uint(exp);
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
    // let res = _a_biguint.unchecked_pow_uint(_exp);
    println!("---------------------------");
}

fn biguint_more_saturating_pow_uint()
{
    println!("biguint_more_saturating_pow_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_uint(10_u8);
    let exp = 30_u8;
    let res = a_biguint.saturating_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_uint(10_u8);
    let exp = 100_u8;
    let res = a_biguint.saturating_pow_uint(exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res, UU32::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = 0_u8;
    let res = a_biguint.saturating_pow_uint(exp);
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
    let res = a_biguint.saturating_pow_uint(exp);
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
    // let res = _a_biguint.saturating_pow_uint(_exp);
    println!("---------------------------");
}

fn biguint_more_saturating_pow_assign_uint()
{
    println!("biguint_more_saturating_pow_assign_uint");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);
    
    let mut a_biguint = UU32::from_uint(10_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1000000000000000000000000000000");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_uint(1000000000000000000000000000000_u128);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 30_u8;
    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint, UU32::max());
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut a_biguint = UU32::from_uint(100_u8);
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let exp = 0_u8;
    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "1");
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

    let exp = 30_u8;
    a_biguint.saturating_pow_assign_uint(exp);
    println!("After a_biguint.saturating_pow_assign_uint({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let mut _a_biguint = UU32::zero();
    let _exp = 0_u8;
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    // _a_biguint.saturating_pow_assign_uint(_exp);
    println!("---------------------------");
}


fn biguint_more_iroot_uint()
{
    biguint_more_checked_iroot_uint();
    biguint_more_unchecked_iroot_uint();
}

fn biguint_more_checked_iroot_uint()
{
    println!("biguint_more_checked_iroot_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 8_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The third root of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "100000000");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 65_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The square root of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "9");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 212_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The square root of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "2");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 213_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The square root of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "1");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = u128::MAX;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The square root of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "1");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::zero();
    let exp = 6_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 0_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::one();
    let exp = 0_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let exp = 0_u8;
    let res = a_biguint.checked_iroot_uint(exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_iroot_uint()
{
    println!("biguint_more_unchecked_iroot_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = 8_u8;
    let res = a_biguint.unchecked_iroot_uint(exp);
    println!("The third root of {} is {}.", a_biguint, res);
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
    let res = a_biguint.unchecked_iroot_uint(exp);
    println!("The square root of {} is {}.", a_biguint, res);
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
    let res = a_biguint.unchecked_iroot_uint(exp);
    println!("The square root of {} is {}.", a_biguint, res);
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
    let res = a_biguint.unchecked_iroot_uint(exp);
    println!("The square root of {} is {}.", a_biguint, res);
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
    let res = a_biguint.unchecked_iroot_uint(exp);
    println!("The square root of {} is {}.", a_biguint, res);
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
    let res = a_biguint.unchecked_iroot_uint(exp);
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
    // let res = _a_biguint.unchecked_iroot_uint(_exp);

    let _a_biguint = U256::one();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_iroot_uint(_exp);

    let _a_biguint = U256::zero();
    let _exp = 0_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_iroot_uint(_exp);
    println!("---------------------------");
}


fn biguint_more_ilog_uint()
{   
    biguint_more_checked_ilog_uint();
    biguint_more_unchecked_ilog_uint();
}

fn biguint_more_checked_ilog_uint()
{
    println!("biguint_more_checked_ilog_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    let res = a_biguint.checked_ilog_uint(base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "2");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 10_u8;
    let res = a_biguint.checked_ilog_uint(base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "64");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::one();
    let base = 6_u8;
    let res = a_biguint.checked_ilog_uint(base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 0_u8;
    let res = a_biguint.checked_ilog_uint(base);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = 6_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = 0_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = 1_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::one();
    let base = 0_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::one();
    let base = 1_u8;
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_ilog_uint()
{
    println!("biguint_more_unchecked_ilog_uint");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = 1_0000_0000_0000_0000_0000_0000_0000_0000_u128;
    let res = a_biguint.unchecked_ilog_uint(base);
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
    let res = a_biguint.unchecked_ilog_uint(base);
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
    let res = a_biguint.unchecked_ilog_uint(base);
    println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let _a_biguint = U256::one();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 6_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::zero();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::one();
    let _base = 0_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);

    let _a_biguint = U256::one();
    let _base = 1_u8;
    // It will panic.
    // let res = _a_biguint.unchecked_ilog_uint(_base);
    println!("---------------------------");
}

fn biguint_more_add()
{
    biguint_more_checked_add();
    biguint_more_unchecked_add();
    biguint_more_saturating_add();
    biguint_more_saturating_add_assign();
    biguint_more_safe_add();
    biguint_more_safe_add_assign();
}

fn biguint_more_checked_add()
{
    println!("biguint_more_checked_add");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_add()
{
    println!("biguint_more_unchecked_add()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_add()
{
    println!("biguint_more_saturating_add");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_add_assign()
{
    println!("biguint_more_saturating_add_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_add()
{
    println!("biguint_more_safe_add");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_add();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_add()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u128);

        let _b_biguint = U512::max();
        let _one_biguint = U512::one();
        let _res = _b_biguint.safe_add(&_one_biguint);
    }
}

fn biguint_more_safe_add_assign()
{
    println!("biguint_more_safe_add_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_add_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_add_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u8);
    
        let mut _a_biguint = U512::max();
        let _one_biguint = U512::one();
        _a_biguint.safe_add_assign(&_one_biguint);
        _a_biguint.safe_add_assign(&_one_biguint);
    }
}


fn biguint_more_sub()
{
    biguint_more_checked_sub();
    biguint_more_unchecked_sub();
    biguint_more_saturating_sub();
    biguint_more_saturating_sub_assign();
    biguint_more_safe_sub();
    biguint_more_safe_sub_assign();
}

fn biguint_more_checked_sub()
{
    println!("biguint_more_checked_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_sub()
{
    println!("biguint_more_unchecked_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_sub()
{
    println!("biguint_more_saturating_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_sub_assign()
{
    println!("biguint_more_saturating_sub_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_sub()
{
    println!("biguint_more_safe_sub");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_sub();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_sub()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u128);
    
        let _b_biguint = U512::zero();
        let _one = U512::one();
        let _res = _b_biguint.safe_sub(&_one);
    }
}

fn biguint_more_safe_sub_assign()
{
    println!("biguint_more_safe_sub_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_sub_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_sub_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_mul()
{
    biguint_more_checked_mul();
    biguint_more_unchecked_mul();
    biguint_more_saturating_mul();
    biguint_more_saturating_mul_assign();
    biguint_more_safe_mul();
    biguint_more_safe_mul_assign();
}

fn biguint_more_checked_mul()
{
    println!("biguint_more_checked_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_mul()
{
    println!("biguint_more_unchecked_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_mul()
{
    println!("biguint_more_saturating_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_mul_assign()
{
    println!("biguint_more_saturating_mul_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_safe_mul()
{
    println!("biguint_more_safe_mul");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_mul();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_mul()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u16);
        
        let _a_biguint = U256::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_biguint = U256::from_uint(248_u8);
        let _res = _a_biguint.safe_mul(&_b_biguint);
    }
}

fn biguint_more_safe_mul_assign()
{
    println!("biguint_more_safe_mul_assign");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    biguint_more_should_panic_safe_mul_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_safe_mul_assign()
{
    #[cfg(debug_assertions)]
    {
        use cryptocol::number::BigUInt_More;
        use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
        define_utypes_with!(u32);

        let mut _a_biguint = UU32::from_string("876801874298166903427690031858186486050853753882811946569946433649006084094").unwrap();
        let _b_biguint = U256::from_uint(248_u8);
        _a_biguint.safe_mul_assign(&_b_biguint);
    }
}


fn biguint_more_div()
{
    biguint_more_checked_div();
    biguint_more_unchecked_div();
    biguint_more_saturating_div();
    biguint_more_saturating_div_assign();
}

fn biguint_more_checked_div()
{
    println!("biguint_more_checked_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_div()
{
    println!("biguint_more_unchecked_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_div()
{
    println!("biguint_more_saturating_div");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_div_assign()
{
    println!("biguint_more_saturating_div_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_rem()
{
    biguint_more_checked_rem();
    biguint_more_unchecked_rem();
    biguint_more_saturating_rem();
    biguint_more_saturating_rem_assign();
}

fn biguint_more_checked_rem()
{
    println!("biguint_more_checked_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_unchecked_rem()
{
    println!("biguint_more_unchecked_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_rem()
{
    println!("biguint_more_saturating_rem");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_saturating_rem_assign()
{
    println!("biguint_more_saturating_rem_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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


fn biguint_more_next_multiple()
{
    biguint_more_next_multiple_of();
    biguint_more_next_multiple_of_assign();
    biguint_more_is_multiple_of();
}

fn biguint_more_next_multiple_of()
{
    println!("biguint_more_next_multiple_of");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_next_multiple_of_assign()
{
    println!("biguint_more_next_multiple_of_assign");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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

fn biguint_more_is_multiple_of()
{
    println!("biguint_more_is_multiple_of()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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



fn biguint_more_miscellaneous_arithmatic_operation()
{
    biguint_more_midpoint();
    biguint_more_midpoint_assign();
}

fn biguint_more_midpoint()
{
    println!("biguint_more_midpoint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);

    // normal case
    let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = U256::from_string("999998888877777666665555544444333332222211111").unwrap();
    let c_biguint = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "44444333332222211111555555555555555555555555555555555555555555555");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is even number and rhs is even number
    let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    let c_biguint = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is even number and rhs is odd number
    let a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    let c_biguint = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is even number
    let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    let c_biguint = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is odd number
    let a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    let c_biguint = a_biguint.midpoint(&b_biguint);
    println!("The middle point of {} and {} is {}.", a_biguint, b_biguint, c_biguint);
    assert_eq!(c_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    assert_eq!(c_biguint.is_overflow(), false);
    assert_eq!(c_biguint.is_underflow(), false);
    assert_eq!(c_biguint.is_infinity(), false);
    assert_eq!(c_biguint.is_undefined(), false);
    assert_eq!(c_biguint.is_divided_by_zero(), false);
    assert_eq!(c_biguint.is_left_carry(), false);
    assert_eq!(c_biguint.is_right_carry(), false);
    println!("---------------------------");
}

fn biguint_more_midpoint_assign()
{
    println!("biguint_more_midpoint()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);

    // normal case
    let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("999998888877777666665555544444333332222211111").unwrap();
    a_biguint.midpoint_assign(&b_biguint);
    println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "44444333332222211111555555555555555555555555555555555555555555555");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is even number and rhs is even number
    let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("888887777766666555554444433333222221111100000").unwrap();
    a_biguint.midpoint_assign(&b_biguint);
    println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is even number and rhs is odd number
    let mut a_biguint = U256::from_string("888886666644444222221111122222333334444455555666667777788888").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("888887777766666555554444433333222221111100001").unwrap();
    a_biguint.midpoint_assign(&b_biguint);
    println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "444443333322222555554444444444444444444444444444444444444444");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is even number
    let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("99999888887777766666555554444433333222221111100000").unwrap();
    a_biguint.midpoint_assign(&b_biguint);
    println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "44444333332222261110999999999999999999999999999999999999999999999");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    // case that self is odd number and rhs is odd number
    let mut a_biguint = U256::from_string("88888666664444422222111112222233333444445555566666777778888899999").unwrap();
    println!("Originally, a_biguint = {}", a_biguint);
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    let b_biguint = U256::from_string("22222444446666688888999998888877777666665555544444333332222211111").unwrap();
    a_biguint.midpoint_assign(&b_biguint);
    println!("After a_biguint.midpoint_assign(), a_biguint = {}.", a_biguint);
    assert_eq!(a_biguint.to_string(), "55555555555555555555555555555555555555555555555555555555555555555");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);
    println!("---------------------------");
}


fn  biguint_more_pow()
{
    biguint_more_checked_pow();
    biguint_more_unchecked_pow();
    biguint_more_saturating_pow();
    biguint_more_saturating_pow_assign();
}

fn biguint_more_checked_pow()
{
    println!("biguint_more_checked_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);

    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let res = a_biguint.checked_pow(&exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "1000000000000000000000000000000");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(100_u8);
    let res = a_biguint.checked_pow(&exp);
    match res
    {
        Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
        None => {
                println!("Overflow");
                assert_eq!(res, None);
            },
    }

    let a_biguint = UU32::zero();
    let exp = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.checked_pow(&exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "0");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = UU32::zero();
    let res = a_biguint.checked_pow(&exp);
    match res
    {
        Some(raised) => {
                println!("{} ** {} = {}", a_biguint, exp, raised);
                assert_eq!(raised.to_string(), "1");
                assert_eq!(raised.is_overflow(), false);
                assert_eq!(raised.is_underflow(), false);
                assert_eq!(raised.is_infinity(), false);
                assert_eq!(raised.is_divided_by_zero(), false);
                assert_eq!(raised.is_undefined(), false);
                assert_eq!(raised.is_left_carry(), false);
                assert_eq!(raised.is_right_carry(), false);
            },
        None => { println!("Overflow"); }
    }

    let a_biguint = UU32::zero();
    let exp = UU32::zero();
    let res = a_biguint.checked_pow(&exp);
    match res
    {
        Some(raised) => { println!("{} ** {} = {}", a_biguint, exp, raised); },
        None => {
                println!("Undefined");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_pow()
{
    println!("biguint_more_unchecked_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    // normal exponentiation
    let a_biguint = U256::from_uint(10_u8);
    let exp = U256::from_uint(30_u8);
    let res = a_biguint.unchecked_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "1000000000000000000000000000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = U256::zero();
    let res = a_biguint.unchecked_pow(&exp);
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
    let a_biguint = U256::zero();
    let exp = U256::from_str("123456789012345678901234567890123456789").unwrap();
    let res = a_biguint.unchecked_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res.to_string(), "0");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // Panic examples
    let _a_biguint = U256::from_uint(10_u8);
    let _exp = U256::from_uint(100_u8);
    // It will panic.
    // let res = _a_biguint.unchecked_pow(&_exp);

    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.unchecked_pow(&_exp);
    println!("---------------------------");
}

fn biguint_more_saturating_pow()
{
    println!("biguint_more_saturating_pow()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    // normal exponentiation
    let a_biguint = UU32::from_uint(10_u8);
    let exp = UU32::from_uint(30_u8);
    let res = a_biguint.saturating_pow(&exp);
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
    let res = a_biguint.saturating_pow(&exp);
    println!("{} ** {} = {}", a_biguint, exp, res);
    assert_eq!(res, UU32::max());
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    // 123456789012345678901234567890123456789 ** 0
    let a_biguint = UU32::from_str("123456789012345678901234567890123456789").unwrap();
    let exp = UU32::zero();
    let res = a_biguint.saturating_pow(&exp);
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
    let res = a_biguint.saturating_pow(&exp);
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
    // let res = _a_biguint.saturating_pow(&_exp);
    println!("---------------------------");
}

fn biguint_more_saturating_pow_assign()
{
    println!("biguint_more_saturating_pow_assign()");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
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
    a_biguint.saturating_pow_assign(&exp);
    println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.saturating_pow_assign(&exp);
    println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint, U256::max());
    assert_eq!(a_biguint.is_overflow(), false);
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
    a_biguint.saturating_pow_assign(&exp);
    println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
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
    a_biguint.saturating_pow_assign(&exp);
    println!("After a_biguint.saturating_pow_assign({}), a_biguint = {}", exp, a_biguint);
    assert_eq!(a_biguint.to_string(), "0");
    assert_eq!(a_biguint.is_overflow(), false);
    assert_eq!(a_biguint.is_underflow(), false);
    assert_eq!(a_biguint.is_infinity(), false);
    assert_eq!(a_biguint.is_undefined(), false);
    assert_eq!(a_biguint.is_divided_by_zero(), false);
    assert_eq!(a_biguint.is_left_carry(), false);
    assert_eq!(a_biguint.is_right_carry(), false);

    #[cfg(test)] // It will panic.
    biguint_more_should_panic_saturating_pow_assign();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_saturating_pow_assign()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u128);

    let mut _a_biguint = U256::zero();
    let _exp = U256::zero();
    println!("Originally, a_biguint = {}", _a_biguint);
    // It will panic.
    _a_biguint.saturating_pow_assign(&_exp);
}


fn biguint_more_iroot()
{
    biguint_more_checked_iroot();
    biguint_more_unchecked_iroot();
}

fn biguint_more_checked_iroot()
{
    println!("biguint_more_checked_iroot");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u128);

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(8_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "100000000");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(65_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "9");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(212_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "2");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(213_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "1");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(u128::MAX).wrapping_add_uint(1_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "1");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); }
    }

    let a_biguint = U256::zero();
    let exp = U256::from_uint(6_u8);
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => {
                println!("The {}-th root of {} is {}.", exp, a_biguint, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::zero();
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::one();
    let exp = U256::zero();
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let exp = U256::zero();
    let res = a_biguint.checked_iroot(&exp);
    match res
    {
        Some(r) => { println!("The {}-th root of {} is {}.", exp, a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_iroot()
{
    println!("biguint_more_unchecked_iroot");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let exp = U256::from_uint(8_u8);
    let res = a_biguint.unchecked_iroot(&exp);
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
    let res = a_biguint.unchecked_iroot(&exp);
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
    let res = a_biguint.unchecked_iroot(&exp);
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
    let res = a_biguint.unchecked_iroot(&exp);
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
    let res = a_biguint.unchecked_iroot(&exp);
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
    let res = a_biguint.unchecked_iroot(&exp);
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
    // let res = _a_biguint.unchecked_iroot(&_exp);

    let _a_biguint = U256::one();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.unchecked_iroot(&_exp);

    let _a_biguint = U256::zero();
    let _exp = U256::zero();
    // It will panic.
    // let res = _a_biguint.unchecked_iroot(&_exp);
    println!("---------------------------");
}


fn biguint_more_ilog()
{
    biguint_more_checked_ilog();
    biguint_more_unchecked_ilog();
    biguint_more_checked_ilog2();
    biguint_more_unchecked_ilog2();
    biguint_more_checked_ilog10();
    biguint_more_unchecked_ilog10();
}



fn biguint_more_checked_ilog()
{
    println!("biguint_more_checked_ilog");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u8);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "2");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(10_u8);
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "64");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::one();
    let base = U256::from_uint(6_u8);
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => {
                println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::zero();
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::one();
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = U256::from_uint(6_u8);
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = U256::zero();
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }

    let a_biguint = U256::zero();
    let base = U256::one();
    let res = a_biguint.checked_ilog(&base);
    match res
    {
        Some(r) => { println!("The logarithm of {} with respect to {} is {}.", a_biguint, base, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_ilog()
{
    println!("biguint_more_unchecked_ilog");
    use std::str::FromStr;
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);
    
    let a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let base = U256::from_uint(1_0000_0000_0000_0000_0000_0000_0000_0000_u128);
    let res = a_biguint.unchecked_ilog(&base);
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
    let res = a_biguint.unchecked_ilog(&base);
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
    let res = a_biguint.unchecked_ilog(&base);
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
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::from_str("1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000").unwrap();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::from_uint(6_u8);
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::zero();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::one();
    let _base = U256::zero();
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);

    let _a_biguint = U256::one();
    let _base = U256::one();
    // It will panic.
    // let res = _a_biguint.unchecked_ilog(&_base);
    println!("---------------------------");
}

fn biguint_more_checked_ilog2()
{
    println!("biguint_more_checked_ilog2");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);
    
    let a_biguint = U256::from_uint(64_u8);
    let res = a_biguint.checked_ilog2();
    match res
    {
        Some(r) => {
                println!("The base 2 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "6");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_uint(70_u8);
    let res = a_biguint.checked_ilog2();
    match res
    {
        Some(r) => {
                println!("The base 2 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "6");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::one();
    let res = a_biguint.checked_ilog2();
    match res
    {
        Some(r) => {
                println!("The base 2 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::zero();
    let res = a_biguint.checked_ilog_uint(1_u8);
    match res
    {
        Some(r) => { println!("The base 2 logarithm of {}is {}.", a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_ilog2()
{
    println!("biguint_more_unchecked_ilog2");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let a_biguint = U256::from_uint(64_u8);
    let res = a_biguint.unchecked_ilog2();
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
    let res = a_biguint.unchecked_ilog2();
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
    let res = a_biguint.unchecked_ilog2();
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
    // let res = _a_biguint.unchecked_ilog2();
    println!("---------------------------");
}

fn biguint_more_checked_ilog10()
{
    println!("biguint_more_checked_ilog10");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);
    
    let a_biguint = U256::from_uint(10000_u32);
    let res = a_biguint.checked_ilog10();
    match res
    {
        Some(r) => {
                println!("The base 10 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "4");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::from_uint(12345_u32);
    let res = a_biguint.checked_ilog10();
    match res
    {
        Some(r) => {
                println!("The base 10 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "4");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::one();
    let res = a_biguint.checked_ilog10();
    match res
    {
        Some(r) => {
                println!("The base 10 logarithm of {} is {}.", a_biguint, r);
                assert_eq!(r.to_string(), "0");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => { println!("Error"); },
    }

    let a_biguint = U256::zero();
    let res = a_biguint.checked_ilog10();
    match res
    {
        Some(r) => { println!("The 10 logarithm of {} is {}.", a_biguint, r); },
        None => {
                println!("Error");
                assert_eq!(res, None);
            },
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_ilog10()
{
    println!("biguint_more_unchecked_ilog10");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    let a_biguint = U256::from_uint(10000_u32);
    let res = a_biguint.unchecked_ilog10();
    println!("The base 10 logarithm of {} is {}.", a_biguint, res);
    assert_eq!(res.to_string(), "4");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_uint(12345_u32);
    let res = a_biguint.unchecked_ilog10();
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
    let res = a_biguint.unchecked_ilog10();
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
    // let res = _a_biguint.unchecked_ilog10();
    println!("---------------------------");
}


fn biguint_more_shift()
{
    biguint_more_checked_shift_left();
    biguint_more_unchecked_shift_left();
    biguint_more_checked_shift_right();
    biguint_more_unchecked_shift_right();
}

fn biguint_more_checked_shift_left()
{
    println!("biguint_more_checked_shift_left()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.checked_shift_left(n);
    match res
    {
        Some(r) => {
                println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), true);
                assert_eq!(r.is_right_carry(), false);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.checked_shift_left(n);
    match res
    {
        Some(r) => {
                println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.checked_shift_left(n);
    match res
    {
        Some(r) => {
                println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), true);
                assert_eq!(r.is_right_carry(), false);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 256_u16;
    let res = a_biguint.checked_shift_left(n);
    match res
    {
        Some(r) => {
                println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
            },
        None => {
                println!("All bits are gone!");
                assert_eq!(res, None);
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 512_u16;
    let res = a_biguint.checked_shift_left(n);
    match res
    {
        Some(r) => {
                println!("{} << {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
            },
        None => {
                println!("All bits are gone!");
                assert_eq!(res, None);
            }
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_shift_left()
{
    println!("biguint_more_unchecked_shift_left()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.unchecked_shift_left(n);
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101111_11111000_00000111_10000000_01111110_01100001_10011101_01010010_10101000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.unchecked_shift_left(n);
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01010000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.unchecked_shift_left(n);
    println!("{} << {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), true);
    assert_eq!(res.is_right_carry(), false);

    #[cfg(test)] // It will panic!
    biguint_more_should_panic_unchecked_shift_left();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_shift_left()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u64);

    let _a_biguint = U256::from_str_radix("00001111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let _n = 256_u16;
    // It will panic!
    let _res = _a_biguint.unchecked_shift_left(_n);

    let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let _n = 512_u16;
    // It will panic!
    let _res = _a_biguint.unchecked_shift_left(_n);
}

fn biguint_more_checked_shift_right()
{
    println!("biguint_more_checked_shift_right()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u16);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.checked_shift_right(n);
    match res
    {
        Some(r) => {
                println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), true);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11110000", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.checked_shift_right(n);
    match res
    {
        Some(r) => {
                println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00110000_00001111");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), false);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.checked_shift_right(n);
    match res
    {
        Some(r) => {
                println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
                assert_eq!(r.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
                assert_eq!(r.is_overflow(), false);
                assert_eq!(r.is_underflow(), false);
                assert_eq!(r.is_infinity(), false);
                assert_eq!(r.is_undefined(), false);
                assert_eq!(r.is_divided_by_zero(), false);
                assert_eq!(r.is_left_carry(), false);
                assert_eq!(r.is_right_carry(), true);
            },
        None => {
                println!("All bits are gone!");
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 256_u16;
    let res = a_biguint.checked_shift_right(n);
    match res
    {
        Some(r) => {
                println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
            },
        None => {
                println!("All bits are gone!");
                assert_eq!(res, None);
            }
    }

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 512_u16;
    let res = a_biguint.checked_shift_right(n);
    match res
    {
        Some(r) => {
                println!("{} >> {} = {}", r.to_string_with_radix_and_stride(2, 8).unwrap(), n, r.to_string_with_radix_and_stride(2, 8).unwrap());
            },
        None => {
                println!("All bits are gone!");
                assert_eq!(res, None);
            }
    }
    println!("---------------------------");
}

fn biguint_more_unchecked_shift_right()
{
    println!("biguint_more_unchecked_shift_right()");
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 3_u8;
    let res = a_biguint.unchecked_shift_right(n);
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01110101_01001010_10111111_11100000_00011110_00000001_11111001_10000110_01100000_00011111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_11110000", 2).unwrap();
    let n = 4_u8;
    let res = a_biguint.unchecked_shift_right(n);
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "1111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10100101_01011111_11110000_00001111_00000000_11111100_11000011_00111010_10101111");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), false);

    let a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_00000000_11111111", 2).unwrap();
    let n = 128_u8;
    let res = a_biguint.unchecked_shift_right(n);
    println!("{} >> {} = {}", a_biguint.to_string_with_radix_and_stride(2, 8).unwrap(), n, res.to_string_with_radix_and_stride(2, 8).unwrap());
    assert_eq!(res.to_string_with_radix_and_stride(2, 8).unwrap(), "11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101");
    assert_eq!(res.is_overflow(), false);
    assert_eq!(res.is_underflow(), false);
    assert_eq!(res.is_infinity(), false);
    assert_eq!(res.is_undefined(), false);
    assert_eq!(res.is_divided_by_zero(), false);
    assert_eq!(res.is_left_carry(), false);
    assert_eq!(res.is_right_carry(), true);

    #[cfg(test)]
    biguint_more_should_panic_unchecked_shift_right();
    println!("---------------------------");
}

#[test]
#[should_panic]
fn biguint_more_should_panic_unchecked_shift_right()
{
    use cryptocol::define_utypes_with;
    use cryptocol::number::BigUInt_More;
    define_utypes_with!(u32);

    let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let _n = 256_u16;
    // It will panic!
    let _res = _a_biguint.unchecked_shift_right(_n);

    let _a_biguint = U256::from_str_radix("11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101_11111111_00000000_11110000_00001111_11001100_00110011_10101010_01010101", 2).unwrap();
    let _n = 512_u16;
    // It will panic!
    let _res = _a_biguint.unchecked_shift_right(_n);
}